use crate::DOMAIN;
use crate::config::Config;
use crate::search::{Order, Sort, search};
use actix_web::{HttpRequest, HttpResponse, get, web};
use qstring::QString;
use serde_json::Value;
use wreq::Client;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(categories)
        .service(ygg_search)
        .service(download_torrent)
        .service(get_user_info);
}

#[get("/categories")]
async fn categories() -> HttpResponse {
    let json = include_str!("../categories.json");
    // set content type to json
    let mut response = HttpResponse::Ok();
    response.content_type("application/json");
    response.body(json)
}

#[get("/search")]
async fn ygg_search(
    data: web::Data<Client>,
    config: web::Data<Config>,
    req_data: HttpRequest,
) -> Result<web::Json<Vec<Value>>, Box<dyn std::error::Error>> {
    let query = req_data.query_string();
    debug!("Received query: {}", query);
    let qs = QString::from(query);
    let mut name = qs.get("name");
    let offset = qs.get("offset").and_then(|s| s.parse::<usize>().ok());
    let category = qs.get("category").and_then(|s| s.parse::<usize>().ok());
    let sub_category = qs.get("sub_category").and_then(|s| s.parse::<usize>().ok());
    let sort = qs.get("sort").and_then(|s| s.parse::<Sort>().ok());
    let order = qs.get("order").and_then(|s| s.parse::<Order>().ok());

    if qs.get("imdbid").is_some() && qs.get("tmdbid").is_some() {
        return Ok(web::Json(vec![]));
    }

    if name.is_none() {
        name = qs.get("q");
    }

    /*if name.is_none() {
        return Ok(web::Json(vec![]));
    }*/

    let torrents = search(&data, name, offset, category, sub_category, sort, order).await;
    match torrents {
        Ok(torrents) => {
            let mut json = vec![];
            for torrent in torrents {
                json.push(torrent.to_json());
            }

            info!("{} torrents found", json.len());
            Ok(web::Json(json))
        }
        Err(e) => {
            // if session expired
            if e.to_string().contains("Session expired") {
                info!("Trying to renew session...");
                let new_client =
                    crate::auth::login(config.username.as_str(), config.password.as_str(), true)
                        .await?;
                data.get_ref().clone_from(&&new_client);
                info!("Session renewed, retrying search...");
                let torrents = search(
                    &new_client,
                    name,
                    offset,
                    category,
                    sub_category,
                    sort,
                    order,
                )
                .await?;
                let mut json = vec![];
                for torrent in torrents {
                    json.push(torrent.to_json());
                }
                info!("{} torrents found", json.len());
                Ok(web::Json(json))
            } else {
                error!("Search error: {}", e);
                Err(e)
            }
        }
    }
}

#[get("/user")]
async fn get_user_info(
    data: web::Data<Client>,
    config: web::Data<Config>,
) -> Result<web::Json<Value>, Box<dyn std::error::Error>> {
    let client = data.get_ref();

    let user = crate::user::get_account(client).await;
    // check if error is session expired
    if let Err(e) = &user {
        if e.to_string().contains("Session expired") {
            info!("Trying to renew session...");
            let new_client =
                crate::auth::login(config.username.as_str(), config.password.as_str(), true)
                    .await?;
            data.get_ref().clone_from(&&new_client);
            info!("Session renewed, retrying to get user info...");
            let user = crate::user::get_account(&new_client).await?;
            let json = serde_json::to_value(&user)?;
            return Ok(web::Json(json));
        }
    }

    let user = user?;
    let json = serde_json::to_value(&user)?;
    Ok(web::Json(json))
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/torrent/{id:[0-9]+}")]
async fn download_torrent(
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
