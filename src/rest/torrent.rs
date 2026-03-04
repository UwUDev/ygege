use crate::nostr::{NostrClient, magnet_from_event};
use actix_web::{HttpRequest, HttpResponse, get, web};

#[get("/torrent/{id}")]
pub async fn download_torrent(
    nostr: web::Data<NostrClient>,
    req_data: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let id = req_data.match_info().get("id").unwrap_or("");

    let event = nostr
        .get_event(id)
        .await
        .map_err(|e| format!("Relay error: {}", e))?;

    match event.and_then(|e| magnet_from_event(&e)) {
        Some(magnet) => Ok(HttpResponse::Found()
            .insert_header(("Location", magnet))
            .finish()),
        None => Ok(HttpResponse::NotFound().body("Torrent not found")),
    }
}
