use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Office {
    pub id: String,
    pub name: String,
    pub address: String,
    pub postal_code: i32,
    pub city: String,
    pub province: String,
    pub size: i32,
    pub target_year: String,
    pub comparative_year: String,
    pub company_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Role,
    pub company_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    Company,
    Admin,
    Certifier,
    God,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Objective {
    pub id: String,
    pub name: String,
    pub utility_type: String,
    pub target: f32,
    pub target_date: String,
    pub company_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: String,
    pub email: String,
    pub role: String,
    pub company_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertificationTemplate {
    pub id: i32,
    pub company_id: String,
    pub name: String,
    pub description: Option<String>,
    pub logo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: String,
    pub user_id: String,
    pub company_id: String,
    pub date: DateTime<Utc>,
    pub url: String,
    pub original_filename: String,
    pub storage_type: DocumentStorageType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DocumentStorageType {
    Local,
    Remote,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Report {
    pub id: String,
    pub company_id: String,
    pub date: DateTime<Utc>,
    pub document_id: String,
    pub format: String,
    pub data_type: String,
    pub selected_tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompanyConfig {
    pub id: String,
    pub company_id: String,
    pub key: String,
    pub label: Option<String>,
    pub visible: bool,
}
