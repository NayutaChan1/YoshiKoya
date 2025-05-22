use redis::{AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub menu_name: String,
    pub branch_address: String,
    pub price: f64,
    pub menu_type: String,
    pub quantity: u32,
}

impl CartItem {
    pub async fn add_to_cart(
        user_id: &str, 
        item: CartItem, 
        redis: &mut redis::aio::Connection
    ) -> Result<(), RedisError> {
        let key = format!("cart:{}", user_id);
        let json = serde_json::to_string(&item)
            .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "JSON serialization failed", e.to_string())))?;
        redis.rpush(key, json).await?;
        Ok(())
    }

    pub async fn get_cart(
        user_id: &str, 
        redis: &mut redis::aio::Connection
    ) -> Result<Vec<CartItem>, RedisError> {
        let key = format!("cart:{}", user_id);
        let items: Vec<String> = redis.lrange(key, 0, -1).await?;
        let cart_items = items
            .into_iter()
            .filter_map(|json| {
                serde_json::from_str(&json)
                    .map_err(|e| println!("Failed to parse cart item: {}", e))
                    .ok()
            })
            .collect();
        Ok(cart_items)
    }

    pub async fn update_cart(
        user_id: &str, 
        items: Vec<CartItem>, 
        redis: &mut redis::aio::Connection
    ) -> Result<(), RedisError> {
        let key = format!("cart:{}", user_id);
        redis.del(&key).await?;
        for item in items {
            let json = serde_json::to_string(&item)
                .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "JSON serialization failed", e.to_string())))?;
            redis.rpush(&key, json).await?;
        }
        Ok(())
    }
}