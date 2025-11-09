use crate::DOMAIN;
use actix_web::{HttpRequest, HttpResponse, get, web};

use wreq::Client;

#[get("/torrent/{id:[0-9]+}")]
pub async fn download_torrent(
    data: web::Data<Client>,
    req_data: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let id = req_data.match_info().get("id").unwrap();
    let id = id.parse::<usize>()?;
    let client = data.get_ref();

    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    let url = format!("https://{}/engine/download_torrent?id={}", domain, id);

    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Failed to download torrent: {}", response.status()).into());
    }
    let bytes = response.bytes().await?;
    if bytes.len() < 250 {
        error!("Torrent {} is too small, probably not found", id);
        // 404
        let mut response = HttpResponse::NotFound();
        response.content_type("application/json");
        return Ok(response.body(format!(r#"{{"error": "Torrent {} not found"}}"#, id)));
    }
    // set attachment for automatic download
    let mut response = HttpResponse::Ok();
    response.content_type("application/x-bittorrent");
    response.insert_header((
        "Content-Disposition",
        format!("attachment; filename=\"{}.torrent\"", id),
    ));
    info!("Torrent {} downloaded", id);
    Ok(response.body(bytes))
}
