use async_trait::async_trait;
use common::error::Result;

use crate::models::{Company, CompanyRequest};
#[async_trait]
pub trait CompanyRepositoryTrait {
    async fn list(&self, name_filter: Option<String>) -> Result<Vec<Company>>;
    async fn get(&self, id: &str) -> Result<Option<Company>>;
    async fn create(&self, company_req: CompanyRequest) -> Result<Company>;
    async fn update(&self, id: &str, company_req: CompanyRequest) -> Result<Option<Company>>;
    async fn duplicate(&self, id: &str) -> Result<Option<Company>>;
}

// Enum para seleccionar el tipo de repositorio
pub enum RepositoryProvider {
    Memory,
    Postgres(String),
}
