use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::query::Query;
use sqlx::{query_as, Database};
use tracing::instrument;
use uuid::Uuid;

use crate::models::{DataType, WorkAttributeType, WorkType};
use crate::requests::CreateWorkType;

use super::repository::WorkTypeRepositoryTrait;
use common::error::AppError;
use common::error::Result;
use common::repositories::postgres::PostgresRepository;

pub static QUERY: &str = "
                    CREATE TABLE IF NOT EXISTS WorkType (
                        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                        title VARCHAR(100) NOT NULL,
                        description TEXT,
                        created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
                    );

                    CREATE TABLE IF NOT EXISTS WorkAttributeType (
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
    #[instrument]
    async fn list(&self) -> Result<Vec<WorkType>> {
        tracing::info!("Listing all worktypes");
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

    #[instrument]
    async fn create(&self, request: CreateWorkType) -> Result<WorkType> {
        tracing::info!("Creating the worktype {:?}", request);
        let pool = self.pool.lock().await;
        let mut tx: sqlx::Transaction<'static, sqlx::Postgres> =
            pool.begin().await.map_err(AppError::Database)?;

        let dao = WorkType::from_create_request(request);

        create_work_type_query(&dao)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        for att in &dao.attributes {
            let query = create_work_attribute_type_query(dao.id, att);
            query.execute(&mut *tx).await.map_err(AppError::Database)?;
        }

        tx.commit().await.map_err(AppError::Database)?;
        Ok(dao)
    }
}

pub fn create_work_attribute_type_query(
    work_type_id: Uuid,
    att: &WorkAttributeType,
) -> Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
    sqlx::query(
        r#"
INSERT INTO WorkAttributeType
(id, work_type_id, name, data_type, is_required, is_hidden, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
"#,
    )
    .bind(att.id)
    .bind(work_type_id)
    .bind(&att.name)
    .bind(att.data_type.to_string())
    .bind(att.is_required)
    .bind(att.is_hidden)
    .bind(att.created_at)
    .bind(att.updated_at)
}

pub fn create_work_type_query(
    work_type: &WorkType,
) -> Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
    sqlx::query(
        r#"
INSERT INTO WorkType
(id, title, description, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5)
"#,
    )
    .bind(work_type.id)
    .bind(&work_type.title)
    .bind(&work_type.description)
    .bind(work_type.created_at)
    .bind(work_type.updated_at)
}
