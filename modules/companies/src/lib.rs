mod handlers;
mod models;
mod repositories;
mod routes;
use async_trait::async_trait;
use axum::Router;
use std::sync::Arc;

use common::{config::Config, modules::Module, repositories::postgres::PostgresRepository};
use repositories::repository::CompanyRepositoryTrait;

pub struct CompaniesModule {
    repository: Arc<dyn CompanyRepositoryTrait + Send + Sync>,
}

#[async_trait]
impl Module for CompaniesModule {
    async fn new(config: &Config) -> Self {
        let repository = match &config.database_url {
            Some(url) => match PostgresRepository::new_with_ensured_query(
                url,
                repositories::postgres::COMPANY_QUERY,
            )
            .await
            {
                Ok(repo) => {
                    tracing::info!("Módulo de compañías: Conectado a PostgreSQL");
                    Arc::new(repo) as Arc<dyn CompanyRepositoryTrait + Send + Sync>
                }
                Err(e) => {
                    tracing::error!("Módulo de compañías: Error al conectar a PostgreSQL: {}", e);
                    tracing::info!("Módulo de compañías: Usando repositorio en memoria");
                    Arc::new(repositories::memory::MemoryCompanyRepository::new())
                        as Arc<dyn CompanyRepositoryTrait + Send + Sync>
                }
            },
            None => {
                tracing::info!("Módulo de compañías: Variable DATABASE_URL no configurada, usando repositorio en memoria");
                Arc::new(repositories::memory::MemoryCompanyRepository::new())
                    as Arc<dyn CompanyRepositoryTrait + Send + Sync>
            }
        };

        Self { repository }
    }

    fn routes(&self) -> Router {
        routes::create_routes(self.repository.clone())
    }
}
