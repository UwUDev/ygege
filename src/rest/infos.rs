use crate::config::Config;
use crate::nostr::NostrClient;
use crate::search::{Order, Sort, search};
use actix_web::{HttpResponse, get, web};

#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/status")]
pub async fn status_check(
    nostr: web::Data<NostrClient>,
    config: web::Data<Config>,
) -> HttpResponse {
    let search_result = search(
        &nostr,
        "Vaiana",
        None,
        Some(Sort::Seed),
        Some(Order::Ascending),
        None,
    )
    .await;

    let (search_status, parsing) = match search_result {
        Ok(torrents) => ("ok", if torrents.is_empty() { "empty" } else { "ok" }),
        Err(e) => {
            error!("Status check search error: {}", e);
            ("failed", "n/a")
        }
    };

    let tmdb = match config.tmdb_token.is_some() {
        true => "enabled",
        false => "disabled",
    };

    let first_relay = nostr
        .relays()
        .first()
        .cloned()
        .unwrap_or_else(|| "error".to_string());

    let status = serde_json::json!({
        "relay": first_relay,
        "search": search_status,
        "parsing": parsing,
        "tmdb_integration": tmdb,
    });

    HttpResponse::Ok().json(status)
}
