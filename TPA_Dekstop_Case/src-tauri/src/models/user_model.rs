//ada 3 class

use serde::{Serialize, Deserialize};
use tauri::command;
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::postgres::PgPoolOptions;
use redis::AsyncCommands;
use uuid::Uuid;
use crate::service::redis_services::{RedisService};
use crate::service::database_service::Database;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: Option<i32>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub user_password: Option<String>,
}

// #[derive(Deserialize)]
// struct Registrant {
//     username: String,
//     email: String,
//     password: String,
// }

// struct Pesannyakalobisa {
//     massage: String,
// }

// struct Login {
//     username: String,
//     password: String,
// }

impl User {
    //C
    pub async fn register_user(&self) -> Result<String, String> {
        // let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";
        // let pool = PgPoolOptions::new()
        //     .max_connections(5)
        //     .connect(database_url)
        //     .await
        //     .map_err(|e| e.to_string())?;
        
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya").await.map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let existing_user = sqlx::query!(
            "SELECT user_email FROM users WHERE user_email = $1",
            self.user_email
        )
        .fetch_optional(pool)
        .await
        .expect("Lu Skill Issue");
    
        if existing_user.is_some() {
            return Err("Udh Ada Woi".to_string());
        }
        

        let raw_password = self.user_password.as_ref().ok_or("Password is required")?;
        let hashed_password = hash(raw_password, DEFAULT_COST).map_err(|e| e.to_string())?;
    
        sqlx::query!(
            "INSERT INTO users (user_name, user_email, user_password) VALUES ($1, $2, $3)",
            self.user_name,
            self.user_email,
            hashed_password
        )
        .execute(pool)
        .await
        .expect("Lu Skill Issue");
    
        Ok("Berhasil Daftar".to_string())
    }

    

}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Login {
//     token: String,
// }

// impl Login {
//     // #[tauri::command]
//     async fn login_user(&self, username : &str, password : &str) -> Result<Login, String> {
//         let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";
//         let pool = PgPoolOptions::new()
//             .max_connections(5)
//             .connect(database_url)
//             .await
//             .map_err(|e| e.to_string())?;
    
//         let userlogin = sqlx::query!(
//             "SELECT user_password FROM users WHERE user_name = $1",
//             username
//         )
//         .fetch_optional(&pool)
//         .await
//         .expect("Jir Gagal");
    
//         let passwordnya = match userlogin {
//             Some(row) => row.user_password,
//             None => return Err("Salah Dan Gaada Woi".to_string()),
//         };
    
//         let is_valid = verify(password, &passwordnya).map_err(|e| e.to_string())?;
//         if !is_valid {
//             return Err("Username atau password salah!".to_string());
//         }
        
//         let token = Uuid::new_v4().to_string();
    
//         let userid = sqlx::query!(
//             "SELECT user_id FROM users WHERE user_name = $1",
//             username
//         )
//         .fetch_optional(&pool)
//         .await
//         .expect("Jir Gagal");
    
//         if let Some(user_db) = userid {
//             let user_data = serde_json::json!({
//                 "user_name": username,
//                 "user_id": user_db.user_id
//             });
    
//             let user_data = serde_json::to_string(&user_data).map_err(|e| e.to_string())?;
    
//             println!("User Data: {}", user_data);
            
//             let redis_service = RedisService::new().unwrap();
//             let mut redis_conn = redis_service.get_redis_connection().await?;
//             redis_conn
//                 .set_ex(&token, &user_data, 3600)
//                 .await
//                 .map_err(|e| e.to_string())?;
//         } else {
//             println!("User tidak ditemukan.");
//         }
    
//         Ok(Login {
//             token,
//         })
//     }
// }

