use crate::models::DataType;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateWorkType {
    pub title: String,
    pub description: Option<String>,
    pub attributes: Vec<CreateWorkAttributeType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateWorkAttributeType {
    pub name: String,
    pub data_type: DataType,
    pub is_required: bool,
    pub is_hidden: bool,
}
