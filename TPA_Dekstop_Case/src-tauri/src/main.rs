// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // dotenvy::dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL tidak ditemukan");
    tpa_dekstop_case_lib::run()
}
