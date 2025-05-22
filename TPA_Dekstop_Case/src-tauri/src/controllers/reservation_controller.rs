use crate::models::waitinglist_model::WaitingList;
use crate::{
    models::reservation_model::{Reservation, ReservationRequest},
    service::database_service::Database,
};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ReservationResponse {
    pub reservation: Option<Reservation>,
    pub is_waitlisted: bool,
    pub message: String,
}

#[tauri::command]
pub async fn create_reservation(
    request: ReservationRequest,
) -> Result<ReservationResponse, String> {
    let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
        .await
        .map_err(|e| e.to_string())?;
    let pool = db.get_pool();

    let table_uuids = request
        .table_ids
        .as_ref()
        .map(|ids| {
            ids.into_iter()
                .map(|id| Uuid::parse_str(&id).map_err(|e| e.to_string()))
                .collect::<Result<Vec<Uuid>, String>>()
        })
        .transpose()?;

    match table_uuids {
        Some(table_ids) => {
            let result = manual_reservation(pool, request.clone(), table_ids).await?;
            Ok(ReservationResponse {
                reservation: Some(result),
                is_waitlisted: false,
                message: "Reservation created successfully".to_string(),
            })
        }
        None => auto_reservation(pool, request).await,
    }
}

async fn manual_reservation(
    pool: &PgPool,
    request: ReservationRequest,
    table_ids: Vec<Uuid>,
) -> Result<Reservation, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let count = sqlx::query!(
        r#"
        SELECT COUNT(*) as count 
        FROM reservation_tables rt
        JOIN reservations r ON rt.reservation_id = r.id
        WHERE rt.table_id = ANY($1) 
        AND (
            r.time_slot BETWEEN $2::timestamp 
            AND ($2::timestamp + (r.time_limit || ' minutes')::interval)
            OR
            ($2::timestamp + ($3 || ' minutes')::interval) BETWEEN r.time_slot 
            AND (r.time_slot + (r.time_limit || ' minutes')::interval)
        )
        "#,
        &table_ids,
        request.time_slot,
        request.time_limit as i32
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?
    .count
    .unwrap_or(0);

    if count > 0 {
        return Err("Tables are already reserved for the selected time slot.".to_string());
    }

    let reservation = sqlx::query_as!(
        Reservation,
        r#"
        INSERT INTO reservations (user_id, address, customer_name, people_count, time_slot, time_limit)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, address, customer_name, people_count, time_slot, created_at, time_limit
        "#,
        request.user_id,
        request.address,
        request.customer_name,
        request.people_count,
        request.time_slot,
        request.time_limit
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    for table_id in table_ids {
        sqlx::query!(
            r#"
            INSERT INTO reservation_tables (reservation_id, table_id)
            VALUES ($1, $2)
            "#,
            reservation.id,
            table_id,
        )
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(reservation)
}

async fn auto_reservation(
    pool: &PgPool,
    request: ReservationRequest,
) -> Result<ReservationResponse, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let tables = sqlx::query!(
        r#"
        SELECT t.id, t.capacity
        FROM tables t
        WHERE t.id NOT IN (
            SELECT rt.table_id
            FROM reservation_tables rt
            JOIN reservations r ON rt.reservation_id = r.id
            WHERE r.address = $1 
            AND (
                r.time_slot BETWEEN 
                    $2::timestamp 
                    AND ($2::timestamp + make_interval(mins => $3::int))
                OR
                ($2::timestamp + make_interval(mins => $3::int)) BETWEEN 
                    r.time_slot 
                    AND (r.time_slot + make_interval(mins => r.time_limit))
            )
        )
        ORDER BY t.capacity DESC
        "#,
        request.address,
        request.time_slot,
        request.time_limit as i32
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    let mut total_capacity = 0;
    let mut selected_tables = Vec::new();

    for table in &tables {
        if total_capacity < request.people_count {
            selected_tables.push(table.id);
            total_capacity += table.capacity;
        } else {
            break;
        }
    }

    if total_capacity < request.people_count {
        let waiting_entry = WaitingList::create(
            pool,
            request.user_id,
            &request.address,
            &request.customer_name,
            request.people_count,
            request.time_slot,
            request.time_limit,
        )
        .await?;

        tx.commit().await.map_err(|e| e.to_string())?;

        return Ok(ReservationResponse {
            reservation: None,
            is_waitlisted: true,
            message: format!("Added to waiting list. Position #{}", waiting_entry.id),
        });
    }

    let reservation = sqlx::query_as!(
        Reservation,
        r#"
        INSERT INTO reservations (user_id, address, customer_name, people_count, time_slot, time_limit)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, address, customer_name, people_count, time_slot, created_at, time_limit
       "#,
        request.user_id,
        request.address,
        request.customer_name,
        request.people_count,
        request.time_slot,
        request.time_limit
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

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(ReservationResponse {
        reservation: Some(reservation),
        is_waitlisted: false,
        message: "Reservation created successfully".to_string(),
    })
}
