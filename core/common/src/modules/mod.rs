use async_trait::async_trait;
use axum::Router;

use crate::config::Config;

#[async_trait]
pub trait Module: Send + Sync {
    async fn new(config: &Config) -> Self
    where
        Self: Sized;

    fn routes(&self) -> Router;
}

