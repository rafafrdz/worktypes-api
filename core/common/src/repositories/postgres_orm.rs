use std::sync::Arc;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::error::{AppError, Result};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct ORMPostgresRepository {
    pub pool: Arc<PgPool>,
}

impl ORMPostgresRepository {
    pub fn new(database_url: &str) -> Result<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .map_err(AppError::Orm)?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}
