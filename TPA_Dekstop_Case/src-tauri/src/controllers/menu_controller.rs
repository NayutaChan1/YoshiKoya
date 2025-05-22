use crate::{models::menu_model::{MenuWithImage, RawMenu}, service::{database_service::Database, graphql_service::GraphQLService}};
use reqwest::Client;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;

// #[tauri::command]
// pub async fn fetch_and_check_images() -> Result<Vec<MenuWithImage>, String> {
//     let client = Client::new();

//     let service = GraphQLService::new("https://yoshikoya.vercel.app/api/graphql");

//     let query = r#"
//     {
//         getMenus {
//             name
//             price
//             type
//         }
//     }
//     "#;

//     let result = service.query(query).await.map_err(|e| e.to_string())?;
    
//     // let response = client
//     //     .post("https://yoshikoya.vercel.app/api/graphql")
//     //     .json(&graph_ql)
//     //     .send()
//     //     .await
//     //     .map_err(|e| e.to_string())?
//     //     .text()
//     //     .await
//     //     .map_err(|e| e.to_string())?;

//     // let data: serde_json::Value = serde_json::from_str(&result)
//     //     .map_err(|e| e.to_string())?;

//     let raw_menus: Vec<RawMenu> = serde_json::from_value(result["data"]["getMenus"].clone())
//         .map_err(|_| "Data tidak valid".to_string())?;

//     let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya").await.map_err(|e| e.to_string())?;
//     let pool = db.get_pool();

//     let mut final_menus = Vec::new();
    
//     for menu in raw_menus {
//         let image_bytes = MenuWithImage::get_image_from_db(&menu.name).await?;
//         let address = Me

//         final_menus.push(MenuWithImage {
//             name: menu.name,
//             price: menu.price,
//             menu_type: menu.menu_type,
//             image_bytes,
//             address,
//         });
//     }

//     Ok(final_menus)
// }

#[tauri::command]
pub async fn get_menu_details(menu_name: String) -> Result<MenuWithImage, String> {
    let menu = MenuWithImage::get_menu_by_menu_name(&menu_name).await?;
    Ok(menu)
}

#[tauri::command]
pub async fn get_all_menus() -> Result<Vec<MenuWithImage>, String> {
    let menus = MenuWithImage::get_all_menus().await?;
    
    let responses = menus
        .into_iter()
        .map(|menu| MenuWithImage {
            name: menu.name,
            price: menu.price,
            menu_type: menu.menu_type,
            image_bytes: menu.image_bytes,
            address: menu.address,
        })
        .collect();

    Ok(responses)
}

#[tauri::command]
pub async fn get_branch_menus(branch_address: String) -> Result<Vec<MenuWithImage>, String> {
    MenuWithImage::get_branch_menus(&branch_address).await
}