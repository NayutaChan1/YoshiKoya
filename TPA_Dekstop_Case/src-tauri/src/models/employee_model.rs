use crate::service::database_service::Database;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Employee {
    pub user_id: i32,
    pub job: String,
    pub employee_code: String,
    pub address: Option<String>,
    pub level: Option<String>,
}


impl Employee {

    pub async fn get_employee(user_id: i32) -> Result<Employee, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        sqlx::query_as!(
            Employee,
            "SELECT user_id, job, employee_code, address, level FROM employee WHERE user_id = $1",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    fn generate_employee_code() -> String {
        let mut rng = thread_rng();
        
        let letters: String = (0..2)
            .map(|_| rng.sample(Alphanumeric) as char)
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| c.to_ascii_uppercase())
            .collect();
            
        let numbers: String = (0..2)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect();
            
        let last_number = rng.gen_range(0..10);
        
        format!("{}{}-{}", letters, numbers, last_number)
    }

    pub async fn reassign_employee(
        user_id: i32,
        new_job: String,
    ) -> Result<Employee, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();


        let new_level = Self::determine_level(&new_job.as_str());

        let updated_employee = sqlx::query_as!(
            Employee,
            r#"
            UPDATE employee 
            SET job = $1, level = $2
            WHERE user_id = $3
            RETURNING user_id, job, employee_code, address, level
            "#,
            new_job,
            new_level,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to reassign employee: {}", e))?;

        Ok(updated_employee)
    }

    fn determine_level(job: &str) -> String {
        match job {
            "Cashier" | "Chef" | "Waiter" | "Delivery Personnel" | "Supplier" => 
                "Restaurant".to_string(),
            "Branch Manager" | "Branch Marketing Staff" | "Branch HR" | "Branch Operation Staff" => 
                "Branch".to_string(),
            "Corporate HR" | "General Manager" => 
                "Corporate".to_string(),
            _ => "Undefined".to_string()
        }
    }

    pub async fn create_employee(
        user_id: i32,
        job: &str,
        address: &str,
    ) -> Result<Employee, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let employee_code = Self::generate_employee_code();
        let level = Self::determine_level(job);

        sqlx::query_as!(
            Employee,
            r#"
            INSERT INTO employee (user_id, job, employee_code, address, level)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING user_id, job, employee_code, address, level
            "#,
            user_id,
            job,
            employee_code,
            address,
            level
        )
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to create employee: {}", e))
    }

    pub async fn get_all_employees() -> Result<Vec<Employee>, String> {
        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya")
            .await
            .map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        let moses =  sqlx::query_as!(
            Employee,
            "SELECT user_id, job, employee_code, address, level FROM employee"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(moses)

    }

    pub async fn find_job_by_user_id(user_id: i32) -> Result<String, String> {

        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya").await.map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        sqlx::query_scalar!(
            "SELECT job FROM employee WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("User tidak punya pekerjaan".to_string())
    }

    pub async fn find_employee_code(user_id: i32) -> Result<String, String> {

        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya").await.map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        sqlx::query_scalar!(
            "SELECT employee_code FROM employee WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("User tidak punya employee code".to_string())
    }

    pub async fn find_address(user_id: i32) -> Result<String, String> {

        let db = Database::new("postgres://postgres:12345678@localhost:5432/YoshiKoya").await.map_err(|e| e.to_string())?;
        let pool = db.get_pool();

        sqlx::query_scalar!(
            "SELECT address FROM employee WHERE user_id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?
        .flatten()
        .ok_or("User tidak punya address".to_string())
    }

}