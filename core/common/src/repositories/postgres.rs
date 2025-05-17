use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::Mutex;

use crate::error::{AppError, Result};

#[derive(Debug)]
pub struct PostgresRepository {
    pub pool: Arc<Mutex<Pool<Postgres>>>,
}

impl PostgresRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(AppError::Database)?;

        Ok(Self {
            pool: Arc::new(Mutex::new(pool)),
        })
    }

    pub async fn new_with_ensured_query(database_url: &str, sql_query: &str) -> Result<Self> {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(AppError::Database)?;

        // Ejecutar cada sentencia individualmente
        for stmt in sql_query.split(';') {
            let trimmed = stmt.trim();
            if !trimmed.is_empty() {
                tracing::info!(sql = %trimmed, "Ejecutando sentencia SQL");
                sqlx::query(trimmed)
                    .execute(&pool)
                    .await
                    .map_err(AppError::Database)?;
            }
        }

        Ok(Self {
            pool: Arc::new(Mutex::new(pool)),
        })
    }
}
