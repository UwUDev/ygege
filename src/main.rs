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
use crate::nostr::{NostrClient, rank_relays};
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

<<<<<<< HEAD
=======
    let outbound_proxy = config.outbound_proxy();

    let http_client = match build_http_client(&config) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to build outbound HTTP client: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(proxy_url) = config.proxy_url.as_deref() {
        info!("Outbound HTTP proxy enabled: {}", proxy_url);
        if config.use_tor {
            info!("Tor routing remains the transport for Nostr relay connections while USE_TOR is enabled");
        } else {
            info!("Nostr relay connections will use the outbound HTTP proxy");
        }
    }

>>>>>>> fd3f5c5 (Enhance proxy configuration support for Nostr relay connections and update documentation)
    if let Some(tmdb_token) = &config.tmdb_token {
        match dbs::get_account_username(tmdb_token).await {
            Ok(_username) => {
                info!("TMDB and IMDB resolver enabled");
            }
            Err(e) => {
                error!("Failed to get TMDB account username: {}", e);
            }
        }
    }

    if config.use_tor {
        info!(
            "Tor routing enabled (proxy: {})",
            config.tor_proxy.as_deref().unwrap_or("127.0.0.1:9050")
        );
    } else {
        info!("Tor routing disabled — connecting to relays directly");
    }

    info!("Ranking Nostr relays by latency...");
    let ranked_relays = rank_relays(
        config.use_tor,
        config.tor_proxy.as_deref(),
        outbound_proxy.as_ref(),
    )
    .await;
    if ranked_relays.is_empty() {
        error!(
            "No Nostr relays are reachable, try again later or check your network connection. Exiting."
        );
        std::process::exit(1);
    }
    info!(
        "Relay order: {}",
        ranked_relays
            .iter()
            .enumerate()
            .map(|(i, url)| format!("{}. {}", i + 1, url))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let nostr_client = NostrClient::new(
        ranked_relays,
        config.use_tor,
        config.tor_proxy.clone(),
        outbound_proxy,
    );

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
