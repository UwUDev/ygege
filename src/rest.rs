use crate::DOMAIN;
use crate::search::{Order, Sort, search};
use actix_web::{HttpRequest, HttpResponse, get, web};
use qstring::QString;
use rquest::Client;
use serde_json::Value;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(categories)
        .service(ygg_search)
        .service(download_torrent);
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
    req_data: HttpRequest,
) -> Result<web::Json<Vec<Value>>, Box<dyn std::error::Error>> {
    let query = req_data.query_string();
    let qs = QString::from(query);
    let name = qs.get("name");
    let offset = qs.get("offset").and_then(|s| s.parse::<usize>().ok());
    let category = qs.get("category").and_then(|s| s.parse::<usize>().ok());
    let sub_category = qs.get("sub_category").and_then(|s| s.parse::<usize>().ok());
    let sort = qs.get("sort").and_then(|s| s.parse::<Sort>().ok());
    let order = qs.get("order").and_then(|s| s.parse::<Order>().ok());

    let torrents = search(&data, name, offset, category, sub_category, sort, order).await;
    match torrents {
        Ok(torrents) => {
            let mut json = vec![];
            for torrent in torrents {
                json.push(torrent.to_json());
            }

            Ok(web::Json(json))
        }
        Err(e) => Err(format!("Failed to fetch search results: {}", e).into()),
    }
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
    // set attachment for automatic download
    let mut response = HttpResponse::Ok();
    response.content_type("application/x-bittorrent");
    response.insert_header((
        "Content-Disposition",
        format!("attachment; filename=\"{}.torrent\"", id),
    ));
    Ok(response.body(bytes))
}
