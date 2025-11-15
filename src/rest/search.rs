use crate::config::Config;
use crate::search::{Order, Sort, search};
use actix_web::{HttpRequest, HttpResponse, get, web};
use futures::future::join_all;
use qstring::QString;
use serde_json::Value;

use crate::dbs::DbQueryType::*;
use wreq::Client;

#[get("/categories")]
pub async fn categories() -> HttpResponse {
    let json = include_str!("../../categories.json");
    let mut response = HttpResponse::Ok();
    response.content_type("application/json");
    response.body(json)
}

async fn batch_best_search(
    client: &Client,
    queries: Vec<String>,
    offset: Option<usize>,
    category: Option<usize>,
    sub_category: Option<usize>,
    sort: Option<Sort>,
    order: Option<Order>,
    ban_words: Option<Vec<String>>,
    config: &Config,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    debug!("Starting parallel search for {} queries", queries.len());

    for (idx, query) in queries.iter().enumerate() {
        debug!("Query #{}: {}", idx + 1, query);
    }

    let search_futures: Vec<_> = queries
        .iter()
        .map(|query| {
            search(
                client,
                Some(query.as_str()),
                offset,
                category,
                sub_category,
                sort,
                order,
                ban_words.clone(),
            )
        })
        .collect();

    let results = join_all(search_futures).await;

    for (idx, result) in results.into_iter().enumerate() {
        match result {
            Ok(torrents) => {
                if !torrents.is_empty() && torrents.len() >= 5 {
                    debug!(
                        "Found {} torrents for query #{} ({})",
                        torrents.len(),
                        idx + 1,
                        queries[idx]
                    );
                    let json: Vec<Value> = torrents.into_iter().map(|t| t.to_json()).collect();
                    return Ok(json);
                } else {
                    debug!("Query #{} ({}) returned no results", idx + 1, queries[idx]);
                }
            }
            Err(e) => {
                if e.to_string().contains("Session expired") {
                    info!("Session expired during TMDB search, attempting renewal...");
                    let new_client = crate::auth::login(
                        config.username.as_str(),
                        config.password.as_str(),
                        true,
                    )
                    .await?;

                    return Box::pin(batch_best_search(
                        &new_client,
                        queries,
                        offset,
                        category,
                        sub_category,
                        sort,
                        order,
                        ban_words,
                        config,
                    ))
                    .await;
                } else {
                    warn!(
                        "Search failed for query #{} ({}): {}",
                        idx + 1,
                        queries[idx],
                        e
                    );
                }
            }
        }
    }

    debug!("All TMDB queries returned empty results");
    Ok(vec![])
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
    let rssarr = qs.get("categories");
    let ban_words = qs.get("ban_words").and_then(|s| {
        let v: Vec<String> = s
            .split(',')
            .map(|word| word.trim().to_string())
            .filter(|w| !w.is_empty())
            .collect();
        if v.is_empty() { None } else { Some(v) }
    });

    if config.tmdb_token.is_some() {
        let db_search = if let Some(id) = qs.get("tmdbid") {
            Some((id, TMDB, "TMDB"))
        } else if let Some(id) = qs.get("imdbid") {
            Some((id, IMDB, "IMDB"))
        } else {
            None
        };

        return if let Some((id, db_type, db_name)) = db_search {
            println!("aaaaa");
            match crate::dbs::get_queries(
                id.to_string(),
                &config.tmdb_token.clone().unwrap(),
                db_type,
            )
            .await
            {
                Ok(queries) => {
                    debug!(
                        "Got {} queries from {} for ID {}",
                        queries.len(),
                        db_name,
                        id
                    );
                    let results = batch_best_search(
                        &data,
                        queries,
                        offset,
                        category,
                        sub_category,
                        sort,
                        order,
                        ban_words.clone(),
                        &config,
                    )
                    .await?;

                    if !results.is_empty() {
                        info!("{} torrents found via {} search", results.len(), db_name);
                        return Ok(web::Json(results));
                    }
                    debug!(
                        "{} search returned no results, falling back to regular search",
                        db_name
                    );
                    Ok(web::Json(vec![]))
                }
                Err(e) => {
                    warn!("Failed to get {} queries for ID {}: {}", db_name, id, e);
                    Ok(web::Json(vec![]))
                }
            }
        } else {
            warn!("No valid database ID provided for DB search");
            Ok(web::Json(vec![]))
        };
    } else {
        if qs.get("tmdbid").is_some() || qs.get("imdbid").is_some() {
            warn!("Database ID provided but no TMDB token configured, skipping database search");
            return Ok(web::Json(vec![]));
        }
    }

    if name.is_none() {
        name = qs.get("q");
    }

    // Prowlarr RSS feed compatibility trick
    if name.is_none() {
        if let Some(rssarr) = rssarr {
            if rssarr == "System.Int32%5B%5D" || rssarr == "System.Int32[]" {
                order = Some(Order::Descending);
                sort = Some(Sort::PublishDate);
            }
        }
    }

    let torrents = search(
        &data,
        name,
        offset,
        category,
        sub_category,
        sort,
        order,
        ban_words.clone(),
    )
    .await;

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
                    ban_words,
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
