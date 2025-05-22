use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuVoucher {
    pub id: i32,
    pub menu_name: String,
    pub code: String,
    pub discount_percent: f64,
    pub start_date: NaiveDateTime,
    pub expiry_date: NaiveDateTime,
    pub active: bool,
    pub created_at: NaiveDateTime,
}

impl MenuVoucher {
    pub async fn find_by_menu_name(pool: &PgPool, menu_name: &str) -> Result<Vec<MenuVoucher>, String> {
        sqlx::query_as!(
            MenuVoucher,
            r#"
            SELECT * FROM menu_vouchers 
            WHERE menu_name = $1 
            AND active = true 
            AND CURRENT_TIMESTAMP BETWEEN start_date AND expiry_date
            "#,
            menu_name
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch vouchers: {}", e))
    }

    pub async fn apply_voucher(
        pool: &PgPool,
        code: &str,
        menu_name: &str
    ) -> Result<MenuVoucher, String> {
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        let voucher = sqlx::query_as!(
            MenuVoucher,
            r#"
            DELETE FROM menu_vouchers 
            WHERE code = $1 
            AND menu_name = $2 
            AND active = true 
            AND CURRENT_TIMESTAMP BETWEEN start_date AND expiry_date
            RETURNING *
            "#,
            code,
            menu_name
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("Invalid or expired voucher")?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(voucher)
    }

    pub async fn create_voucher(
        pool: &PgPool,
        menu_name: &str,
        code: &str,
        discount_percent: f64,
        start_date: NaiveDateTime,
        expiry_date: NaiveDateTime,
    ) -> Result<MenuVoucher, String> {
        if discount_percent <= 0.0 || discount_percent > 100.0 {
            return Err("Discount must be between 0 and 100".to_string());
        }

        if start_date >= expiry_date {
            return Err("Start date must be before expiry date".to_string());
        }

        sqlx::query_as!(
            MenuVoucher,
            r#"
            INSERT INTO menu_vouchers (
                menu_name, code, discount_percent, 
                start_date, expiry_date
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            menu_name,
            code,
            discount_percent,
            start_date,
            expiry_date
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to create voucher: {}", e))
    }
}