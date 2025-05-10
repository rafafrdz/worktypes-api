use axum::{http::Method, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub async fn create_app(router: Router) -> Router {
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

    // Construir la aplicación con las rutas de todos los módulos
    Router::new()
        .merge(router)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
