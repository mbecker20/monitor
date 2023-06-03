use axum::Router;
use dotenv::dotenv;
use merge_config_files::parse_config_file;
use monitor_types::CoreConfig;
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};

type SpaRouter = Router;

#[derive(Deserialize, Debug)]
pub struct Env {
    #[serde(default = "default_config_path")]
    pub config_path: String,
    #[serde(default = "default_frontend_path")]
    pub frontend_path: String,
    pub port: Option<u16>,
}

pub fn load() -> (CoreConfig, SpaRouter, ServeFile, u16) {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse environment variables");
    let config: CoreConfig =
        parse_config_file(env.config_path.as_str()).expect("failed to parse config");
    let spa_router = Router::new().nest_service(
        "/assets",
        ServeDir::new(&env.frontend_path)
            .not_found_service(ServeFile::new(format!("{}/index.html", env.frontend_path))),
    );
    let index_html_service = ServeFile::new(format!("{}/index.html", env.frontend_path));
    let port = env.port.unwrap_or(config.port);
    (config, spa_router, index_html_service, port)
}

pub fn default_config_path() -> String {
    "/config/config.toml".to_string()
}

fn default_frontend_path() -> String {
    "/frontend".to_string()
}
