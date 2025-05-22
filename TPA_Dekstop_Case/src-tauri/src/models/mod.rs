// use serde::{Deserialize, Serialize};
pub mod user_model;
pub mod checksession_model;
pub mod employee_model;
pub mod menu_model;
pub mod branch_model;
pub mod rediscart_model;
pub mod transaction_model;
pub mod reservation_model;
pub mod table_model;
pub mod waitinglist_model;
pub mod voucher_model;
pub mod jobstatus_model;

// pub struct User {
//     pub user_id: Option<i32>,
//     pub user_name: Option<String>,
//     pub user_email: Option<String>,
//     pub user_password: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Login {
//     token: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct CheckSession {
//     user_name: String,
//     user_id: i32,
// }