use redis::{aio::Connection, AsyncCommands, Client};
use std::env;

#[derive(Clone)]
pub struct RedisService {
    client: Client,
}

impl RedisService {
    pub fn new() -> Result<Self, String> {
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
        let client = Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }

    pub async fn get_redis_connection(&self) -> Result<Connection, String> {
        self.client
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn store_session(&self, token: &str, user_data: &str) -> Result<(), String> {
        let mut conn = self.get_redis_connection().await?;
        conn.set_ex(token, user_data, 3600)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
