use crate::{
    models::waitinglist_model::WaitingList,
    models::reservation_model::Reservation,
    service::database_service::Database,
};
use sqlx::PgPool;
use uuid::Uuid;

#[tauri::command]
pub async fn process_waiting_list() -> Result<(), String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    let entries = WaitingList::get_all_entries(pool).await?;

    for entry in entries {
        if let Some(reservation) = try_create_reservation(pool, &entry).await? {
            notify_user(
                entry.user_id,
                &format!(
                    "Your waitlisted reservation for {} has been confirmed!", 
                    entry.requested_time
                )
            ).await?;
        }
    }

    Ok(())
}

async fn try_create_reservation(pool: &PgPool, entry: &WaitingList) -> Result<Option<Reservation>, String> {
    let available_tables = entry.find_available_tables(pool).await?;

    let mut total_capacity = 0;
    let mut selected_tables = Vec::new();
    
    for (table_id, capacity) in &available_tables {
        if total_capacity < entry.people_count {
            selected_tables.push(*table_id);
            total_capacity += capacity;
        } else {
            break;
        }
    }

    if total_capacity >= entry.people_count {
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        let reservation = sqlx::query_as!(
            Reservation,
            r#"
            INSERT INTO reservations (
                user_id, address, customer_name, 
                people_count, time_slot, time_limit
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, address, customer_name, 
                      people_count, time_slot, created_at, time_limit
            "#,
            entry.user_id,
            entry.address,
            entry.customer_name,
            entry.people_count,
            entry.requested_time,
            entry.time_limit 
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        for table_id in selected_tables {
            sqlx::query!(
                r#"
                INSERT INTO reservation_tables (reservation_id, table_id)
                VALUES ($1, $2)
                "#,
                reservation.id,
                table_id
            )
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        entry.delete(pool).await?;
        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(Some(reservation))
    } else {
        Ok(None)
    }
}

async fn notify_user(user_id: i32, message: &str) -> Result<(), String> {
    println!("Notification to user {}: {}", user_id, message);
    Ok(())
}

#[tauri::command]
pub async fn get_branch_waiting_list(address: String) -> Result<Vec<WaitingList>, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    WaitingList::get_branch_waitlist(pool, &address).await
}

#[tauri::command]
pub async fn get_branch_reservations(address: String) -> Result<Vec<Reservation>, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    WaitingList::get_branch_reservations(pool, &address).await
}