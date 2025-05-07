use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use uuid::Uuid;

use super::repository::CompanyRepositoryTrait;
use crate::{
    error::{AppError, Result},
    modules::companies::models::{Company, CompanyRequest},
};

pub struct MemoryCompanyRepository {
    companies: RwLock<HashMap<String, Company>>,
}

impl MemoryCompanyRepository {
    pub fn new() -> Self {
        Self {
            companies: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl CompanyRepositoryTrait for MemoryCompanyRepository {
    async fn list(&self, name_filter: Option<String>) -> Result<Vec<Company>> {
        let companies = self.companies.read().unwrap();

        let result = match name_filter {
            Some(name) => companies
                .values()
                .filter(|company| company.name.to_lowercase().contains(&name.to_lowercase()))
                .cloned()
                .collect(),
            None => companies.values().cloned().collect(),
        };

        Ok(result)
    }

    async fn get(&self, id: &str) -> Result<Option<Company>> {
        let companies = self.companies.read().unwrap();
        Ok(companies.get(id).cloned())
    }

    async fn create(&self, company_req: CompanyRequest) -> Result<Company> {
        let mut companies = self.companies.write().unwrap();

        let mut company = Company::new(company_req.name);
        company.cif_number = company_req.cif_number.or_else(|| {
            Some(format!(
                "CIF-{}",
                Uuid::new_v4().to_string().split('-').next().unwrap()
            ))
        });
        company.billing_address = company_req.billing_address;
        company.postal_code = company_req.postal_code;
        company.city = company_req.city;
        company.province = company_req.province;
        company.industry = company_req.industry;
        company.industry_sub_category = company_req.industry_sub_category;

        let company_clone = company.clone();
        companies.insert(company.id.clone(), company);
        Ok(company_clone)
    }

    async fn update(&self, id: &str, company_req: CompanyRequest) -> Result<Option<Company>> {
        let mut companies = self.companies.write().unwrap();

        if let Some(company) = companies.get_mut(id) {
            company.name = company_req.name;
            if let Some(cif) = company_req.cif_number {
                company.cif_number = Some(cif);
            }
            if let Some(address) = company_req.billing_address {
                company.billing_address = Some(address);
            }
            if let Some(postal) = company_req.postal_code {
                company.postal_code = Some(postal);
            }
            if let Some(city) = company_req.city {
                company.city = Some(city);
            }
            if let Some(province) = company_req.province {
                company.province = Some(province);
            }
            if let Some(industry) = company_req.industry {
                company.industry = Some(industry);
            }
            if let Some(sub) = company_req.industry_sub_category {
                company.industry_sub_category = Some(sub);
            }
            company.updated_at = chrono::Utc::now();

            return Ok(Some(company.clone()));
        }

        Ok(None)
    }

    async fn duplicate(&self, id: &str) -> Result<Option<Company>> {
        let companies_read = self.companies.read().unwrap();

        if let Some(company) = companies_read.get(id) {
            let mut duplicated = company.duplicate();
            duplicated.cif_number = Some(format!(
                "CIF-{}",
                Uuid::new_v4().to_string().split('-').next().unwrap()
            ));

            drop(companies_read);

            let mut companies_write = self.companies.write().unwrap();
            let duplicated_clone = duplicated.clone();
            companies_write.insert(duplicated.id.clone(), duplicated);

            return Ok(Some(duplicated_clone));
        }

        Ok(None)
    }
}
