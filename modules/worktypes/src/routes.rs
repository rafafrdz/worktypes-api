use std::sync::Arc;

use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{
    handlers::{create_worktype, list_worktypes},
    repositories::repository::WorkTypeRepositoryTrait,
};

pub fn create_routes(repository: Arc<dyn WorkTypeRepositoryTrait + Send + Sync>) -> Router {
    Router::new()
        .route("/worktypes", get(list_worktypes).post(create_worktype))
        .with_state(repository)
}
