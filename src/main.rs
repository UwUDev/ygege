mod auth;
mod config;
mod domain;
mod parser;
mod resolver;
mod rest;
mod search;

use crate::auth::login;
use crate::domain::get_ygg_domain;
use actix_web::{App, HttpServer, web};
use std::sync::Mutex;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub static DOMAIN: Mutex<String> = Mutex::new(String::new());
pub const LOGIN_PAGE: &str = "/auth/login";
pub const LOGIN_PROCESS_PAGE: &str = "/auth/process_login";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match config::load_config() {
        Ok(config) => config,
        Err(e) => {
            eprint!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Off)
        .filter_module("ygege", config.log_level)
        .init();

    // get the ygg domain
    let domain = get_ygg_domain().await.unwrap_or_else(|_| {
        error!("Failed to get YGG domain");
        std::process::exit(1);
    });
    let mut domain_lock = DOMAIN.lock().unwrap();
    *domain_lock = domain.clone();
    drop(domain_lock);

    std::fs::create_dir_all("sessions")?;
    let client = login(config.username.as_str(), config.password.as_str(), true).await?;
    info!("Logged in to YGG with username: {}", config.username);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(rest::config_routes)
    })
    .bind(format!("{}:{}", config.bind_ip, config.bind_port))?
    .run()
    .await?;

    Ok(())
}
