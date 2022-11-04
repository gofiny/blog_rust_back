use std::net::Ipv4Addr;
use std::env;


pub struct Config {
    pub rust_log: String,
    pub app_host: Ipv4Addr,
    pub app_port: u16,
    pub database_dsn: String,
}

impl Config {
    pub fn load() -> Self {
        let rust_log = env::var("RUST_LOG").unwrap();
        let app_host: Ipv4Addr = env::var("APP_HOST").unwrap().parse().unwrap();
        let app_port: u16 = env::var("APP_PORT").unwrap().parse().unwrap();
        let database_dsn: String = env::var("DATABASE_DSN").unwrap();

        Self { rust_log, app_host, app_port, database_dsn }
    }
}

