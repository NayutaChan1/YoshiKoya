use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::reservation_model::Reservation;

#[derive(Debug, Serialize, Deserialize)]
pub struct WaitingList {
    pub id: i32,
    pub user_id: i32,
    pub address: String,
    pub customer_name: String,
    pub people_count: i32,
    pub requested_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub time_limit: i32,
}

impl WaitingList {
    pub async fn get_all_entries(pool: &PgPool) -> Result<Vec<WaitingList>, String> {
        sqlx::query_as!(
            WaitingList,
            r#"
            SELECT 
                id,
                user_id,
                address,
                customer_name,
                people_count,
                requested_time as "requested_time!",
                created_at as "created_at!",
                time_limit
            FROM waiting_list 
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch waiting list: {}", e))
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i32,
        address: &str,
        customer_name: &str,
        people_count: i32,
        requested_time: NaiveDateTime,
        time_limit: i32,
    ) -> Result<WaitingList, String> {
        sqlx::query_as!(
            WaitingList,
            r#"
            INSERT INTO waiting_list (
                user_id, address, customer_name, 
                people_count, requested_time, time_limit
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING 
                id, 
                user_id, 
                address, 
                customer_name, 
                people_count, 
                requested_time as "requested_time!", 
                created_at as "created_at!",
                time_limit
            "#,
            user_id,
            address,
            customer_name,
            people_count,
            requested_time,
            time_limit
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to create waiting list entry: {}", e))
    }

    pub async fn get_branch_waitlist(
        pool: &PgPool,
        address: &str,
    ) -> Result<Vec<WaitingList>, String> {
        sqlx::query_as!(
            WaitingList,
            r#"
            SELECT 
                id,
                user_id,
                address,
                customer_name,
                people_count,
                requested_time as "requested_time!",
                created_at as "created_at!",
                time_limit
            FROM waiting_list 
            WHERE address = $1
            ORDER BY created_at ASC
            "#,
            address
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch branch waiting list: {}", e))
    }

    pub async fn get_branch_reservations(
        pool: &PgPool,
        address: &str,
    ) -> Result<Vec<Reservation>, String> {
        sqlx::query_as!(
            Reservation,
            r#"
            SELECT 
                id,
                user_id,
                address,
                customer_name,
                people_count,
                time_slot as "time_slot!",
                created_at as "created_at!",
                time_limit as "time_limit!"
            FROM reservations 
            WHERE address = $1 
            AND time_slot > CURRENT_TIMESTAMP
            ORDER BY time_slot ASC
            "#,
            address
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch branch reservations: {}", e))
    }

    pub async fn find_available_tables(&self, pool: &PgPool) -> Result<Vec<(Uuid, i32)>, String> {
        sqlx::query!(
            r#"
            SELECT t.id, t.capacity
            FROM tables t
            JOIN reservations r ON r.address = $1
            WHERE t.id NOT IN (
                SELECT rt.table_id
                FROM reservation_tables rt
                JOIN reservations r ON rt.reservation_id = r.id
                WHERE r.address = $1 
                AND (
                    r.time_slot BETWEEN $2::timestamp 
                    AND ($2::timestamp + make_interval(mins => r.time_limit))
                    OR
                    ($2::timestamp + make_interval(mins => $3)) BETWEEN r.time_slot 
                    AND (r.time_slot + make_interval(mins => r.time_limit))
                )
            )
            ORDER BY t.capacity
            "#,
            self.address,
            self.requested_time,
            self.time_limit
        )
        .fetch_all(pool)
        .await
        .map(|rows| rows.into_iter().map(|row| (row.id, row.capacity)).collect())
        .map_err(|e| format!("Failed to find available tables: {}", e))
    }

    pub async fn delete(&self, pool: &PgPool) -> Result<(), String> {
        sqlx::query!(
            r#"
            DELETE FROM waiting_list
            WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|e| format!("Failed to delete waiting list entry: {}", e))
    }
}
