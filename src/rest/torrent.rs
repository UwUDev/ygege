use crate::DOMAIN;
use crate::config::Config;
use actix_web::{HttpRequest, HttpResponse, get, web};
use rs_torrent_magnet;
use serde_json::Value;
use tokio::time::{Duration, sleep};
use wreq::Client;

#[get("/torrent/{id:[0-9]+}")]
pub async fn download_torrent(
    data: web::Data<Client>,
    config: web::Data<Config>,
    req_data: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let id = req_data.match_info().get("id").unwrap();
    let id = id.parse::<usize>()?;

    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    // Request token
    let url = format!("https://{}/engine/start_download_timer", domain);
    let body = format!("torrent_id={}", id);

    debug!("Request download token {} {}", url, body);

    let response = data
        .post(&url)
        .body(body)
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to get token: {}", response.status()).into());
    }

    let body: Value = response.json().await?;
    debug!("Response {}", body);

    let token = body
        .get("token")
        .and_then(|h| h.as_str())
        .ok_or("Token not found in start_download_timer response")?;

    if !config.turbo_enabled.unwrap_or(false) {
        debug!("Wait 30 secs...");
        sleep(Duration::from_secs(30)).await;
        debug!("Wait is over");
    }

    // Request signed torrent file
    let url = format!(
        "https://{}/engine/download_torrent?id={}&token={}",
        domain, id, token
    );
    debug!("download URL {}", url);

    let response = data.get(&url).send().await?;

    if !response.status().is_success() {
        if response.status() == 302 {
            warn!("Probably limit reached");
        }
        return Err(format!(
            "Failed to get torrent file: {} {}",
            response.status(),
            response.text().await?
        )
        .into());
    }

    let body = response.bytes().await?;

    // Convert torrent to magnet
    let magnet_link = rs_torrent_magnet::magnet_from_torrent(body.to_vec())
        .map_err(|e| format!("Failed to convert torrent to magnet: {:?}", e))?;

    debug!("Magnet link: {}", magnet_link);

    Ok(HttpResponse::Found()
        .append_header(("Location", magnet_link))
        .finish())
}
