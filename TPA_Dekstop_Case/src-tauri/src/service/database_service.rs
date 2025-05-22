use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Database { pool })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
