use crate::service::database_service::Database;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub branch_name: String,
    pub branch_address: String,
    pub opening_time: NaiveTime,
    pub closing_time: NaiveTime,
    #[serde(skip_serializing)]
    pub is_open: bool,
}

impl Branch {
    // pub fn calculate_is_open(&self) -> bool {
    //     let now = chrono::Local::now().naive_local().time();
    //     now >= self.opening_time && now <= self.closing_time
    // }

    pub fn format_time(&self) -> (String, String) {
        (
            self.opening_time.format("%H:%M").to_string(),
            self.closing_time.format("%H:%M").to_string(),
        )
    }
}

impl Branch {
    pub async fn get_branch(branch_address: &str) -> Result<Branch, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let row = sqlx::query!(
            r#"
            SELECT 
                branchname, 
                address, 
                operation_hours_open as "opening_time?",
                operation_hours_close as "closing_time?"
            FROM branches 
            WHERE address = $1
            "#,
            branch_address
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or("Branch not found")?;

        let mut branch = Branch {
            branch_name: row.branchname,
            branch_address: row.address,
            opening_time: row.opening_time.ok_or("Opening time not set")?,
            closing_time: row.closing_time.ok_or("Closing time not set")?,
            is_open: false,
        };

        // println!("Branch: {:?}", branch);

        // branch.is_open = branch.calculate_is_open();
        Ok(branch)
    }

    pub async fn update_branch_hours(
        branch_address: &str,
        new_opening: NaiveTime,
        new_closing: NaiveTime,
    ) -> Result<Branch, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        if new_opening >= new_closing {
            return Err("Opening time must be before closing time".to_string());
        }

        println!("Updating branch hours for: {}", branch_address);
        println!("New opening time: {}", new_opening);
        println!("New closing time: {}", new_closing);

        sqlx::query!(
            r#"
            UPDATE branches 
            SET 
                operation_hours_open = $1,
                operation_hours_close = $2
            WHERE address = $3          -- Changed to match by address instead of branchname
            RETURNING branchname, address, operation_hours_open as "opening_time!", operation_hours_close as "closing_time!"
            "#,
            new_opening,
            new_closing,
            branch_address
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Update failed: {}", e))
        .and_then(|row| {
            Ok(Branch {
                branch_name: row.branchname,
                branch_address: row.address,
                opening_time: row.opening_time,
                closing_time: row.closing_time,
                is_open: false,
            })
        })
    }

    pub async fn get_all_branches() -> Result<Vec<Branch>, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let rows = sqlx::query!(
            r#"
            SELECT 
                branchname,
                address,
                operation_hours_open as "opening_time?",
                operation_hours_close as "closing_time?"
            FROM branches
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch menus: {}", e))?;

        let branches= rows
            .into_iter()
            .map(|row: _| -> Result<Branch, String> {
                Ok(Branch {
                    branch_name: row.branchname,
                    branch_address: row.address,
                    opening_time: row.opening_time.ok_or("Opening time not set")?,
                    closing_time: row.closing_time.ok_or("Closing time not set")?,
                    is_open: false,
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to fetch menus: {}", e))?;

        Ok(branches)
    }

    pub async fn create_branch(
        branch_name: String,
        branch_address: String,
        opening_time: NaiveTime,
        closing_time: NaiveTime,
    ) -> Result<Branch, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        if opening_time >= closing_time {
            return Err("Opening time must be before closing time".to_string());
        }

        let exists = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM branches WHERE address = $1) as exists",
            branch_address
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .exists
        .unwrap_or(false);

        if exists {
            return Err("Branch with this address already exists".to_string());
        }

        let row = sqlx::query!(
            r#"
            INSERT INTO branches (branchname, address, operation_hours_open, operation_hours_close)
            VALUES ($1, $2, $3, $4)
            RETURNING 
                branchname,
                address,
                operation_hours_open as "opening_time!",
                operation_hours_close as "closing_time!"
            "#,
            branch_name,
            branch_address,
            opening_time,
            closing_time
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to create branch: {}", e))?;

        Ok(Branch {
            branch_name: row.branchname,
            branch_address: row.address,
            opening_time: row.opening_time,
            closing_time: row.closing_time,
            is_open: false,
        })
    }


    pub async fn close_branch(branch_address: &str) -> Result<(), String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();
    
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    
        sqlx::query!(
            r#"
            DELETE FROM reservation_tables 
            WHERE reservation_id IN (
                SELECT id FROM reservations WHERE address = $1
            )
            "#,
            branch_address
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to delete reservation tables: {}", e))?;
    
        sqlx::query!(
            "DELETE FROM reservations WHERE address = $1",
            branch_address
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to delete reservations: {}", e))?;
    
        sqlx::query!(
            "DELETE FROM waiting_list WHERE address = $1",
            branch_address
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to delete waiting list entries: {}", e))?;
    
        sqlx::query!(
            "DELETE FROM branches WHERE address = $1",
            branch_address
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to delete branch: {}", e))?;
    
        tx.commit().await.map_err(|e| format!("Failed to commit transaction: {}", e))?;
    
        Ok(())
    }


}
