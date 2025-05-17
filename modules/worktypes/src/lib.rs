use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use common::{config::Config, modules::Module, repositories::postgres::PostgresRepository};
use common::{error::AppError, error::Result};
use repositories::repository::WorkTypeRepositoryTrait;

mod handlers;
mod models;
mod repositories;
mod requests;
mod routes;

pub struct WorktypesModule {
    repository: Arc<dyn WorkTypeRepositoryTrait + Send + Sync>,
}

#[async_trait]
impl Module for WorktypesModule {
    async fn create(config: &Config) -> Result<Self> {
        match PostgresRepository::new_with_ensured_query(
            &config.database_url,
            repositories::postgres::QUERY,
        )
        .await
        {
            Ok(repo) => {
                tracing::info!("Módulo de Worktypes: Conectado a PostgreSQL");
                let psql_repo = Arc::new(repo) as Arc<dyn WorkTypeRepositoryTrait + Send + Sync>;
                Ok(Self {
                    repository: psql_repo,
                })
            }
            Err(e) => Err(AppError::Internal(format!(
                "Worktype Module: Problem connecting to PostgreSQL. Error: {}",
                e
            ))),
        }
    }

    fn routes(&self) -> Router {
        routes::create_routes(self.repository.clone())
    }
}
