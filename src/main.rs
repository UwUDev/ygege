mod categories;
mod config;
mod dbs;
mod nostr;
mod parser;
mod rate_limiter;
pub mod rest;
mod search;

use crate::categories::{CATEGORIES_CACHE, init_categories};
use crate::config::load_config;
use crate::nostr::NostrClient;
use actix_web::{App, HttpServer, web};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

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
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        print_version();
        return Ok(());
    }

    let config = match load_config() {
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

    info!(
        "Ygégé v{} (commit: {}, branch: {}, built: {})",
        VERSION, BUILD_COMMIT, BUILD_BRANCH, BUILD_DATE
    );

    if let Some(tmdb_token) = &config.tmdb_token {
        match dbs::get_account_username(tmdb_token).await {
            Ok(username) => {
                info!("TMDB and IMDB resolver enabled");
                info!("TMDB account username: {}", username);
            }
            Err(e) => {
                error!("Failed to get TMDB account username: {}", e);
            }
        }
    }

    let relay_url = config.relay_url().to_string();
    info!("Using Nostr relay: {}", relay_url);

    let nostr_client = NostrClient::new(&relay_url);

    CATEGORIES_CACHE
        .set(init_categories())
        .map_err(|_| "Failed to set categories cache")?;
    info!(
        "Categories initialized: {} top-level categories",
        CATEGORIES_CACHE.get().unwrap().len()
    );

    let nostr_data = web::Data::new(nostr_client);
    let config_clone = config.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(nostr_data.clone())
            .app_data(web::Data::new(config_clone.clone()))
            .configure(rest::config_routes)
    })
    .bind(format!("{}:{}", config.bind_ip, config.bind_port))?
    .run()
    .await?;

    Ok(())
}
