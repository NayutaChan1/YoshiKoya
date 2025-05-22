use crate::models::checksession_model::CheckSession;


#[tauri::command]
pub async fn check_session_handler(token : String) -> Result<CheckSession, String> {
    CheckSession::check_session(&token).await.map_err(|e| e.to_string())
}