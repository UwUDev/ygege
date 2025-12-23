mod auth;
mod categories;
mod config;
mod dbs;
mod domain;
mod parser;
mod rate_limiter;
pub mod resolver;
mod rest;
mod search;
mod user;
mod utils;

use crate::auth::login;
use crate::categories::init_categories;
use crate::config::load_config;
use crate::domain::{OWN_IP, get_own_ip, get_ygg_domain};
use actix_web::{App, HttpServer, web};
use std::sync::Mutex;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub const LEAKED_IP: &str = "89.42.231.91";

pub static DOMAIN: Mutex<String> = Mutex::new(String::new());
pub const LOGIN_PAGE: &str = "/auth/login";
pub const LOGIN_PROCESS_PAGE: &str = "/auth/process_login";

// Build information from environment variables
const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_COMMIT: &str = match option_env!("BUILD_COMMIT") {
    Some(commit) => commit,
    None => "unknown",
};
const BUILD_DATE: &str = match option_env!("BUILD_DATE") {
    Some(date) => date,
    None => "unknown",
};
const BUILD_BRANCH: &str = match option_env!("BUILD_BRANCH") {
    Some(branch) => branch,
    None => "unknown",
};

fn print_version() {
    println!("Ygégé v{}", VERSION);
    println!("Commit: {}", BUILD_COMMIT);
    println!("Build Date: {}", BUILD_DATE);
    println!("Branch: {}", BUILD_BRANCH);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for --version flag
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        print_version();
        return Ok(());
    }

    let mut config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Off)
        .filter_module("ygege", config.log_level)
        .init();

    // Display version information
    info!(
        "Ygégé v{} (commit: {}, branch: {}, built: {})",
        VERSION, BUILD_COMMIT, BUILD_BRANCH, BUILD_DATE
    );

    let own_ip = get_own_ip().await?;
    info!(
        "Detected own IP address: {}...",
        own_ip.get(0..6).unwrap_or("N/A")
    );
    OWN_IP.set(own_ip)?;

    if let Some(tmdb_token) = &config.tmdb_token {
        match dbs::get_account_username(tmdb_token).await {
            Ok(username) => {
                info!("TMDB and IMDB resolver enabled");
                info!("TMDB account username: {}", username);
            }
            Err(e) => {
                error!("Failed to get TMDB account username: {}", e);
                config.tmdb_token = None;
            }
        }
    }

    // get the ygg domain
    let domain = match &config.ygg_domain {
        Some(d) => {
            info!("Using configured YGG domain: {}", d);
            d.clone()
        }
        None => {
            let d = get_ygg_domain().await;
            match d {
                Ok(domain) => {
                    info!("Using detected YGG domain: {}", domain);
                    domain
                }
                Err(e) => {
                    error!("Failed to get YGG domain: {}", e);
                    std::process::exit(1);
                }
            }
        }
    };
    let mut domain_lock = DOMAIN.lock().unwrap();
    *domain_lock = domain.clone();
    drop(domain_lock);

    std::fs::create_dir_all("sessions")?;
    let client = login(config.username.as_str(), config.password.as_str(), true).await?;
    info!("Logged in to YGG with username: {}", config.username);

    // Initialize categories cache
    if let Err(e) = init_categories(&client).await {
        warn!("Failed to initialize categories cache: {}", e);
    } else {
        let categories = search::CATEGORIES_CACHE.get().unwrap().len();
        let sub_categories: usize = search::CATEGORIES_CACHE
            .get()
            .unwrap()
            .iter()
            .map(|cat| cat.sub_categories.len())
            .sum();
        info!(
            "Categories cache initialized: {} categories, {} sub-categories",
            categories, sub_categories
        );
    }

    let config_clone = config.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(config_clone.clone()))
            .configure(rest::config_routes)
    })
    .bind(format!("{}:{}", config.bind_ip, config.bind_port))?
    .run()
    .await?;

    Ok(())
}
