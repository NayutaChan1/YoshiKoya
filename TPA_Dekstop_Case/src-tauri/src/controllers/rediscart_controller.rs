use crate::models::rediscart_model::CartItem;
use crate::service::redis_services::RedisService;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AddToCartRequest {
    pub user_id: String,
    pub menu_name: String,
    pub branch_address: String,
    pub price: f64,
    pub menu_type: String,
    pub quantity: u32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCartRequest {
    pub user_id: String,
    pub cart_items: Vec<CartItem>,
}

#[tauri::command]
pub async fn add_to_cart(request: AddToCartRequest) -> Result<(), String> {
    let redis_service = RedisService::new().unwrap();
    let mut redis_conn = redis_service.get_redis_connection().await?;

    let cart_item = CartItem {
        menu_name: request.menu_name,
        branch_address: request.branch_address,
        price: request.price,
        menu_type: request.menu_type,
        quantity: request.quantity,
    };
    
    CartItem::add_to_cart(&request.user_id, cart_item, &mut redis_conn)
        .await
        .map_err(|e| format!("Failed to add to cart: {}", e))
}

#[tauri::command]
pub async fn get_cart_items(user_id: String) -> Result<Vec<CartItem>, String> {
    let redis_service = RedisService::new().unwrap();
    let mut redis_conn = redis_service.get_redis_connection().await?;

    CartItem::get_cart(&user_id, &mut redis_conn)
        .await
        .map_err(|e| format!("Failed to get cart items: {}", e))
}

#[tauri::command]
pub async fn update_cart(request: UpdateCartRequest) -> Result<(), String> {
    let redis_service = RedisService::new().unwrap();
    let mut redis_conn = redis_service.get_redis_connection().await?;

    CartItem::update_cart(&request.user_id, request.cart_items, &mut redis_conn)
        .await
        .map_err(|e| format!("Failed to update cart: {}", e))
}