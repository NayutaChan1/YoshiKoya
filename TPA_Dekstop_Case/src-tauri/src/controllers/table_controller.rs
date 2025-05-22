use chrono::NaiveDateTime;
use crate::models::table_model::{AvailableTable, Table};

#[tauri::command]
pub async fn get_available_tables(
    address: String,
    time_slot: String,
) -> Result<Vec<AvailableTable>, String> {
    let time_slot = NaiveDateTime::parse_from_str(&time_slot, "%Y-%m-%dT%H:%M:%S%.fZ")
        .map_err(|e| format!("Invalid time format: {}", e))?;

    Table::get_available_tables(&address, time_slot).await
}