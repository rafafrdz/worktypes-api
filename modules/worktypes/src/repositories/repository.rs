use async_trait::async_trait;

use crate::{models::WorkType, requests::CreateWorkType};
use common::error::Result;

#[async_trait]
pub trait WorkTypeRepositoryTrait {
    async fn list(&self) -> Result<Vec<WorkType>>;
    async fn create(&self, request: CreateWorkType) -> Result<WorkType>;
}
