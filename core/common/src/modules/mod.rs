use async_trait::async_trait;
use axum::Router;

use crate::config::Config;
use crate::error::Result;

#[async_trait]
pub trait Module: Send + Sync {
    async fn create(config: &Config) -> Result<Self>
    where
        Self: Sized;

    fn routes(&self) -> Router;
}
