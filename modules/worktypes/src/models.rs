use std::{fmt, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use common::error::AppError;
// Aqui definimos los modelos para los tipos de entidades de trabajo
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkType {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub attributes: Vec<WorkAttributeType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkAttributeType {
    pub id: Uuid,
    pub name: String,
    pub data_type: DataType,
    pub is_required: bool,
    pub is_hidden: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
// Aqui definimos los modelos para los las implementaciones de las entidades de trabajo
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkAttributeItem {
    pub id: Uuid,
    pub attribute_type: WorkAttributeType,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItem {
    pub id: Uuid,
    pub work_type: WorkType,
    pub work_attributes: Vec<WorkAttributeType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataType {
    StringType,
    NumericType,
}

impl FromStr for DataType {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "string" => Ok(DataType::StringType),
            "numeric" => Ok(DataType::NumericType),
            other => Err(AppError::Validation(format!(
                "unknown data type: {}",
                other
            ))),
        }
    }
}
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x: &'static str = match self {
            DataType::StringType => "string",
            DataType::NumericType => "numeric",
        };
        write!(f, "{}", x)
    }
}

impl Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "string" => Ok(DataType::StringType),
            "numeric" => Ok(DataType::NumericType),
            other => Err(serde::de::Error::custom(format!(
                "unknown data type: {}",
                other
            ))),
        }
    }
}

impl WorkType {
    pub fn default(title: String) -> Self {
        let now = Utc::now();
        let summary: WorkAttributeType = WorkAttributeType::summary();
        let description: WorkAttributeType = WorkAttributeType::description();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            attributes: vec![summary, description],
            created_at: now,
            updated_at: now,
        }
    }
}

impl WorkAttributeType {
    pub fn new(name: String, data_type: DataType, is_required: bool, is_hidden: bool) -> Self {
        let now = Utc::now();
        WorkAttributeType {
            id: Uuid::new_v4(),
            name,
            data_type,
            is_required,
            is_hidden,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_name_data_type(name: String, data_type: DataType) -> Self {
        WorkAttributeType::new(name, data_type, false, false)
    }

    #[must_use]
    pub fn summary() -> Self {
        Self::new("Summary".to_string(), DataType::StringType, true, false)
    }

    #[must_use]
    pub fn description() -> Self {
        Self::new("Description".to_string(), DataType::StringType, true, false)
    }
}
