use crate::DOMAIN;
use crate::auth::KEY;
use crate::config::Config;
use crate::parser::extract_partial_torrent_infos;
use crate::search::get_rate_limiter;
use crate::utils::{
    calculate_torrent_hash, check_session_expired, flatten_tree, parse_torrent_files,
};
use actix_web::{HttpRequest, HttpResponse, get, web};
use serde_json::Value;
use wreq::Client;

#[get("/torrent/info/{path:.*}")]
pub async fn torrent_info(
    data: web::Data<Client>,
    config: web::Data<Config>,
    req_data: HttpRequest,
) -> Result<web::Json<Value>, Box<dyn std::error::Error>> {
    let path = req_data.match_info().get("path").unwrap_or("");

    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    let url = format!("https://{}/torrent/{}", domain, path);
    let _guard = get_rate_limiter().acquire().await;

    let client = data.get_ref();
    let response = client.get(&url).send().await?;
    if check_session_expired(&response) {
        info!("Session expired, trying to renew session...");
        let new_client =
            crate::auth::login(config.username.as_str(), config.password.as_str(), true).await?;
        data.get_ref().clone_from(&&new_client);
        info!("Session renewed, retrying torrent info...");

        let response = new_client.get(&url).send().await?;
        if check_session_expired(&response) {
            return Err("Session expired after renewal".into());
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
        let response = new_client.get(&url).send().await?;
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

        return Ok(web::Json(result));
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

    let _guard = get_rate_limiter().acquire().await;

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

#[get("/torrent/{path:.*}")]
pub async fn download_torrent(
    data: web::Data<Client>,
    req_data: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let path = req_data.match_info().get("path").unwrap_or("");
    //
    let id = path
        .split('/')
        .last()
        .and_then(|s| s.split('-').next())
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or("Failed to extract torrent ID from path")?;

    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    let url = format!("https://{}/engine/get_nfo?id={}", domain, id);

    let _guard = get_rate_limiter().acquire().await;

    let client = data.get_ref();
    let response = client.get(&url).send().await?;
    println!("a");
    if check_session_expired(&response) {
        return Err("Session expired".into());
    }
    println!("b");
    if !response.status().is_success() {
        return Err(format!("Failed to get torrent nfo: {}", response.status()).into());
    }
    let body = response.text().await?;
    let mut name = String::new();
    for line in body.lines() {
        if line.starts_with("Complete name : ") {
            name = line["Complete name : ".len()..].trim().to_string();
        }
    }

    let url = format!("https://{}/torrent/{}", domain, path);

    let _guard = get_rate_limiter().acquire().await;

    let response = client.get(&url).send().await?;
    if check_session_expired(&response) {
        return Err("Session expired".into());
    }
    if !response.status().is_success() {
        return Err(format!("Failed to get torrent info: {}", response.status()).into());
    }
    let body = response.text().await?;
    let document = scraper::Html::parse_document(&body);
    /*															<tr>
        <td>Info Hash</td>
        <td>99432b3992a01eb5c7b4047a459cfad2e735a900</td>
    </tr>*/
    let mut hash = String::new();
    let selector = scraper::Selector::parse("tr").unwrap();
    for element in document.select(&selector) {
        let td_selector = scraper::Selector::parse("td").unwrap();
        let tds: Vec<scraper::ElementRef> = element.select(&td_selector).collect();
        if tds.len() == 2 {
            let key = tds[0]
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string();
            let value = tds[1]
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string();
            if key == "Info Hash" {
                hash = value;
                break;
            }
        }
    }

    // redirect to /torrent/{hash}/{name}
    let response = HttpResponse::Found()
        .append_header(("Location", format!("/torrent_direct/{}/{}", hash, name)))
        .finish();
    Ok(response)
}

#[get("/torrent_direct/{hash:.*}/{name:.*}")]
pub async fn download_torrent_direct(
    data: web::Data<Client>,
    req_data: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let hash = req_data.match_info().get("hash").unwrap_or("");
    let name = req_data.match_info().get("name").unwrap_or("");
    let key = KEY.get();
    if key.is_none() {
        return Err("Passkey not found, user might not be logged in".into());
    }
    let key = key.unwrap();
    let magnet_link = format!(
        "magnet:?xt=urn:btih:{hash}&dn={name}&tr=http%3A%2F%2Ftracker.p2p-world.net%3A8080%2F{key}%2Fannounce"
    );
    // redirect to magnet link
    let response = HttpResponse::Found()
        .append_header(("Location", magnet_link))
        .finish();
    Ok(response)
}

#[get("/torrent/{id:[0-9]+}/files")]
pub async fn torrent_files(
    data: web::Data<Client>,
    config: web::Data<Config>,
    req_data: HttpRequest,
) -> Result<web::Json<Value>, Box<dyn std::error::Error>> {
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
        info!("Session expired, trying to renew session...");
        let new_client =
            crate::auth::login(config.username.as_str(), config.password.as_str(), true).await?;
        data.get_ref().clone_from(&&new_client);
        info!("Session renewed, retrying torrent files...");

        let response = new_client.get(&url).send().await?;
        if check_session_expired(&response) {
            return Err("Session expired after renewal".into());
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

        let tree = parse_torrent_files(&bytes)?;

        let flat_tree_data = flatten_tree(&tree);
        let total_size: i64 = flat_tree_data.iter().map(|(_, size)| size).sum();
        let flat_tree: Vec<Value> = flat_tree_data
            .into_iter()
            .map(|(path, size)| {
                serde_json::json!({
                    "path": path,
                    "size": size
                })
            })
            .collect();

        let result = serde_json::json!({
            "tree": tree,
            "flat_tree": flat_tree,
            "total_size": total_size,
        });

        return Ok(web::Json(result));
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

    let tree = parse_torrent_files(&bytes)?;

    let flat_tree_data = flatten_tree(&tree);
    let total_size: i64 = flat_tree_data.iter().map(|(_, size)| size).sum();
    let flat_tree: Vec<Value> = flat_tree_data
        .into_iter()
        .map(|(path, size)| {
            serde_json::json!({
                "path": path,
                "size": size
            })
        })
        .collect();

    let result = serde_json::json!({
        "tree": tree,
        "flat_tree": flat_tree,
        "total_size": total_size,
    });

    Ok(web::Json(result))
}
