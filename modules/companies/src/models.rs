use chrono::{DateTime, Utc};
use offices::models::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub cif_number: Option<String>,
    pub billing_address: Option<String>,
    pub postal_code: Option<i32>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub industry: Option<String>,
    pub industry_sub_category: Option<String>,
    pub offices: Vec<Office>,
    pub users: Vec<User>,
    pub objectives: Vec<Objective>,
    pub employees: Vec<Employee>,
    pub certificates: Vec<CertificationTemplate>,
    pub documents: Vec<Document>,
    pub reports: Vec<Report>,
    pub company_config: Vec<CompanyConfig>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Company {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            cif_number: None,
            billing_address: None,
            postal_code: None,
            city: None,
            province: None,
            industry: None,
            industry_sub_category: None,
            offices: Vec::new(),
            users: Vec::new(),
            objectives: Vec::new(),
            employees: Vec::new(),
            certificates: Vec::new(),
            documents: Vec::new(),
            reports: Vec::new(),
            company_config: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    pub fn duplicate(&self) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: format!("{} (copia)", self.name),
            cif_number: None, // nuevo CIF generado externamente
            billing_address: self.billing_address.clone(),
            postal_code: self.postal_code,
            city: self.city.clone(),
            province: self.province.clone(),
            industry: self.industry.clone(),
            industry_sub_category: self.industry_sub_category.clone(),
            offices: Vec::new(),
            users: Vec::new(),
            objectives: Vec::new(),
            employees: Vec::new(),
            certificates: Vec::new(),
            documents: Vec::new(),
            reports: Vec::new(),
            company_config: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct CompanyRequest {
    pub name: String,
    pub cif_number: Option<String>,
    pub billing_address: Option<String>,
    pub postal_code: Option<i32>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub industry: Option<String>,
    pub industry_sub_category: Option<String>,
    pub offices: Vec<Office>,
    pub users: Vec<User>,
    pub objectives: Vec<Objective>,
    pub employees: Vec<Employee>,
    pub certificates: Vec<CertificationTemplate>,
    pub documents: Vec<Document>,
    pub reports: Vec<Report>,
    pub company_config: Vec<CompanyConfig>,
}

#[derive(Debug, Deserialize)]
pub struct CompanyQuery {
    pub name: Option<String>,
}

impl CompanyRequest {
    pub fn to_company(self) -> Company {
        let now = Utc::now();
        Company {
            id: Uuid::new_v4().to_string(),
            name: self.name,
            cif_number: self.cif_number,
            billing_address: self.billing_address,
            postal_code: self.postal_code,
            city: self.city,
            province: self.province,
            industry: self.industry,
            industry_sub_category: self.industry_sub_category,
            offices: self.offices,
            users: self.users,
            objectives: self.objectives,
            employees: self.employees,
            certificates: self.certificates,
            documents: self.documents,
            reports: self.reports,
            company_config: self.company_config,
            created_at: now,
            updated_at: now,
        }
    }
}
