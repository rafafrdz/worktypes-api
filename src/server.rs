use axum::{http::Method, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::modules::ModuleRegistry;

pub async fn create_app(config: &Config) -> Router {
    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
        ]);

    // Inicializar el registro de módulos
    let module_registry = ModuleRegistry::new(config).await;

    // Construir la aplicación con las rutas de todos los módulos
    Router::new()
        .merge(module_registry.companies_routes())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
