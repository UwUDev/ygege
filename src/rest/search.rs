use crate::config::Config;
use crate::dbs::DbQueryType::*;
use crate::nostr::NostrClient;
use crate::parser::Torrent;
use crate::search::{Order, Sort, search};
use actix_web::{HttpRequest, HttpResponse, get, web};
use futures::future::join_all;
use qstring::QString;
use serde_json::Value;
use std::collections::HashSet;

async fn batch_best_search(
    nostr: &NostrClient,
    queries: Vec<String>,
    category: Option<usize>,
    sort: Option<Sort>,
    order: Option<Order>,
    ban_words: Option<Vec<String>>,
) -> Result<Vec<Torrent>, Box<dyn std::error::Error + Send + Sync>> {
    debug!("Starting parallel search for {} queries", queries.len());

    let search_futures: Vec<_> = queries
        .iter()
        .map(|query| {
            search(
                nostr,
                query.as_str(),
                category,
                sort,
                order,
                ban_words.clone(),
            )
        })
        .collect();

    let results = join_all(search_futures).await;

    let mut collected_torrents: HashSet<Torrent> = HashSet::new();

    for (idx, result) in results.into_iter().enumerate() {
        match result {
            Ok(mut torrents) => {
                if torrents.len() > 5 {
                    debug!(
                        "Found {} torrents for query #{} - returning immediately",
                        torrents.len(),
                        idx + 1,
                    );
                    Torrent::sort(&mut torrents, sort, order);
                    return Ok(torrents);
                } else {
                    torrents.into_iter().for_each(|t| {
                        collected_torrents.insert(t);
                    });
                }
            }
            Err(e) => {
                warn!("Search failed for query #{}: {}", idx + 1, e);
            }
        }
    }

    let mut torrents: Vec<Torrent> = collected_torrents.into_iter().collect();
    Torrent::sort(&mut torrents, sort, order);
    Ok(torrents)
}

async fn batch_category_search(
    nostr: &NostrClient,
    name: &str,
    cats_list: Vec<usize>,
    sort: Option<Sort>,
    order: Option<Order>,
    ban_words: Option<Vec<String>>,
) -> Result<Vec<Torrent>, Box<dyn std::error::Error + Send + Sync>> {
    debug!(
        "Starting parallel search across {} categories",
        cats_list.len()
    );

    let search_futures: Vec<_> = cats_list
        .iter()
        .map(|cat| search(nostr, name, Some(*cat), sort, order, ban_words.clone()))
        .collect();

    let results = join_all(search_futures).await;

    let mut collected_torrents: HashSet<Torrent> = HashSet::new();

    for (idx, result) in results.into_iter().enumerate() {
        match result {
            Ok(torrents) => {
                debug!(
                    "Category {} returned {} results",
                    cats_list[idx],
                    torrents.len()
                );
                torrents.into_iter().for_each(|t| {
                    collected_torrents.insert(t);
                });
            }
            Err(e) => {
                warn!("Search failed for category {}: {}", cats_list[idx], e);
            }
        }
    }

    let mut torrents: Vec<Torrent> = collected_torrents.into_iter().collect();
    Torrent::sort(&mut torrents, sort, order);
    Ok(torrents)
}

#[get("/search")]
pub async fn ygg_search(
    nostr: web::Data<NostrClient>,
    config: web::Data<Config>,
    req_data: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let query = req_data.query_string();
    debug!("Received query: {}", query);
    let qs = QString::from(query);
    let name = qs.get("name").or(qs.get("q")).unwrap_or("");
    let category = qs.get("category").and_then(|s| s.parse::<usize>().ok());
    let mut sort = qs.get("sort").and_then(|s| s.parse::<Sort>().ok());
    let mut order = qs.get("order").and_then(|s| s.parse::<Order>().ok());
    let cats = qs.get("categories");
    let connarr = qs.get("connarr");

    let ban_words = qs.get("ban_words").and_then(|s| {
        let v: Vec<String> = s
            .split(',')
            .map(|word| word.trim().to_string())
            .filter(|w| !w.is_empty())
            .collect();
        if v.is_empty() { None } else { Some(v) }
    });

    let mut categories_list = if let Some(cats) = cats {
        let decoded = urlencoding::decode(cats).unwrap_or(std::borrow::Cow::Borrowed(cats));
        let parsed: Vec<usize> = decoded
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();
        if !parsed.is_empty() {
            Some(parsed)
        } else {
            None
        }
    } else {
        None
    };

    if categories_list.is_some() && connarr.is_some() {
        if categories_list.as_ref().unwrap().len() > 2 {
            categories_list = None;
        }
    }

    // TMDB/IMDB lookup
    if config.tmdb_token.is_some() && (qs.get("tmdbid").is_some() || qs.get("imdbid").is_some()) {
        let db_search = if let Some(id) = qs.get("tmdbid") {
            Some((id, TMDB, "TMDB"))
        } else if let Some(id) = qs.get("imdbid") {
            Some((id, IMDB, "IMDB"))
        } else {
            None
        };

        if let Some((id, db_type, db_name)) = db_search {
            match crate::dbs::get_queries(
                id.to_string(),
                &config.tmdb_token.clone().unwrap(),
                db_type,
            )
            .await
            {
                Ok(queries) => {
                    let results =
                        batch_best_search(&nostr, queries, category, sort, order, ban_words)
                            .await
                            .map_err(|e| format!("{}", e))?;

                    if !results.is_empty() {
                        info!("{} torrents found via {} search", results.len(), db_name);
                        let json: Vec<Value> = results.into_iter().map(|t| t.to_json()).collect();
                        return Ok(HttpResponse::Ok().json(json));
                    }
                    return Ok(HttpResponse::Ok().json(Vec::<Value>::new()));
                }
                Err(e) => {
                    warn!("Failed to get {} queries for ID {}: {}", db_name, id, e);
                    return Ok(HttpResponse::Ok().json(Vec::<Value>::new()));
                }
            }
        }
    } else if qs.get("tmdbid").is_some() || qs.get("imdbid").is_some() {
        warn!("Database ID provided but no TMDB token configured, skipping database search");
        return Ok(HttpResponse::Ok().json(Vec::<Value>::new()));
    }

    // RSS feed: return recent torrents sorted by date
    if name.is_empty() && connarr.is_some() {
        order = Some(Order::Descending);
        sort = Some(Sort::PublishDate);
    }

    // Bulk category search
    if category.is_none() && categories_list.is_some() {
        let cats = categories_list.unwrap();
        let results = batch_category_search(&nostr, name, cats, sort, order, ban_words)
            .await
            .map_err(|e| format!("{}", e))?;
        info!("{} torrents found via bulk category search", results.len());
        let json: Vec<Value> = results.into_iter().map(|t| t.to_json()).collect();
        return Ok(HttpResponse::Ok().json(json));
    }

    let torrents = search(&nostr, name, category, sort, order, ban_words)
        .await
        .map_err(|e| format!("{}", e))?;

    let json: Vec<Value> = torrents.iter().map(|t| t.to_json()).collect();
    info!("{} torrents found", json.len());
    Ok(HttpResponse::Ok().json(json))
}
