use crate::config::Config;
use crate::search::{Order, Sort, search};
use crate::{DOMAIN, resolver};
use actix_web::{HttpRequest, HttpResponse, get, web};
use qstring::QString;
use serde_json::Value;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use wreq::Client;
use wreq::dns::Resolve;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(categories)
        .service(ygg_search)
        .service(download_torrent)
        .service(get_user_info)
        .service(health_check)
        .service(status_check);
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

#[get("/status")]
async fn status_check(data: web::Data<Client>) -> HttpResponse {
    let domain_lock = DOMAIN.lock().unwrap();
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    let search = search(
        &data,
        Some("Vaiana"),
        None,
        None,
        None,
        Some(Sort::Seed),
        Some(Order::Ascending),
    )
    .await;

    let auth: &str;
    let search_status: &str;
    let parsing: &str;
    match search {
        Ok(torrents) => {
            auth = "authenticated";
            search_status = "ok";
            if torrents.is_empty() {
                parsing = "failed";
            } else {
                parsing = "ok";
            }
        }
        Err(e) => {
            if e.to_string().contains("Session expired") {
                auth = "not_authenticated";
                search_status = "ok";
                parsing = "n/a";
            } else {
                error!("Status check auth error: {}", e);
                auth = "auth_error";
                search_status = "failed";
                parsing = "n/a";
            }
        }
    }

    let user = crate::user::get_account(&data).await;
    let user_status = match user.is_ok() {
        true => "ok",
        false => "failed",
    };

    // DNS lookup to check if domain resolves via 1.1.1.1
    let resolver = resolver::AsyncCloudflareResolverAdapter::new().unwrap();
    let mut domain_ping = "unreachable";
    let dns_lookup = match resolver
        .resolve(wreq::dns::Name::from_str(domain).unwrap())
        .await
    {
        Ok(ip) => {
            // convert wreq::dns::resolve::Addrs to core::net::ip_addr::IpAddr
            let ip = ip
                .into_iter()
                .next()
                .and_then(|socket_addr| Some(socket_addr.ip()));

            if ip.is_some() {
                let ip_addr = ip.unwrap();
                info!("Resolved IP: {}", ip_addr);

                let socket_addr = SocketAddr::new(ip_addr, 443);
                domain_ping =
                    match timeout(Duration::from_secs(5), TcpStream::connect(socket_addr)).await {
                        Ok(Ok(_)) => {
                            info!("TCP connection to {} successful", socket_addr);
                            "reachable"
                        }
                        Ok(Err(e)) => {
                            error!("TCP connection failed: {}", e);
                            "unreachable"
                        }
                        Err(_) => {
                            error!("TCP connection timeout");
                            "timeout"
                        }
                    };
            }

            "resolves"
        }
        Err(_) => "does_not_resolve",
    };

    let status = serde_json::json!({
        "domain": domain,
        "auth": auth,
        "search": search_status,
        "user_info": user_status,
        "domain_reachability": domain_ping,
        "domain_dns": dns_lookup,
        "parsing": parsing,
    });

    HttpResponse::Ok().json(status)
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
