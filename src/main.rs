use actix_web::{HttpServer, App, web::Data};
use tracing_subscriber::fmt::format::FmtSpan;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Postgres, Pool};

use config::Config;
use controllers::{health_check, users};

mod config;
mod controllers;
mod models;
mod handlers;
mod utils;
mod schemas;


pub struct AppState {
    db_pool: Pool<Postgres>,
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::load();

    // todo: Убрать инициализацию в отдельный метод (внутри отдельного стракта)
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Unable connect to the database");


    tracing_subscriber::fmt()
        .with_env_filter(config.rust_log)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db_pool: db_pool.clone()}))
            .service(health_check::health_check_scope())
            .service(users::users_scope())
    })
    .bind((config.app_host, config.app_port))?
    .run()
    .await
}