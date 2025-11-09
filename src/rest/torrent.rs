use crate::DOMAIN;
use crate::parser::extract_partial_torrent_infos;
use crate::utils::{
    calculate_torrent_hash, check_session_expired, flatten_tree, parse_torrent_files,
};
use actix_web::{HttpRequest, HttpResponse, get, web};
use serde_json::Value;
use wreq::Client;

#[get("/torrent/info/{path:.*}")]
pub async fn torrent_info(
    data: web::Data<Client>,
    req_data: HttpRequest,
) -> Result<web::Json<Value>, Box<dyn std::error::Error>> {
    let path = req_data.match_info().get("path").unwrap_or("");

    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    let url = format!("https://{}/torrent/{}", domain, path);
    let client = data.get_ref();
    let response = client.get(&url).send().await?;
    if check_session_expired(&response) {
        return Err("Session expired".into());
    }
    if !response.status().is_success() {
        if response.status().as_u16() == 404 {
            let mut response = HttpResponse::NotFound();
            response.content_type("application/json");
            return Ok(web::Json(serde_json::json!({"error": "Torrent not found"})));
        }
        return Err(format!("Failed to get torrent info: {}", response.status()).into());
    }
    let body = response.text().await?;
    let document = scraper::Html::parse_document(&body);
    let partial = extract_partial_torrent_infos(&document)?;
    let id = path
        .split('/')
        .last()
        .and_then(|s| s.split('-').next())
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or("Failed to extract torrent ID from path")?;

    let url = format!("https://{}/engine/download_torrent?id={}", domain, id);

    let response = client.get(&url).send().await?;
    if check_session_expired(&response) {
        return Err("Session expired".into());
    }
    if !response.status().is_success() {
        return Err(format!("Failed to download torrent: {}", response.status()).into());
    }
    let bytes = response.bytes().await?;
    if bytes.len() < 250 {
        error!("Torrent {} is too small, probably not found", id);
        let mut response = HttpResponse::NotFound();
        response.content_type("application/json");
        return Ok(web::Json(
            serde_json::json!({"error": format!("Torrent {} not found", id)}),
        ));
    }

    let hash = calculate_torrent_hash(&bytes)?;
    let tree = parse_torrent_files(&bytes)?;

    let flat_tree_data = flatten_tree(&tree);
    let flat_tree: Vec<Value> = flat_tree_data
        .into_iter()
        .map(|(path, size)| {
            serde_json::json!({
                "path": path,
                "size": size
            })
        })
        .collect();

    let mut result = serde_json::json!({
        "id": id,
        "hash": hash,
        "tree": tree,
        "flat_tree": flat_tree,
    });

    if let Some(partial_obj) = partial.as_object() {
        if let Some(result_obj) = result.as_object_mut() {
            for (key, value) in partial_obj {
                result_obj.insert(key.clone(), value.clone());
            }
        }
    }

    Ok(web::Json(result))
}

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
    if check_session_expired(&response) {
        return Err("Session expired".into());
    }
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
