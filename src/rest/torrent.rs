use crate::DOMAIN;
use crate::auth::KEY;
use actix_web::{HttpRequest, HttpResponse, get, web};
use serde_json::Value;
use wreq::{Client, ClientBuilder};

#[get("/torrent/{id:[0-9]+}")]
pub async fn download_torrent(
    data: web::Data<Client>,
    req_data: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let id = req_data.match_info().get("id").unwrap();
    let id = id.parse::<usize>()?;

    let client = ClientBuilder::new().build()?;
    let url = format!("https://yggapi.eu/torrent/{}", id);
    let response = client
        .get(&url)
        .header("Accept", "application/x-bittorrent")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download torrent: {}", response.status()).into());
    }

    let body: Value = response.json().await?;

    let hash = body
        .get("hash")
        .and_then(|h| h.as_str())
        .ok_or("Hash not found in torrent response")?;
    let announce = KEY.get().ok_or("API key not set")?;

    // Convert hex hash to bytes, then encode as base32
    let hash_bytes = hex::decode(hash)?;
    let hash_base32 = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &hash_bytes);

    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    // https://www.yggtorrent.org/engine/get_nfo?torrent=1405878

    let url = format!("https://{}/engine/get_nfo?torrent={}", domain, id);
    let response = data.get(&url).send().await?;

    let mut title = String::new();
    if response.status().is_success() {
        let nfo_text = response.text().await?;
        for line in nfo_text.lines() {
            if line.to_lowercase().starts_with("complete name") {
                if let Some(pos) = line.find(':') {
                    title = line[pos + 1..].trim().to_string();
                    break;
                }
            }
        }
    }

    let tracker = format!(
        "http%3A%2F%2Ftracker.p2p-world.net%3A8080%2F{}%2Fannounce",
        announce
    );

    let magnet_link = format!(
        "magnet:?xt=urn:btih:{}&dn={}&tr={}",
        hash_base32,
        urlencoding::encode(title.as_str()),
        tracker
    );

    debug!("Magnet link: {}", magnet_link);

    Ok(HttpResponse::Found()
        .append_header(("Location", magnet_link))
        .finish())
}
