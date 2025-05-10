use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use common::{config::Config, modules::Module, repositories::postgres::PostgresRepository};
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
    async fn new(config: &Config) -> Self {
        let repository = match &config.database_url {
            Some(url) => {
                match PostgresRepository::new_with_ensured_query(url, repositories::postgres::QUERY)
                    .await
                {
                    Ok(repo) => {
                        tracing::info!("Módulo de compañías: Conectado a PostgreSQL");
                        Arc::new(repo) as Arc<dyn WorkTypeRepositoryTrait + Send + Sync>
                    }
                    Err(e) => {
                        tracing::error!(
                            "Módulo de compañías: Error al conectar a PostgreSQL: {}",
                            e
                        );
                        tracing::info!("Módulo de compañías: Usando repositorio en memoria");
                        panic!("Change here to Result<WorktypesModule>")
                        // Arc::new(repositories::memory::MemoryCompanyRepository::new())
                        //     as Arc<dyn WorkTypeRepositoryTrait + Send + Sync>
                    }
                }
            }
            None => {
                tracing::info!("Módulo de compañías: Variable DATABASE_URL no configurada, usando repositorio en memoria");
                panic!("Change here to Result<WorktypesModule>")
                // Arc::new(repositories::memory::MemoryCompanyRepository::new())
                //     as Arc<dyn WorkTypeRepositoryTrait + Send + Sync>
            }
        };

        Self { repository }
    }

    fn routes(&self) -> Router {
        routes::create_routes(self.repository.clone())
    }
}
