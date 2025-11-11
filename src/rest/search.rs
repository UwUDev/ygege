use crate::config::Config;
use crate::search::{Order, Sort, search};
use actix_web::{HttpRequest, HttpResponse, get, web};
use qstring::QString;
use serde_json::Value;

use wreq::Client;

#[get("/categories")]
pub async fn categories() -> HttpResponse {
    let json = include_str!("../../categories.json");
    let mut response = HttpResponse::Ok();
    response.content_type("application/json");
    response.body(json)
}

#[get("/search")]
pub async fn ygg_search(
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
    let mut sort = qs.get("sort").and_then(|s| s.parse::<Sort>().ok());
    let mut order = qs.get("order").and_then(|s| s.parse::<Order>().ok());
    let rssarr = qs.get("categories"); // Connard ?

    // Prowlarr indexer test compatibility... (Connard v2 ?)
    if qs.get("imdbid").is_some() && qs.get("tmdbid").is_some() {
        return Ok(web::Json(vec![]));
    }

    if name.is_none() {
        name = qs.get("q");
    }

    // Prowlarr RSS feed compatibility trick (Connard v3 ?)
    if name.is_none() {
        if let Some(rssarr) = rssarr {
            if rssarr == "System.Int32%5B%5D" || rssarr == "System.Int32[]" {
                order = Some(Order::Descending);
                sort = Some(Sort::PublishDate);
            }
        }
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
