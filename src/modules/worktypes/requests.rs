use super::models::WorkAttributeType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateWorkType {
    pub title: String,
    pub description: Option<String>,
    pub attributes: Vec<WorkAttributeType>,
}
