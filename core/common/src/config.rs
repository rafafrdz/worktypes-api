use dotenvy::{self, dotenv};

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        // dotenv().ok();
        // let c = envy::from_env::<Config>();
        let database_url = std::env::var("DATABASE_URL").expect("Invalid DATABASE_URL variable.");
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        Self { database_url, port }
    }
}
