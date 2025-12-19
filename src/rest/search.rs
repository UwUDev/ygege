use crate::config::Config;
use crate::search::{Order, Sort, search, get_rate_limiter};
use actix_web::{HttpRequest, HttpResponse, get, web};
use futures::future::join_all;
use qstring::QString;
use serde_json::Value;
use std::collections::HashSet;
use scraper::{Html, Selector};
use serde::Serialize;
use crate::dbs::DbQueryType::*;
use crate::parser::Torrent;
use wreq::Client;
use crate::DOMAIN;

#[derive(Debug, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub sub_categories: Vec<Category>,
}

#[get("/categories")]
pub async fn categories(data: web::Data<Client>) -> HttpResponse {
    let domain_lock = DOMAIN.lock().unwrap();
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    let _guard = get_rate_limiter().acquire().await;
    let url = format!("https://{}/", domain);

    match data.get(&url).send().await {
        Ok(response) => {
            let body = response.text().await.unwrap_or_default();
            let document = Html::parse_document(&body);

            let mut categories = Vec::new();
            let cat_selector = Selector::parse("#cat > ul > li:not(.misc)").unwrap();
            let link_selector = Selector::parse("a").unwrap();

            for cat_li in document.select(&cat_selector) {
                let links: Vec<_> = cat_li.select(&link_selector).collect();
                if links.is_empty() { continue; }

                // main category
                let main_href = links[0].value().attr("href").unwrap_or("");
                let main_name = links[0].text().collect::<String>().trim().to_string().replace("\n\t\t\t\t\t\t\t", " ");

                if let Some(cat_id) = extract_param(main_href, "category") {
                    let mut subs = Vec::new();

                    // subcategories
                    for link in links.iter().skip(1) {
                        let href = link.value().attr("href").unwrap_or("");
                        let name = link.text().collect::<String>().trim().to_string().replace("\n\t\t\t\t\t\t\t", " ");

                        if let Some(sub_id) = extract_param(href, "sub_category") {
                            subs.push(Category {
                                id: sub_id,
                                name,
                                sub_categories: vec![],
                            });
                        }
                    }

                    categories.push(Category {
                        id: cat_id,
                        name: main_name,
                        sub_categories: subs,
                    });
                }
            }

            HttpResponse::Ok().json(categories)
        }
        Err(e) => {
            error!("Failed to fetch categories: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch categories")
        }
    }
}

fn extract_param(url: &str, param: &str) -> Option<String> {
    url.split('&')
        .find(|s| s.contains(param))
        .and_then(|s| s.split('=').nth(1))
        .map(|s| s.to_string())
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
) -> Result<Vec<Torrent>, Box<dyn std::error::Error>> {
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

    let mut collected_torrents: HashSet<Torrent> = HashSet::new();

    for (idx, result) in results.into_iter().enumerate() {
        match result {
            Ok(mut torrents) => {
                if torrents.len() > 5 {
                    debug!(
                        "Found {} torrents for query #{} ({}) - returning immediately (> 5)",
                        torrents.len(),
                        idx + 1,
                        queries[idx]
                    );
                    Torrent::sort(&mut torrents, sort, order);
                    return Ok(torrents);
                } else if torrents.len() >= 5 {
                    debug!(
                        "Found {} torrents for query #{} ({}) - collecting for merge",
                        torrents.len(),
                        idx + 1,
                        queries[idx]
                    );
                    torrents.into_iter().for_each(|t| {
                        collected_torrents.insert(t);
                    });
                } else if !torrents.is_empty() && collected_torrents.is_empty() {
                    debug!(
                        "Found {} torrents for query #{} ({}) - collecting as fallback",
                        torrents.len(),
                        idx + 1,
                        queries[idx]
                    );
                    torrents.into_iter().for_each(|t| {
                        collected_torrents.insert(t);
                    });
                } else {
                    debug!(
                        "Query #{} ({}) returned {} results",
                        idx + 1,
                        queries[idx],
                        torrents.len()
                    );
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

    if !collected_torrents.is_empty() {
        debug!(
            "Returning {} merged torrents from multiple queries",
            collected_torrents.len()
        );
        let mut torrents: Vec<Torrent> = collected_torrents.into_iter().collect();
        Torrent::sort(&mut torrents, sort, order);
        return Ok(torrents);
    }

    debug!("All TMDB queries returned empty results");
    Ok(vec![])
}

async fn batch_category_search(
    client: &Client,
    name: Option<&str>,
    offset: Option<usize>,
    cats_list: Vec<usize>,
    sub_category: Option<usize>,
    sort: Option<Sort>,
    order: Option<Order>,
    ban_words: Option<Vec<String>>,
    config: &Config,
) -> Result<Vec<Torrent>, Box<dyn std::error::Error>> {
    debug!(
        "Starting parallel search across {} categories",
        cats_list.len()
    );

    let search_futures: Vec<_> = cats_list
        .iter()
        .map(|cat| {
            search(
                client,
                name,
                offset,
                Some(*cat),
                sub_category,
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
                if e.to_string().contains("Session expired") {
                    info!("Session expired during category search, attempting renewal...");
                    let new_client = crate::auth::login(
                        config.username.as_str(),
                        config.password.as_str(),
                        true,
                    )
                    .await?;

                    return Box::pin(batch_category_search(
                        &new_client,
                        name,
                        offset,
                        cats_list,
                        sub_category,
                        sort,
                        order,
                        ban_words,
                        config,
                    ))
                    .await;
                } else {
                    warn!("Search failed for category {}: {}", cats_list[idx], e);
                }
            }
        }
    }

    debug!(
        "Returning {} merged torrents from {} categories",
        collected_torrents.len(),
        cats_list.len()
    );
    let mut torrents: Vec<Torrent> = collected_torrents.into_iter().collect();
    Torrent::sort(&mut torrents, sort, order);
    Ok(torrents)
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
    let cats = qs.get("categories");
    let connarr = qs.get("connarr");

    debug!("Prowlarr/Jackett detected");

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

    if config.tmdb_token.is_some() && (qs.get("tmdbid").is_some() || qs.get("imdbid").is_some()) {
        let db_search = if let Some(id) = qs.get("tmdbid") {
            Some((id, TMDB, "TMDB"))
        } else if let Some(id) = qs.get("imdbid") {
            Some((id, IMDB, "IMDB"))
        } else {
            None
        };

        return if let Some((id, db_type, db_name)) = db_search {
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
                        let torrent_json: Vec<Value> =
                            results.into_iter().map(|t| t.to_json()).collect();
                        return Ok(web::Json(torrent_json));
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
        if connarr.is_some() {
            order = Some(Order::Descending);
            sort = Some(Sort::PublishDate);
        }
    }

    // Bulk category search when categories are provided without a specific category
    if category.is_none() && categories_list.is_some() {
        let cats = categories_list.unwrap();
        debug!(
            "Performing bulk search across {} categories: {:?}",
            cats.len(),
            cats
        );

        let results = batch_category_search(
            &data,
            name,
            offset,
            cats,
            sub_category,
            sort,
            order,
            ban_words.clone(),
            &config,
        )
        .await?;

        info!("{} torrents found via bulk category search", results.len());
        let torrent_json: Vec<Value> = results.into_iter().map(|t| t.to_json()).collect();
        return Ok(web::Json(torrent_json));
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
