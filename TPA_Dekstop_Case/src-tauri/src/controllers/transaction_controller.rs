use crate::models::transaction_model::{Transaction, TransactionDetail};
use crate::service::database_service::Database;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub menu_name: String,
    pub branch_address: String,
    pub price: f64,
    pub menu_type: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub user_id: i32,
    pub cart_items: Vec<CartItem>,
    pub total_amount: f64,
}

#[tauri::command]
pub async fn create_transaction(request: CreateTransactionRequest) -> Result<(), String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let transaction_id = Transaction::create_transaction(request.user_id, request.total_amount)
        .await
        .map_err(|e| e.to_string())?;

    let transaction_items: Vec<TransactionDetail> = request
        .cart_items
        .into_iter()
        .map(|item| TransactionDetail {
            id: 0,
            transaction_id,
            menu_name: item.menu_name,
            branch_address: item.branch_address,
            price: item.price,
            menu_type: item.menu_type,
            quantity: item.quantity,
        })
        .collect();

    Transaction::insert_transaction_items(transaction_id, transaction_items)
        .await
        .map_err(|e| format!("Failed to insert transaction items: {}", e))?;

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_user_transactions(user_id: i32) -> Result<Vec<Transaction>, String> {
    Transaction::get_user_transactions(user_id)
        .await
        .map_err(|e| format!("Failed to fetch transactions: {}", e))
}
