use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use super::{
    models::{CompanyQuery, CompanyRequest},
    repositories::repository::CompanyRepositoryTrait,
};
use crate::error::AppError;

pub async fn list_companies(
    State(repository): State<Arc<dyn CompanyRepositoryTrait + Send + Sync>>,
    Query(query): Query<CompanyQuery>,
) -> impl IntoResponse {
    match repository.list(query.name).await {
        Ok(companies) => (StatusCode::OK, Json(companies)).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn create_company(
    State(repository): State<Arc<dyn CompanyRepositoryTrait + Send + Sync>>,
    Json(payload): Json<CompanyRequest>,
) -> impl IntoResponse {
    match repository.create(payload).await {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn get_company(
    State(repository): State<Arc<dyn CompanyRepositoryTrait + Send + Sync>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match repository.get(&id).await {
        Ok(Some(company)) => (StatusCode::OK, Json(company)).into_response(),
        Ok(None) => {
            AppError::NotFound(format!("Compañía con ID {} no encontrada", id)).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn update_company(
    State(repository): State<Arc<dyn CompanyRepositoryTrait + Send + Sync>>,
    Path(id): Path<String>,
    Json(payload): Json<CompanyRequest>,
) -> impl IntoResponse {
    match repository.update(&id, payload).await {
        Ok(Some(company)) => (StatusCode::OK, Json(company)).into_response(),
        Ok(None) => {
            AppError::NotFound(format!("Compañía con ID {} no encontrada", id)).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn duplicate_company(
    State(repository): State<Arc<dyn CompanyRepositoryTrait + Send + Sync>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match repository.duplicate(&id).await {
        Ok(Some(company)) => (StatusCode::CREATED, Json(company)).into_response(),
        Ok(None) => {
            AppError::NotFound(format!("Compañía con ID {} no encontrada", id)).into_response()
        }
        Err(e) => e.into_response(),
    }
}
