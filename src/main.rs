mod auth;
mod domain;
mod parser;
mod resolver;
mod rest;
mod search;

use crate::auth::login;
use crate::domain::get_ygg_domain;
use actix_web::{App, HttpServer, web};
use serde_json::Value;
use std::sync::Mutex;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub static DOMAIN: Mutex<String> = Mutex::new(String::new());
pub const LOGIN_PAGE: &str = "/auth/login";
pub const LOGIN_PROCESS_PAGE: &str = "/auth/process_login";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Off)
        .filter_module("ygege", log::LevelFilter::Trace)
        .init();

    // check args (username, password)
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        error!("Usage: {} <username> <password>", args[0]);
        std::process::exit(1);
    }
    let username = &args[1];
    let password = &args[2];

    // get the ygg domain
    let domain = get_ygg_domain().await.unwrap_or_else(|_| {
        error!("Failed to get YGG domain");
        std::process::exit(1);
    });
    let mut domain_lock = DOMAIN.lock().unwrap();
    *domain_lock = domain.clone();
    drop(domain_lock);

    std::fs::create_dir_all("sessions")?;
    let client = login(username, password, true).await?;
    info!("Logged in to YGG with username: {}", username);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(rest::config_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
