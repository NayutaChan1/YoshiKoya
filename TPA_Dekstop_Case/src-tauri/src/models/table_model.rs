use crate::service::database_service::Database;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Table {
    pub id: String,
    pub capacity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTable {
    pub id: String,
    pub capacity: i32,
    pub is_available: bool,
    pub position: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Table {
    pub async fn get_available_tables(
        address: &str,
        time_slot: chrono::NaiveDateTime,
    ) -> Result<Vec<AvailableTable>, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        println!(
            "Checking for address: {} at time: {}",
            address,
            time_slot.and_utc()
        );

        let all_tables = sqlx::query_as!(
            Table,
            r#"
            SELECT id, capacity 
            FROM tables
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch tables: {}", e))?;

        println!("Found {} total tables", all_tables.len());

        let check_reservations = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM reservations
            WHERE address = $1
            "#,
            address
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check reservations: {}", e))?;

        println!(
            "Total reservations for this address: {}",
            check_reservations.count.unwrap_or(0)
        );

        let reserved_table_ids = sqlx::query!(
            r#"
            SELECT DISTINCT rt.table_id, r.time_slot, r.address
            FROM reservation_tables rt
            JOIN reservations r ON rt.reservation_id = r.id
            WHERE r.address = $1 
            AND r.time_slot BETWEEN $2::timestamptz - INTERVAL '120 minutes' 
                   AND $2::timestamptz + INTERVAL '120 minutes'
            "#,
            address,
            time_slot.and_utc()
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch reserved tables: {}", e))?;

        println!("Query results:");
        for row in &reserved_table_ids {
            println!(
                "Table ID: {}, Address: {}",
                row.table_id,
                row.address.as_deref().unwrap_or("No address")
            );
        }

        let reserved_ids: Vec<String> = reserved_table_ids
            .into_iter()
            .map(|row| row.table_id.to_string())
            .collect();

        println!("Reserved IDs: {:?}", reserved_ids);

        let available_tables = all_tables
            .into_iter()
            .enumerate()
            .map(|(index, table)| AvailableTable {
                id: table.id.clone(),
                capacity: table.capacity,
                is_available: !reserved_ids.contains(&table.id),
                position: format!("{},{}", index % 4, index / 4),
            })
            .collect();

        Ok(available_tables)
    }
}
