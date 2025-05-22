use crate::{
    models::voucher_model::MenuVoucher,
    service::database_service::Database,
};
use chrono::NaiveDateTime;
use tauri::command;

#[command]
pub async fn get_menu_vouchers(menu_name: String) -> Result<Vec<MenuVoucher>, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    MenuVoucher::find_by_menu_name(pool, &menu_name).await
}

#[command]
pub async fn apply_menu_voucher(
    code: String, 
    menu_name: String
) -> Result<MenuVoucher, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    MenuVoucher::apply_voucher(pool, &code, &menu_name).await
}

#[command]
pub async fn create_menu_voucher(
    menu_name: String,
    code: String,
    discount_percent: f64,
    start_date: String,
    expiry_date: String,
) -> Result<MenuVoucher, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    let start = NaiveDateTime::parse_from_str(&start_date, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "Invalid start date format. Use YYYY-MM-DD HH:MM:SS".to_string())?;
    
    let expiry = NaiveDateTime::parse_from_str(&expiry_date, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "Invalid expiry date format. Use YYYY-MM-DD HH:MM:SS".to_string())?;

    MenuVoucher::create_voucher(
        pool,
        &menu_name,
        &code,
        discount_percent,
        start,
        expiry
    ).await
}