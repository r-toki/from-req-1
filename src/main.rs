mod config;
mod presentation;

use crate::config::CONFIG;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let host = &CONFIG.host;
    let port = &CONFIG.port;

    HttpServer::new(|| App::new().configure(presentation::init))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
