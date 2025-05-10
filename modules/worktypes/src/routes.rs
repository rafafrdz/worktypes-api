use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};

use crate::repositories::repository::WorkTypeRepositoryTrait;

pub fn create_routes(repository: Arc<dyn WorkTypeRepositoryTrait + Send + Sync>) -> Router {
    Router::new()
        // Here implement Routes using handler
        .with_state(repository)
}
