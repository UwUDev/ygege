use crate::DOMAIN;
use crate::config::Config;
use crate::rest::client_extractor::MaybeCustomClient;
use actix_web::{HttpRequest, HttpResponse, get, web};
use tokio::time::{Duration, sleep};

#[get("/torrent/{id:[0-9]+}")]
pub async fn download_torrent(
    data: MaybeCustomClient,
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
    let body_str = format!("torrent_id={}", id);

    log::debug!("Request download token {} {}", url, body_str);

    let response = data.client.post_form(&url, &body_str).await?;

    if response.status < 200 || response.status >= 300 {
        return Err(format!("Failed to get token: {}", response.status).into());
    }

    // EXTRACTION DU TOKEN 100% BLINDÉE (Ignore le HTML/JS de FlareSolverr)
    let body_text = response.body.as_str();
    let token_start = body_text.find("\"token\":\"")
        .map(|i| i + 9)
        .or_else(|| body_text.find("\"token\": \"").map(|i| i + 10))
        .ok_or("Token not found in start_download_timer response")?;

    let rest = &body_text[token_start..];
    let token_end = rest.find('"').ok_or("Malformed token response")?;
    let token = &rest[..token_end];

    if !config.turbo_enabled.unwrap_or(false) {
        log::debug!("Wait 30 secs...");
        sleep(Duration::from_secs(30)).await;
        log::debug!("Wait is over");
    }

    // Request signed torrent file
    let url = format!(
        "https://{}/engine/download_torrent?id={}&token={}",
        domain, id, token
    );
    log::debug!("download URL {}", url);

    let (status, body_bytes) = data.client.get_bytes(&url).await?;

    if status < 200 || status >= 300 {
        if status == 302 {
            return match crate::utils::get_remaining_downloads(&data.client).await {
                Ok(0) => {
                    log::error!("No remaining downloads");
                    Err("No remaining downloads".into())
                }
                Ok(n) => {
                    log::warn!(
                        "Failed to download torrent, but you have {} remaining downloads, might be caused by an insufficient ratio.",
                        n
                    );
                    Err("Failed to download torrent, but you have remaining downloads.".into())
                }
                Err(e) => {
                    log::error!("Error while checking remaining downloads: {}", e);
                    Err("Failed to download torrent and check remaining downloads.".into())
                }
            };
        }
        return Err(format!("Failed to get torrent file: HTTP {}", status).into());
    }

    let mut response_builder = HttpResponse::Ok();
    response_builder
        .content_type("application/x-bittorrent")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}.torrent\"", id),
        ));

    if let Some(cookies) = data.cookies_header {
        response_builder.insert_header(("X-Session-Cookies", cookies));
    }

    Ok(response_builder.body(body_bytes))
}
