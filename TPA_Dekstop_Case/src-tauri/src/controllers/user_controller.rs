use crate::models::user_model::User;
// use crate::models::;
use crate::service::database_service::Database;
use bcrypt::verify;
use redis::AsyncCommands;
use sqlx::postgres::PgPoolOptions;
use tauri::command;
use uuid::Uuid;

#[command]
pub async fn register_user(
    username: String,
    email: String,
    password: String,
) -> Result<String, String> {
    let user = User {
        user_id: None,
        user_name: Some(username),
        user_email: Some(email),
        user_password: Some(password),
    };

    user.register_user().await
}

#[command]
pub async fn login_user(username: String, password: String) -> Result<String, String> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:12345678@localhost:5432/YoshiKoya")
    //     .await
    //     .map_err(|e| e.to_string())?;

    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;

    let pool = db.get_pool();

    let user = sqlx::query!(
        "SELECT user_id, user_password FROM users WHERE user_name = $1",
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    let user = user.ok_or("User not found".to_string())?;

    let is_valid = verify(&password, &user.user_password).map_err(|e| e.to_string())?;
    if !is_valid {
        return Err("Invalid password".into());
    }

    let token = Uuid::new_v4().to_string();
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client
        .get_async_connection()
        .await
        .map_err(|e| e.to_string())?;

    let user_data = serde_json::json!({
        "user_name": username,
        "user_id": user.user_id
    });

    con.set_ex(&token, user_data.to_string(), 3600)
        .await
        .map_err(|e| e.to_string())?;

    Ok(token)
}

