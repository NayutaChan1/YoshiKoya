use crate::{
    models::jobstatus_model::JobStatus,
    service::database_service::Database,
};

#[tauri::command]
pub async fn apply_for_job(
    user_id: i32,
    role: String,
    branch_address: String,
) -> Result<JobStatus, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    JobStatus::apply_job(pool, user_id, &role, &branch_address).await
}

#[tauri::command]
pub async fn update_job_status(
    user_id: i32,
    status: String,
) -> Result<JobStatus, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    JobStatus::update_status(pool, user_id, &status).await
}

#[tauri::command]
pub async fn get_pending_applications() -> Result<Vec<JobStatus>, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    JobStatus::get_pending_applications(pool).await
}

#[tauri::command]
pub async fn get_user_applications(user_id: i32) -> Result<Option<JobStatus>, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    JobStatus::get_user_application(pool, user_id).await
}