use std::sync::Arc;

use crate::{repositories::repository::WorkTypeRepositoryTrait, requests::CreateWorkType};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn list_worktypes(
    State(repository): State<Arc<dyn WorkTypeRepositoryTrait + Send + Sync>>,
) -> impl IntoResponse {
    match repository.list().await {
        Ok(companies) => (StatusCode::OK, Json(companies)).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn create_worktype(
    State(repository): State<Arc<dyn WorkTypeRepositoryTrait + Send + Sync>>,
    Json(payload): Json<CreateWorkType>,
) -> impl IntoResponse {
    match repository.create(payload).await {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => e.into_response(),
    }
}
