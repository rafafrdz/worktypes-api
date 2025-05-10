use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
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
            cif_number: None, // Generamos un nuevo CIF en el repositorio
            billing_address: self.billing_address.clone(),
            postal_code: self.postal_code,
            city: self.city.clone(),
            province: self.province.clone(),
            industry: self.industry.clone(),
            industry_sub_category: self.industry_sub_category.clone(),
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
}

#[derive(Debug, Deserialize)]
pub struct CompanyQuery {
    pub name: Option<String>,
}
