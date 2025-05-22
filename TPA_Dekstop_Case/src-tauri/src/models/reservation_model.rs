use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: Uuid,
    pub user_id: Option<i32>,
    pub address: Option<String>,
    pub customer_name: Option<String>,
    pub people_count: Option<i32>,
    pub time_slot: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub time_limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: Uuid,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub capacity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationRequest {
    pub user_id: i32,
    pub address: String,
    pub customer_name: String,
    pub people_count: i32,
    pub time_slot: NaiveDateTime,
    pub time_limit: i32,
    pub table_ids: Option<Vec<String>>,
}