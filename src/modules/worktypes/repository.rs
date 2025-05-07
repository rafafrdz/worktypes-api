use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::models::{DataType, WorkAttributeType, WorkType};
use crate::error::{AppError, Result};
use crate::repositories::postgres::PostgresRepository;

#[async_trait]
pub trait WorkTypeRepositoryTrait {
    async fn list(&self) -> Result<Vec<WorkType>>;
    // async fn create(&self, request: CreateWorkType) -> Result<WorkType>;
}

pub static WORKTYPE_QUERY: &str = "
                    CREATE TABLE WorkType (
                        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                        title VARCHAR(100) NOT NULL,
                        description TEXT,
                        created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
                    );

                    CREATE TABLE WorkAttributeType (
                        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                        work_type_id UUID NOT NULL REFERENCES WorkType(id) ON DELETE CASCADE,
                        name VARCHAR(100) NOT NULL,
                        data_type VARCHAR(50) NOT NULL,
                        is_required BOOLEAN NOT NULL DEFAULT FALSE,
                        is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
                        created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
                    );
            ";

#[derive(Debug)]
struct FlatWorkTypeRow {
    work_type_id: Uuid,
    title: String,
    description: Option<String>,
    work_type_created_at: DateTime<Utc>,
    work_type_updated_at: DateTime<Utc>,
    attribute_id: Option<Uuid>,
    attribute_name: Option<String>,
    data_type: Option<String>,
    is_required: Option<bool>,
    is_hidden: Option<bool>,
    attribute_created_at: Option<DateTime<Utc>>,
    attribute_updated_at: Option<DateTime<Utc>>,
}

#[async_trait]
impl WorkTypeRepositoryTrait for PostgresRepository {
    async fn list(&self) -> Result<Vec<WorkType>> {
        let pool = self.pool.lock().await;
        let rows: Vec<FlatWorkTypeRow> = sqlx::query_as!(
            FlatWorkTypeRow,
            r#"
                SELECT
                    wt.id AS work_type_id,
                    wt.title,
                    wt.description,
                    wt.created_at AS work_type_created_at,
                    wt.updated_at AS work_type_updated_at,
                    wat.id AS attribute_id,
                    wat.name AS attribute_name,
                    wat.data_type,
                    wat.is_required,
                    wat.is_hidden,
                    wat.created_at AS attribute_created_at,
                    wat.updated_at AS attribute_updated_at
                FROM WorkType wt
                LEFT JOIN WorkAttributeType wat ON wt.id = wat.work_type_id
                ORDER BY wt.id
    "#
        )
        .fetch_all(&*pool)
        .await?;
        let mut map: HashMap<Uuid, WorkType> = HashMap::new();

        for row in rows {
            let entry: &mut WorkType = map.entry(row.work_type_id).or_insert_with(|| WorkType {
                id: row.work_type_id,
                title: row.title.clone(),
                description: row.description.clone(),
                created_at: row.work_type_created_at,
                updated_at: row.work_type_updated_at,
                attributes: Vec::new(),
            });

            let data_type: Option<DataType> = row
                .data_type
                .as_ref()
                .and_then(|dt| dt.parse::<DataType>().ok());

            if let (Some(attribute_id), Some(dt)) = (row.attribute_id, data_type) {
                entry.attributes.push(WorkAttributeType {
                    id: attribute_id,
                    name: row.attribute_name.unwrap(),
                    data_type: dt,
                    is_required: row.is_required.unwrap(),
                    is_hidden: row.is_hidden.unwrap(),
                    created_at: row.attribute_created_at.unwrap(),
                    updated_at: row.attribute_updated_at.unwrap(),
                });
            }
        }

        let work_types = map.into_values().collect();
        Ok(work_types)
    }
}
