mod auth;
mod domain;
mod parser;
mod resolver;
mod search;

use crate::auth::login;
use crate::domain::get_ygg_domain;
use std::sync::Mutex;
use serde_json::Value;

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

    info!("Searching for `Vaiana 2`");
    let search_result = search::search(&client,Some("Vaiana 2"), None, None, None, None, None).await?;
    if search_result.is_empty() {
        info!("No results found");
    } else {
        info!("Found {} results", search_result.len());
        let best = search_result.iter().max_by_key(|t| t.seed).unwrap();
        info!("Best torrent ({} seeds): {}", best.seed, best.name);
        info!("Dowload link: {}", best.get_url()?);
        let value = best.to_json();
        println!("{}", serde_json::to_string_pretty(&value)?);
    }

    Ok(())
}
