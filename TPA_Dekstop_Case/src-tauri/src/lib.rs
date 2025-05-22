mod models;
mod controllers;
mod service;

use controllers::user_controller::{self, login_user, register_user};
use controllers::checksession_controller::check_session_handler;
use controllers::employee_controller::{get_employee, get_all_employees, reassign_employee};
use controllers::branch_controller::{update_branch_hours, get_branch, get_branch_hours, calculate_is_open, close_branch, get_all_branches, create_branch};
use controllers::rediscart_controller::{add_to_cart, get_cart_items, update_cart};
use controllers::transaction_controller::{create_transaction, get_user_transactions};
use crate::controllers::menu_controller::{get_all_menus, get_menu_details, get_branch_menus};
use crate::controllers::reservation_controller::{create_reservation};
use crate::controllers::table_controller::{get_available_tables};
use crate::controllers::waitinglist_controller::{process_waiting_list, get_branch_reservations, get_branch_waiting_list};
use crate::controllers::voucher_controller::{get_menu_vouchers, apply_menu_voucher, create_menu_voucher};
use crate::controllers::jobstatus_controller::{apply_for_job, update_job_status, get_pending_applications, get_user_applications};


use tauri::{generate_handler, Builder};
use tauri::Manager;

// use core::error;
// use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Number};
// use sqlx::{PgPool, Error};
use sqlx::postgres::PgPoolOptions;
use std::{env, result};
use tauri::State;
// use tauri::{http::response, State};
use reqwest::Client;
use std::error::Error;
// use tauri::command;
// use tokio_postgres::{NoTls, Error};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Local, NaiveTime, Timelike};
use redis::AsyncCommands;
use std::sync::Mutex;
use uuid::Uuid;

use base64::decode;

#[derive(Deserialize)]
struct Registrant {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct UserData {
    user_name: String,
    user_id: i32,
}

#[derive(Deserialize)]
struct SessionCheck {
    token: String,
}

// #[derive(Serialize)]
// struct AuthResponse {
//     message: String,
//     is_authenticated: bool,
// }

#[derive(Serialize)]
struct Pesannyakalobisa {
    massage: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Menu {
    name: String,
    price: f64,
    menu_type: String,
}

#[derive(Debug, Serialize)]
struct MenuWithImage {
    name: String,
    price: f64,
    menu_type: String,
    image_bytes: Option<String>,
}

#[derive(Debug, Serialize)]
struct MenuWithBranch {
    menu_name: String,
    branch_name: String,
    branch_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CartItem {
    menu_name: String,
    branch_name: String,
    branch_address: String,
    price: f64,
    menu_type: String,
    quantity: u32,
}

#[derive(Default)]
struct CartState {
    items: Mutex<Vec<CartItem>>,
}

#[derive(Debug, Serialize)]
struct BranchInfo {
    branch_name: String,
    branch_address: String,
}

#[derive(Debug, Serialize)]
struct OperationHours {
    opening_time: String,
    closing_time: String,
    is_open: bool,
}

//udah
// async fn get_redis_connection() -> Result<redis::aio::Connection, String> {
//     let client = redis::Client::open("redis://127.0.0.1/").map_err(|e| e.to_string())?;
//     let conn = client
//         .get_async_connection()
//         .await
//         .map_err(|e| e.to_string())?;
//     Ok(conn)
// }

//udah
// #[tauri::command]
// async fn register_user(user: Registrant) -> Result<Pesannyakalobisa, String> {
//     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await
//         .map_err(|e| e.to_string())?;

//     let existing_user = sqlx::query!(
//         "SELECT user_email FROM users WHERE user_email = $1",
//         user.email
//     )
//     .fetch_optional(&pool)
//     .await
//     .expect("Lu Skill Issue");

//     if existing_user.is_some() {
//         return Err("Udh Ada Woi".to_string());
//     }

//     let hashed_password = hash(user.password, DEFAULT_COST).map_err(|e| e.to_string())?;

//     sqlx::query!(
//         "INSERT INTO users (user_name, user_email, user_password) VALUES ($1, $2, $3)",
//         user.username,
//         user.email,
//         hashed_password
//     )
//     .execute(&pool)
//     .await
//     .expect("Lu Skill Issue");

//     Ok(Pesannyakalobisa {
//         massage: "Allhamdulilah Bisa".to_string(),
//     })
// }

// //udah
// #[tauri::command]
// async fn login_user(user: Login) -> Result<LoginResponse, String> {
//     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await
//         .map_err(|e| e.to_string())?;

//     let userlogin = sqlx::query!(
//         "SELECT user_password FROM users WHERE user_name = $1",
//         user.username
//     )
//     .fetch_optional(&pool)
//     .await
//     .expect("Jir Gagal");

//     let passwordnya = match userlogin {
//         Some(row) => row.user_password,
//         None => return Err("Salah Dan Gaada Woi".to_string()),
//     };

//     let is_valid = verify(user.password, &passwordnya).map_err(|e| e.to_string())?;
//     if !is_valid {
//         return Err("Username atau password salah!".to_string());
//     }

//     let token = Uuid::new_v4().to_string();

//     let userid = sqlx::query!(
//         "SELECT user_id FROM users WHERE user_name = $1",
//         user.username
//     )
//     .fetch_optional(&pool)
//     .await
//     .expect("Jir Gagal");

//     if let Some(user_db) = userid {
//         let user_data = serde_json::json!({
//             "user_name": user.username,
//             "user_id": user_db.user_id
//         });

//         let user_data = serde_json::to_string(&user_data).map_err(|e| e.to_string())?;

//         println!("User Data: {}", user_data);

//         let mut redis_conn = get_redis_connection().await?;
//         redis_conn
//             .set_ex(&token, &user_data, 3600)
//             .await
//             .map_err(|e| e.to_string())?;
//     } else {
//         println!("User tidak ditemukan.");
//     }

//     // let mut redis_conn = get_redis_connection().await?;
//     // redis_conn
//     //     .set_ex(&token, &user_data, 3600)
//     //     .await
//     //     .map_err(|e| e.to_string())?;

//     Ok(LoginResponse {
//         message: "Login berhasil!".to_string(),
//         token,
//     })

//     // Ok(Pesannyakalobisa { massage: "Bisa Login Anjay".to_string() })
// }

// #[tokio::main]

//gaguna
// async fn fetch_and_store_data() -> Result<(), Box<dyn Error>> {
//     let client = Client::new();

//     let graph_ql = json!({
//         "query" : "{ getUsers { name email } }"
//     });

//     let response = client
//         .post("https://yoshikoya.vercel.app/api/graphql")
//         .json(&graph_ql)
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();

//     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";

//     let data: serde_json::Value = serde_json::from_str(&response).unwrap();
//     let users = data["data"]["getUsers"].as_array().unwrap();

//     // let (db_client, connection) = tokio_postgres::connect("host=5321 user=postgres dbname=YoshiKoya password=12345678", NoTls).await?;

//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await?;

//     // let pool = PgPool::connect(&database_url).await?;

//     println!("Gacor");

//     let data: serde_json::Value = serde_json::from_str(&response)?;
//     let users = data["data"]["getUsers"]
//         .as_array()
//         .ok_or("Invalid JSON")?;

//     for user in users {
//         let name = user["name"].as_str().ok_or("Invalid name data")?;
//         let email = user["email"].as_str().ok_or("Invalid email data")?;

//         let password_query = json!({
//             "query" : "query Query($email: String!) { getUserPasswordByEmail(email : $email) }",
//             "variables" : { "email": email }
//         });

//         let password_response = client
//             .post("https://yoshikoya.vercel.app/api/graphql")
//             .json(&password_query)
//             .send()
//             .await?
//             .text()
//             .await?;

//         let password_data: serde_json::Value = serde_json::from_str(&password_response)?;
//         let password = password_data["data"]["getUserPasswordByEmail"]
//             .as_str()
//             .ok_or("Invalid password")?;

//         // let hashed_password = hash(password, DEFAULT_COST)?;

//         // let resul2t = sqlx::query!()

//         sqlx::query!("INSERT INTO users (user_name, user_email, user_password) VALUES ($1, $2, $3) ON CONFLICT (user_email) DO UPDATE SET user_name = EXCLUDED.user_name, user_password = EXCLUDED.user_password;",
//             name,
//             email,
//             password).execute(&pool).await.expect("Lu Skill Issue");
//     }
//     Ok(())
// }

// #[tauri::command]
// async fn check_session(session: SessionCheck) -> Result<UserData, String> {
//     let mut redis_conn = get_redis_connection().await?;

//     let user_data_json: Option<String> = redis_conn
//         .get(&session.token)
//         .await
//         .map_err(|e| e.to_string())?;

//     match user_data_json {
//         Some(json_data) => match serde_json::from_str::<UserData>(&json_data) {
//             Ok(user_data) => Ok(user_data),
//             Err(e) => {
//                 println!("Deserialization error: {:?}", e);
//                 Err("gagal".to_string())
//             }
//         },
//         None => Err("Session gaada".to_string()),
//     }
// }

// #[tauri::command]
// async fn get_user_job(user_id: i32) -> Result<String, String> {
//     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";

//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await
//         .map_err(|e| e.to_string())?;

//     let result = sqlx::query!("SELECT job FROM employee WHERE user_id = $1", user_id)
//         .fetch_optional(&pool)
//         .await
//         .map_err(|e| e.to_string())?;

//     match result {
//         Some(row) => Ok(row.job),
//         None => Err("Anda Tidak Punya Kerja".to_string()),
//     }
// }

// #[tauri::command]
// async fn get_employee_code(user_id: i32) -> Result<String, String> {
//     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";

//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await
//         .map_err(|e| e.to_string())?;

//     let result = sqlx::query!(
//         "SELECT employee_code FROM employee WHERE user_id = $1",
//         user_id
//     )
//     .fetch_optional(&pool)
//     .await
//     .map_err(|e| e.to_string())?;

//     match result {
//         Some(row) => Ok(row.employee_code),
//         None => Err("Anda Tidak Punya Kerja".to_string()),
//     }
// }

// // #[tauri::command]
// // async fn start_fetch() -> Result<String, String> {
// //     fetch_and_store_data().await.map_err(|e| e.to_string())?;
// //     let response = json!({ "message": "Data nya aman kah?" });
// //     Ok(response.to_string())
// // }

// #[tauri::command]
// async fn get_address() -> Result<Vec<MenuWithBranch>, String> {
//     let client = Client::new();

//     let graph_ql = json!({
//         "query": "{ getMenus { name branch { address name } } }"
//     });

//     let response = client
//         .post("https://yoshikoya.vercel.app/api/graphql")
//         .json(&graph_ql)
//         .send()
//         .await
//         .map_err(|e| e.to_string())?
//         .text()
//         .await
//         .map_err(|e| e.to_string())?;

//     // println!("GraphQL response: {}", response);

//     let data: serde_json::Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;

//     let menus = data["data"]["getMenus"]
//         .as_array()
//         .ok_or("Failed to fetc".to_string())?;

//     let mut menu_with_branches = Vec::new();

//     for menu in menus {
//         let menu_name = menu
//             .get("name")
//             .and_then(|v| v.as_str())
//             .unwrap_or("")
//             .to_string();
//         if let Some(branch) = menu.get("branch") {
//             let branch_name = branch
//                 .get("name")
//                 .and_then(|v| v.as_str())
//                 .unwrap_or("")
//                 .to_string();
//             let branch_address = branch
//                 .get("address")
//                 .and_then(|v| v.as_str())
//                 .unwrap_or("")
//                 .to_string();

//             menu_with_branches.push(MenuWithBranch {
//                 menu_name,
//                 branch_name,
//                 branch_address,
//             });
//         }
//     }

//     Ok(menu_with_branches)
// }

// // #[tauri::command]
// // async fn fetch_and_check_images() -> Result<Vec<MenuWithImage>, String> {
// //     let client = Client::new();

// //     let graph_ql = json!({
// //         "query": "{ getMenus { name price type } }"
// //     });

// //     let response = client
// //         .post("https://yoshikoya.vercel.app/api/graphql")
// //         .json(&graph_ql)
// //         .send()
// //         .await
// //         .map_err(|e| e.to_string())?
// //         .text()
// //         .await
// //         .map_err(|e| e.to_string())?;

// //     let data: serde_json::Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;

// //     // println!("GraphQL response: {}", response);

// //     let raw_menus = data["data"]["getMenus"]
// //         .as_array()
// //         .ok_or("Data gak sesuai".to_string())?;

// //     // println!("Raw menus: {:?}", raw_menus);

// //     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";
// //     let pool = PgPoolOptions::new()
// //         .max_connections(5)
// //         .connect(database_url)
// //         .await
// //         .map_err(|e| e.to_string())?;

// //     let mut final_menus = Vec::new();

// //     for item in raw_menus {
// //         let name = item
// //             .get("name")
// //             .and_then(|v| v.as_str())
// //             .unwrap_or("")
// //             .to_string();
// //         let price = item.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
// //         let menu_type = item
// //             .get("type")
// //             .and_then(|v| v.as_str())
// //             .unwrap_or("")
// //             .to_string();
// //         // println!("Final menus: {:?}", name);
// //         let result = sqlx::query!("SELECT image_base64 FROM menus WHERE menu_name = $1", name)
// //             .fetch_optional(&pool)
// //             .await
// //             .map_err(|e| e.to_string())?;

// //         let image_bytes = if let Some(row) = result {
// //             Some(row.image_base64)
// //         } else {
// //             None
// //         };

// //         final_menus.push(MenuWithImage {
// //             name,
// //             price,
// //             menu_type,
// //             image_bytes,
// //         });
// //     }
// //     // println!("Final menus: {:?}", final_menus);
// //     Ok(final_menus)
// // }

// #[tauri::command]
// async fn get_branch_info(employee_code: String) -> Result<Option<BranchInfo>, String> {
//     let client = Client::new();

//     let graph_ql = json!({
//         "query": r#"
//         {
//             getEmployees {
//                 employeeCode
//                 branch {
//                     address
//                     name
//                 }
//             }
//         }
//         "#
//     });

//     let response = client
//         .post("https://yoshikoya.vercel.app/api/graphql")
//         .json(&graph_ql)
//         .send()
//         .await
//         .map_err(|e| e.to_string())?
//         .text()
//         .await
//         .map_err(|e| e.to_string())?;

//     let data: serde_json::Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;

//     let employees = data["data"]["getEmployees"]
//         .as_array()
//         .ok_or("Failed to fetch employee".to_string())?;

//     let matching_employee = employees
//         .iter()
//         .find(|emp| emp["employeeCode"].as_str() == Some(&employee_code));

//     if let Some(employee) = matching_employee {
//         if let Some(branch) = employee.get("branch") {
//             let branch_name = branch
//                 .get("name")
//                 .and_then(|v| v.as_str())
//                 .unwrap_or("")
//                 .to_string();
//             let branch_address = branch
//                 .get("address")
//                 .and_then(|v| v.as_str())
//                 .unwrap_or("")
//                 .to_string();

//             return Ok(Some(BranchInfo {
//                 branch_name,
//                 branch_address,
//             }));
//         }
//     }

//     Ok(None)
// }

// #[tauri::command]
// async fn get_branch_operation_hours(branch_name: String) -> Result<OperationHours, String> {
//     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";
//     let pool = sqlx::postgres::PgPoolOptions::new()
//         .max_connections(5)
//         .acquire_timeout(std::time::Duration::from_secs(5))
//         .connect(database_url)
//         .await
//         .map_err(|e| format!("Gagal terhubung ke database: {}", e))?;

//     let row= sqlx::query!(
//         "SELECT 
//             operation_hours_open as \"opening_time!\",
//             operation_hours_close as \"closing_time!\" 
//          FROM branches 
//          WHERE branchname = $1",
//         branch_name
//     )
//     .fetch_optional(&pool)
//     .await
//     .map_err(|e| format!("Kesalahan query: {}", e))?
//     .ok_or_else(|| "Cabang tidak ditemukan".to_string())?;

//     let now = chrono::Local::now()
//         .naive_local()
//         .time();
    
//     let format_time = |t: chrono::NaiveTime| -> String {
//         t.format("%H:%M").to_string()
//     };

//     Ok(OperationHours {
//         opening_time: format_time(row.opening_time),
//         closing_time: format_time(row.closing_time),
//         is_open: now >= row.opening_time && now <= row.closing_time,
//     })
// }

// #[derive(Debug, Deserialize)]
// struct UpdateOperationHoursRequest {
//     branch_name: String,
//     opening_time: String,
//     closing_time: String,
// }

// #[tauri::command]
// async fn update_operation_hours(request: UpdateOperationHoursRequest) -> Result<String, String> {
//     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(database_url)
//         .await
//         .map_err(|e| format!("Database errir: {}", e))?;

//     let opening_time = NaiveTime::parse_from_str(&request.opening_time, "%H:%M")
//         .map_err(|e| format!("Fromat buka salah: {}", e))?;
//     let closing_time = NaiveTime::parse_from_str(&request.closing_time, "%H:%M")
//         .map_err(|e| format!("Format tutup salah: {}", e))?;

//     if opening_time >= closing_time {
//         return Err("Buka 24 jam".to_string());
//     }

//     let result = sqlx::query!(
//         "UPDATE branches 
//          SET operation_hours_open = $1, operation_hours_close = $2 
//          WHERE branchname = $3",
//         opening_time,
//         closing_time,
//         request.branch_name
//     )
//     .execute(&pool)
//     .await
//     .map_err(|e| format!("Failed to update operation hours: {}", e))?;

//     // if result.rows_affected() == 0 {
//     //     return Err("Branch not found".to_string());
//     // }

//     Ok("Operation hours updated success".to_string())
// }

// pub async fn check_session_handler(token : String) -> Result<CheckSession, String> {
//     CheckSession::check_session(&token).await.map_err(|e| e.to_string())
// }

// #[tauri::command]
// fn add_to_cart(state: State<CartState>, item: CartItem) -> Result<(), String> {
//     let mut cart = state.items.lock().map_err(|e| e.to_string())?;
//     cart.push(item);
//     Ok(())
// }

// #[tauri::command]
// fn get_cart_items(state: State<CartState>) -> Result<Vec<CartItem>, String> {
//     let cart = state.items.lock().map_err(|e| e.to_string())?;
//     Ok(cart.clone())
// }

// #[tauri::command]
// fn update_cart_items(state: tauri::State<CartState>, items: Vec<CartItem>) {
//     let mut cart = state.items.lock().unwrap();
//     *cart = items;
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    



    tauri::Builder::default()

        // .setup(|app| {
        //     let database_url = "postgres://postgres:12345678@localhost:5432/YoshiKoya";
        //     let db = Database::new(database_url);
        //     // app.manage(db);
        //     Ok(())
        // })

        .manage(CartState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // start_fetch,
            register_user, // refactor
            login_user, //refactor
            check_session_handler, // refactor
            // get_user_job,
            // get_employee_code,
            get_employee, // refactor
            // fetch_and_check_images, // refactor
            // get_address,
            add_to_cart, // refactor
            get_cart_items, // refactor
            update_cart, // refactor
            // add_to_cart,
            // get_cart_items,
            // update_cart_items,
            // get_branch_info,
            // get_branch_operation_hours,
            get_branch_hours, // refactor
            update_branch_hours, // refactor
            get_branch, // refactor
            calculate_is_open, // refactor
            get_all_menus, // refactor
            get_menu_details, // refactor
            create_transaction, // refactor
            get_user_transactions, // refactor
            create_reservation, // refactor
            get_all_branches, // refactor
            get_available_tables, // refactor
            process_waiting_list, // refactor
            get_branch_reservations, // refactor
            get_branch_waiting_list, // refactor
            create_branch, // refactor
            close_branch, // refactor
            get_menu_vouchers, // refactor
            apply_menu_voucher, // refactor
            create_menu_voucher, // refactor
            get_branch_menus, // refactor
            apply_for_job, // refactor
            update_job_status, // refactor
            get_pending_applications, // refactor
            get_user_applications, // refactor
            get_all_employees, // refactor
            reassign_employee, // refactor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
