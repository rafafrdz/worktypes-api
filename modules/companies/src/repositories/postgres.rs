use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::query_as;
use uuid::Uuid;
use common::{error::{AppError, Result}, repositories::postgres::PostgresRepository};


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
impl CompanyRepositoryTrait for PostgresRepository {
    async fn list(&self, name_filter: Option<String>) -> Result<Vec<Company>> {
        let pool = self.pool.lock().await;

        let companies: Vec<DbCompany> = match name_filter {
            Some(name) => query_as!(
                DbCompany,
                r#"
                    SELECT *
                    FROM Company
                    WHERE name ILIKE $1
                    "#,
                format!("%{}%", name)
            )
            .fetch_all(&*pool)
            .await
            .map_err(AppError::Database)?,
            None => query_as!(
                DbCompany,
                r#"
                    SELECT *
                    FROM Company
                    "#
            )
            .fetch_all(&*pool)
            .await
            .map_err(AppError::Database)?,
        };

        Ok(companies.into_iter().map(|c| c.into()).collect())
    }

    async fn get(&self, id: &str) -> Result<Option<Company>> {
        let pool = self.pool.lock().await;

        let company: Option<DbCompany> = query_as!(
            DbCompany,
            r#"
            SELECT *
            FROM Company
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await
        .map_err(AppError::Database)?;

        Ok(company.map(|c| c.into()))
    }

    async fn create(&self, company_req: CompanyRequest) -> Result<Company> {
        let pool = self.pool.lock().await;
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();

        // Generamos un CIF si no se proporciona
        let cif_number = company_req.cif_number.unwrap_or_else(|| {
            format!(
                "CIF-{}",
                Uuid::new_v4().to_string().split('-').next().unwrap()
            )
        });

        let company: DbCompany = query_as!(
            DbCompany,
            r#"
            INSERT INTO Company (id, name, cif_number, billing_address, postal_code, city, province, industry, industry_sub_category, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, name, cif_number, billing_address, postal_code, city, province, industry, industry_sub_category, created_at, updated_at
            "#,
            id,
            company_req.name,
            cif_number,
            company_req.billing_address,
            company_req.postal_code,
            company_req.city,
            company_req.province,
            company_req.industry,
            company_req.industry_sub_category,
            now,
            now
        )
        .fetch_one(&*pool)
        .await
        .map_err(AppError::Database)?;

        Ok(company.into())
    }

    async fn update(&self, id: &str, company_req: CompanyRequest) -> Result<Option<Company>> {
        let pool = self.pool.lock().await;
        let now = Utc::now();

        // Primero verificamos si la compañía existe
        let existing = sqlx::query!(r#"SELECT id FROM Company WHERE id = $1"#, id)
            .fetch_optional(&*pool)
            .await
            .map_err(AppError::Database)?;

        if existing.is_none() {
            return Ok(None);
        }

        // Obtenemos los valores actuales para no sobrescribir con NULL
        let current: DbCompany = query_as!(
            DbCompany,
            r#"
            SELECT *
            FROM Company
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*pool)
        .await
        .map_err(AppError::Database)?;

        // Actualizamos con los nuevos valores o mantenemos los actuales
        let company: DbCompany = query_as!(
            DbCompany,
            r#"
            UPDATE Company
            SET
                name = $1,
                cif_number = $2,
                billing_address = $3,
                postal_code = $4,
                city = $5,
                province = $6,
                industry = $7,
                industry_sub_category = $8,
                updated_at = $9
            WHERE id = $10
            RETURNING id, name, cif_number, billing_address, postal_code, city, province, industry, industry_sub_category, created_at, updated_at
            "#,
            company_req.name,
            company_req.cif_number.or(current.cif_number),
            company_req.billing_address.or(current.billing_address),
            company_req.postal_code.or(current.postal_code),
            company_req.city.or(current.city),
            company_req.province.or(current.province),
            company_req.industry.or(current.industry),
            company_req.industry_sub_category.or(current.industry_sub_category),
            now,
            id
        )
        .fetch_one(&*pool)
        .await
        .map_err(AppError::Database)?;

        Ok(Some(company.into()))
    }

    async fn duplicate(&self, id: &str) -> Result<Option<Company>> {
        let pool = self.pool.lock().await;

        // Primero obtenemos la compañía original
        let original = query_as!(
            DbCompany,
            r#"
            SELECT *
            FROM Company
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await
        .map_err(AppError::Database)?;

        if let Some(original) = original {
            let now = Utc::now();
            let new_id = Uuid::new_v4().to_string();
            let new_name = format!("{} (copia)", original.name);
            let new_cif = format!(
                "CIF-{}",
                Uuid::new_v4().to_string().split('-').next().unwrap()
            );

            // Creamos la copia
            let company: DbCompany = query_as!(
                DbCompany,
                r#"
                INSERT INTO Company (id, name, cif_number, billing_address, postal_code, city, province, industry, industry_sub_category, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING id, name, cif_number, billing_address, postal_code, city, province, industry, industry_sub_category, created_at, updated_at
                "#,
                new_id,
                new_name,
                new_cif, // Generamos un nuevo CIF ya que debe ser único
                original.billing_address,
                original.postal_code,
                original.city,
                original.province,
                original.industry,
                original.industry_sub_category,
                now,
                now
            )
            .fetch_one(&*pool)
            .await
            .map_err(AppError::Database)?;

            Ok(Some(company.into()))
        } else {
            Ok(None)
        }
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
