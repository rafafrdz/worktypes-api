use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use common::{config::Config, modules::Module, repositories::postgres::PostgresRepository};
use common::{error::AppError, error::Result};
use repositories::repository::WorkTypeRepositoryTrait;
use tracing::instrument;

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
    #[instrument]
    async fn create(config: &Config) -> Result<Self> {
        let repo_opt: Result<PostgresRepository> = PostgresRepository::new_with_ensured_query(
            &config.database_url,
            repositories::postgres::QUERY,
        )
        .await;

        repo_opt
            .map(|r| {
                tracing::info!("[Worktype Module] Conectado a PostgreSQL");
                let psql_repo = Arc::new(r) as Arc<dyn WorkTypeRepositoryTrait + Send + Sync>;
                Self {
                    repository: psql_repo,
                }
            })
            .map_err(|e| {
                AppError::Internal(format!(
                    "[Worktype Module] Problem connecting to PostgreSQL. Error: {}",
                    e
                ))
            })
    }

    fn routes(&self) -> Router {
        routes::create_routes(self.repository.clone())
    }
}