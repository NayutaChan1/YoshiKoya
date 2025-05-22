use chrono::NaiveTime;
use crate::models::branch_model::Branch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBranchHoursRequest {
    pub branch_address: String,
    pub opening_time: String,
    pub closing_time: String,
}

#[tauri::command]
pub async fn get_branch(address: String) -> Result<Branch, String> {
    Branch::get_branch(&address).await
}

#[tauri::command]
pub async fn get_branch_hours(address: String) -> Result<(String, String), String> {
    let branch = Branch::get_branch(&address).await?;
    let (opening_time, closing_time) = branch.format_time();
    Ok((opening_time, closing_time))
}

#[tauri::command]
pub async fn update_branch_hours(
    request: UpdateBranchHoursRequest
) -> Result<Branch, String> {

    

    let opening_time = request.opening_time.split(":").take(2).collect::<Vec<&str>>().join(":");
    let closing_time = request.closing_time.split(":").take(2).collect::<Vec<&str>>().join(":");

    let opening = NaiveTime::parse_from_str(&opening_time, "%H:%M")
        .map_err(|e| format!("Invalid opening time format: {}. Please use HH:MM format", e))?;
    
    let closing = NaiveTime::parse_from_str(&closing_time, "%H:%M")
        .map_err(|e| format!("Invalid closing time format: {}. Please use HH:MM format", e))?;

    Branch::update_branch_hours(&request.branch_address, opening, closing).await
}

#[tauri::command]
pub async fn calculate_is_open(address: String) -> Result<bool, String> {
    let branch = Branch::get_branch(&address).await?;
    let now = chrono::Local::now().naive_local().time();
    Ok(now >= branch.opening_time && now <= branch.closing_time)
}

#[tauri::command]
pub async fn get_all_branches() -> Result<Vec<Branch>, String> {
    Branch::get_all_branches().await
}

#[tauri::command]
pub async fn create_branch(
    branch_name: String,
    branch_address: String,
    opening_time: String,
    closing_time: String,
) -> Result<Branch, String> {
    // Parse time strings to NaiveTime
    let opening = NaiveTime::parse_from_str(&opening_time, "%H:%M")
        .map_err(|_| "Invalid opening time format. Use HH:MM".to_string())?;
    
    let closing = NaiveTime::parse_from_str(&closing_time, "%H:%M")
        .map_err(|_| "Invalid closing time format. Use HH:MM".to_string())?;

    Branch::create_branch(branch_name, branch_address, opening, closing).await
}

#[tauri::command]
pub async fn close_branch(address: String) -> Result<(), String> {
    Branch::close_branch(&address).await
}