use serde::{Serialize, Deserialize};
// use crate::service::redis_services::RedisService;
use crate::service::redis_services::{RedisService};
use redis::AsyncCommands;


#[derive(Debug, Serialize, Deserialize)]
pub struct CheckSession {
    user_name: String,
    user_id: i32,
}

impl CheckSession {
    pub async fn check_session(token : &str) -> Result<CheckSession, String> {
        let redis_service = RedisService::new().unwrap();
        let mut redis_conn = redis_service.get_redis_connection().await?;
    
    
        let user_data_json: Option<String> = redis_conn
            .get(token)
            .await
            .map_err(|e| e.to_string())?;
    
        match user_data_json {
            Some(json_data) => match serde_json::from_str::<CheckSession>(&json_data) {
                Ok(user_data) => Ok(user_data),
                Err(e) => {
                    println!("Raw JSON from Redis: {}", json_data);
                    println!("Deserialization error: {:?}", e);
                    Err("gagal".to_string())
                }
            },
            None => Err("Session gaada".to_string()),
        }
    }
}
