use std::env;
use std::time::Duration;
use actix_web::{HttpServer, App};
use tracing_subscriber::fmt::format::FmtSpan;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use config::Config;
use controllers::health_check;

mod config;
mod controllers;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::load();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_dsn)
        .await
        .expect("Unable connect to the database");


    tracing_subscriber::fmt()
        .with_env_filter(config.rust_log)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    HttpServer::new(|| {
        App::new().service(health_check::health_check_scope(),)
    })
    .bind((config.app_host, config.app_port))?
    .run()
    .await
}