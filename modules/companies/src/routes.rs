use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};

use super::{
    handlers::{create_company, duplicate_company, get_company, list_companies, update_company},
    repositories::repository::CompanyRepositoryTrait,
};

pub fn create_routes(repository: Arc<dyn CompanyRepositoryTrait + Send + Sync>) -> Router {
    Router::new()
        .route("/companies", get(list_companies).post(create_company))
        .route("/companies/{id}", get(get_company).put(update_company))
        .route("/companies/{id}/duplicate", post(duplicate_company))
        .with_state(repository)
}
