pub mod companies;
pub mod worktypes;

use crate::config::Config;

pub struct ModuleRegistry {
    companies_module: companies::CompaniesModule,
}

impl ModuleRegistry {
    pub async fn new(config: &Config) -> Self {
        let companies_module = companies::CompaniesModule::new(config).await;

        Self { companies_module }
    }

    pub fn companies_routes(&self) -> axum::Router {
        self.companies_module.routes()
    }
}
