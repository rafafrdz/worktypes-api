use axum::Router;
use common::modules::Module;
use common::{config::Config, server::create_app};
use companies::CompaniesModule;
use tokio::net::TcpListener;
use worktypes::WorktypesModule;

pub struct AppModules {
    pub companies: CompaniesModule,
    // pub worktypes: WorktypesModule,
    // more modules here:
    // pub new_module: NewModule,
}

impl AppModules {
    pub async fn init(config: &Config) -> Self {
        let c = config.database_url.clone();
        tracing::info!(c);
        let companies: CompaniesModule = CompaniesModule::create(config).await.unwrap();
        // let worktypes: WorktypesModule = WorktypesModule::create(config).await.unwrap();
        // more modules here:
        // let new_module = NewModule::create(config).await.unwrap();

        Self {
            companies,
            // worktypes,
            // more modules here:
            // new_module
        }
    }

    pub fn combined_routes(&self) -> Router {
        let routes: Vec<Router> = vec![
            self.companies.routes(),
            // self.worktypes.routes(),
            // more routes here:
            // self.new_module.routes(),
        ];
        routes
            .into_iter()
            .reduce(|acc, router| acc.merge(router))
            .unwrap_or_else(Router::new)
    }
}

pub async fn create_routes(config: &Config) -> Router {
    AppModules::init(config).await.combined_routes()
}
pub async fn run() {
    // Inicializar el logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Cargar configuración
    let config: Config = Config::from_env();
    let router: Router = create_routes(&config).await;
    // Crear la aplicación
    let app = create_app(router).await;

    // Configurar la dirección del servidor
    let raw_addr = format!("127.0.0.1:{}", config.port);
    let addr = TcpListener::bind(raw_addr.clone()).await.unwrap();
    tracing::info!("Servidor escuchando en {}", raw_addr);

    // Iniciar el servidor
    axum::serve(addr, app).await.unwrap();
}
