

use crate::models::employee_model::Employee;

#[tauri::command]
pub async fn get_employee(user_id: i32) -> Result<Employee, String> {
    Employee::get_employee(user_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_employees() -> Result<Vec<Employee>, String> {
    Employee::get_all_employees().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reassign_employee(
    user_id: i32,
    new_job: String,
) -> Result<Employee, String> {
    Employee::reassign_employee(user_id, new_job).await.map_err(|e| e.to_string())
}