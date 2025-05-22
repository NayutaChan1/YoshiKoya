use crate::models::employee_model::Employee;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatus {
    pub id: i32,
    pub user_id: i32,
    pub role: String,
    pub branch_address: String,
    pub status: String,
    pub applied_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl JobStatus {
    pub async fn apply_job(
        pool: &PgPool,
        user_id: i32,
        role: &str,
        branch_address: &str,
    ) -> Result<JobStatus, String> {
        let has_accepted = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM job_status 
                WHERE user_id = $1 AND status = 'ACCEPTED'
            ) as "exists!"
            "#,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .exists;

        if has_accepted {
            return Err("You already have an accepted job application".to_string());
        }

        sqlx::query_as!(
            JobStatus,
            r#"
            INSERT INTO job_status (user_id, role, branch_address)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            user_id,
            role,
            branch_address
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to submit application: {}", e))
    }

    pub async fn update_status(
        pool: &PgPool,
        user_id: i32,
        new_status: &str,
    ) -> Result<JobStatus, String> {
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        let updated_status = sqlx::query_as!(
            JobStatus,
            r#"
            UPDATE job_status 
            SET status = $1, updated_at = CURRENT_TIMESTAMP
            WHERE user_id = $2
            RETURNING *
            "#,
            new_status,
            user_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| format!("Failed to update application status: {}", e))?;

        if new_status == "ACCEPTED" {
            Employee::create_employee(
                updated_status.user_id,
                &updated_status.role,
                &updated_status.branch_address,
            )
            .await
            .map_err(|e| format!("Failed to create employee record: {}", e))?;
        }

        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        Ok(updated_status)
    }

    pub async fn get_pending_applications(pool: &PgPool) -> Result<Vec<JobStatus>, String> {
        sqlx::query_as!(
            JobStatus,
            r#"
            SELECT * FROM job_status 
            WHERE status = 'PENDING'
            ORDER BY applied_at ASC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch pending applications: {}", e))
    }

    pub async fn get_user_application(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<Option<JobStatus>, String> {
        sqlx::query_as!(
            JobStatus,
            r#"
            SELECT * FROM job_status 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to fetch user application: {}", e))
    }
}
