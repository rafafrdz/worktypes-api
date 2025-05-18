use async_trait::async_trait;
use chrono::{DateTime, Utc};
use common::error::AppError;
use common::error::Result;
use common::repositories::postgres_orm::ORMPostgresRepository;

use crate::models::{Company, CompanyRequest};

use super::repository::CompanyRepositoryTrait;

pub static QUERY: &str = "
            CREATE TABLE IF NOT EXISTS company (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                cif_number TEXT UNIQUE,
                billing_address TEXT,
                postal_code INTEGER,
                city TEXT,
                province TEXT,
                industry TEXT,
                industry_sub_category TEXT,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL
            )
            ";

#[async_trait]
impl CompanyRepositoryTrait for ORMPostgresRepository {
    async fn list(&self, name_filter: Option<String>) -> Result<Vec<Company>> {
        // TODO: Implement logic to list companies from the database
        Err(AppError::NotImplemented)
    }

    async fn get(&self, id: &str) -> Result<Option<Company>> {
        // TODO: Implement logic to get a company by id from the database
        Err(AppError::NotImplemented)
    }

    async fn create(&self, req: CompanyRequest) -> Result<Company> {
        // TODO: Implement logic to create a new company in the database
        Err(AppError::NotImplemented)
    }

    async fn update(&self, id: &str, req: CompanyRequest) -> Result<Option<Company>> {
        // TODO: Implement logic to update a company in the database
        Err(AppError::NotImplemented)
    }

    async fn duplicate(&self, id: &str) -> Result<Option<Company>> {
        // TODO: Implement logic to duplicate a company in the database
        Err(AppError::NotImplemented)
    }
}

// Estructura para mapear los resultados de la base de datos
struct DbCompany {
    id: String,
    name: String,
    cif_number: Option<String>,
    billing_address: Option<String>,
    postal_code: Option<i32>,
    city: Option<String>,
    province: Option<String>,
    industry: Option<String>,
    industry_sub_category: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<DbCompany> for Company {
    fn from(db_company: DbCompany) -> Self {
        Self {
            id: db_company.id,
            name: db_company.name,
            cif_number: db_company.cif_number,
            billing_address: db_company.billing_address,
            postal_code: db_company.postal_code,
            city: db_company.city,
            province: db_company.province,
            industry: db_company.industry,
            industry_sub_category: db_company.industry_sub_category,
            created_at: db_company.created_at,
            updated_at: db_company.updated_at,
        }
    }
}
