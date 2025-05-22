use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPool;
use crate::service::database_service::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuWithImage {
    pub name: String,
    pub price: f64,
    pub menu_type: String,
    pub image_bytes: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawMenu {
    pub name: String,
    pub price: f64,
    #[serde(rename = "type")]
    pub menu_type: String,
}

impl MenuWithImage {
    pub async fn get_image_from_db(menu_name: &str) -> Result<Option<String>, String> {

        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let result = sqlx::query!(
            "SELECT image_base64 FROM menus WHERE menu_name = $1",
            menu_name
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.map(|row| row.image_base64))
    }

    pub async fn get_menu_by_menu_name(menu_name: &str) -> Result<MenuWithImage, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let result = sqlx::query!(
            "SELECT image_base64, address, menu_type, price FROM menus WHERE menu_name = $1",
            menu_name
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        match result {
            Some(row) => Ok(MenuWithImage {
                name: menu_name.to_string(),
                price: row.price.unwrap_or_default(),
                menu_type: row.menu_type.unwrap_or_default(),
                image_bytes: Some(row.image_base64),
                address: row.address,
            }),
            None => Err("Menu not found".to_string()),
        }
    }

    pub async fn get_all_menus() -> Result<Vec<MenuWithImage>, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let results = sqlx::query!(
            r#"
            SELECT 
                menu_name, 
                image_base64, 
                address, 
                menu_type, 
                price 
            FROM menus
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch menus: {}", e))?;

        let menus = results
            .into_iter()
            .map(|row| MenuWithImage {
                name: row.menu_name,
                price: row.price.unwrap_or_default(),
                menu_type: row.menu_type.unwrap_or_default(),
                image_bytes: Some(row.image_base64),
                address: row.address,
            })
            .collect();

        Ok(menus)
    }

    pub async fn get_branch_menus(branch_address: &str) -> Result<Vec<MenuWithImage>, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let results = sqlx::query!(
            r#"
            SELECT 
                menu_name, 
                image_base64, 
                address, 
                menu_type, 
                price 
            FROM menus
            WHERE address = $1
            ORDER BY menu_type, menu_name
            "#,
            branch_address
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch branch menus: {}", e))?;

        let menus = results
            .into_iter()
            .map(|row| MenuWithImage {
                name: row.menu_name,
                price: row.price.unwrap_or_default(),
                menu_type: row.menu_type.unwrap_or_default(),
                image_bytes: Some(row.image_base64),
                address: row.address,
            })
            .collect();

        Ok(menus)
    }

}