mod config;
mod error;
mod modules;
mod server;

use crate::config::Config;
use crate::server::create_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Inicializar el logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Cargar configuración
    let config = Config::from_env();

    // Crear la aplicación
    let app = create_app(&config).await;

    // Configurar la dirección del servidor
    let raw_addr = format!("127.0.0.1:{}", config.port);
    let addr = TcpListener::bind(raw_addr.clone()).await.unwrap();
    tracing::info!("Servidor escuchando en {}", raw_addr);

    // Iniciar el servidor
    axum::serve(addr, app).await.unwrap();
}
