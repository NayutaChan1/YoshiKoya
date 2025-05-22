use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;
use crate::service::database_service::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub total_amount: f64,
    pub transaction_date: NaiveDateTime,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDetail {
    pub id: i32,
    pub transaction_id: i32,
    pub menu_name: String,
    pub branch_address: String,
    pub price: f64,
    pub menu_type: String,
    pub quantity: i32,
}

impl Transaction {
    pub async fn create_transaction(
        user_id: i32,
        total_amount: f64,
    ) -> Result<i32, String> {

        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let result = sqlx::query!(
            r#"
            INSERT INTO transactions (user_id, total_amount, status)
            VALUES ($1, $2, 'pending')
            RETURNING id
            "#,
            user_id,
            total_amount
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.id)
    }

    pub async fn insert_transaction_items(
        transaction_id: i32,
        items: Vec<TransactionDetail>,
    ) -> Result<(), String> {

        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        for item in items {
            sqlx::query!(
                r#"
                INSERT INTO transaction_items (transaction_id, menu_name, branch_address, price, menu_type, quantity)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                transaction_id,
                item.menu_name,
                item.branch_address,
                item.price,
                item.menu_type,
                item.quantity
            )
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn get_user_transactions(user_id: i32) -> Result<Vec<Transaction>, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT id, user_id, total_amount, transaction_date, status
            FROM transactions
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(transactions)
    }

    pub async fn get_transaction_items(transaction_id: i32) -> Result<Vec<TransactionDetail>, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let items = sqlx::query_as!(
            TransactionDetail,
            r#"
            SELECT 
                id as "id!", 
                transaction_id as "transaction_id!", 
                menu_name as "menu_name!", 
                branch_address as "branch_address!", 
                price as "price!", 
                menu_type as "menu_type!", 
                quantity as "quantity!"
            FROM transaction_items
            WHERE transaction_id = $1
            "#,
            transaction_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(items)
    }

}