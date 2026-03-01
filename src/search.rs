pub(crate) use crate::categories::CATEGORIES_CACHE;
use crate::parser::Torrent;
use crate::rate_limiter::RateLimiter;
use crate::utils::check_session_expired;
use crate::ygg_client::YggClient;
use crate::{DOMAIN, parser};
use std::str::FromStr;
use std::sync::OnceLock;
use urlencoding::{decode, encode};

static RATE_LIMITER: OnceLock<RateLimiter> = OnceLock::new();

pub(crate) fn get_rate_limiter() -> &'static RateLimiter {
    RATE_LIMITER.get_or_init(|| RateLimiter::default())
}

pub async fn search(
    client: &YggClient,
    name: &str,
    offset: Option<usize>,
    category: Option<usize>,
    sub_category: Option<usize>,
    sort: Option<Sort>,
    order: Option<Order>,
    ban_words: Option<Vec<String>>,
    quote_search: bool,
) -> Result<Vec<Torrent>, Box<dyn std::error::Error>> {
    let name = match quote_search {
        true if !name.is_empty() => {
            let decoded = decode(name).unwrap_or(std::borrow::Cow::Borrowed(name));
            decoded
                .split(|c| c == '+' || c == ' ')
                .filter(|w| !w.is_empty())
                .map(|w| format!("\"{}\"", w))
                .collect::<Vec<_>>()
                .join(" ")
        }
        _ => name.to_owned(),
    };
    debug!(
        "Searching for torrents (name: {:?}, offset: {:?}, category: {:?}, sub_category: {:?}, sort: {:?}, order: {:?})",
        name, offset, category, sub_category, sort, order
    );

    let _guard = get_rate_limiter().acquire().await;

    let url = build_query_url(name.as_str(), offset, category, sub_category, sort, order)?;
    let start = std::time::Instant::now();
    let response = client.get(&url).await?;

    if check_session_expired(response.status, &response.url) {
        return Err("Session expired".into());
    }

    debug!("Search response: {}", response.status);
    let torrents = parser::extract_torrents(&response.body)?;
    let torrents = if let Some(ban_words) = ban_words {
        torrents
            .into_iter()
            .filter(|torrent| {
                !ban_words
                    .iter()
                    .any(|word| torrent.name.to_lowercase().contains(&word.to_lowercase()))
            })
            .collect()
    } else {
        torrents
    };
    let stop = std::time::Instant::now();
    debug!(
        "Found {} torrents in {:?}",
        torrents.len(),
        stop.duration_since(start)
    );
    Ok(torrents)
}

#[derive(Debug, Clone, Copy)]
pub enum Sort {
    Name,
    Seed,
    Comments,
    PublishDate,
    Completed,
    Leech,
}

#[derive(Debug, Clone, Copy)]
pub enum Order {
    Ascending,
    Descending,
}
impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            Sort::Name => "name",
            Sort::Seed => "seed",
            Sort::Comments => "comments",
            Sort::PublishDate => "publish_date",
            Sort::Completed => "completed",
            Sort::Leech => "leech",
        }
    }
}

impl Order {
    pub fn as_str(&self) -> &str {
        match self {
            Order::Ascending => "asc",
            Order::Descending => "desc",
        }
    }
}

impl FromStr for Sort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "name" => Ok(Sort::Name),
            "seed" => Ok(Sort::Seed),
            "comments" => Ok(Sort::Comments),
            "publish_date" => Ok(Sort::PublishDate),
            "completed" => Ok(Sort::Completed),
            "leech" => Ok(Sort::Leech),
            _ => Err(format!("Valeur de tri invalide : {}", s)),
        }
    }
}

impl FromStr for Order {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Order::Ascending),
            "desc" => Ok(Order::Descending),
            _ => Err(format!("Ordre invalide : {}", s)),
        }
    }
}

fn build_query_url(
    name: &str,
    offset: Option<usize>,
    category: Option<usize>,
    sub_category: Option<usize>,
    sort: Option<Sort>,
    order: Option<Order>,
) -> Result<String, Box<dyn std::error::Error>> {
    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    let mut url = format!("https://{domain}/engine/search?name={}", encode(&name));
    if let Some(offset) = offset {
        url.push_str(&format!("&page={offset}"));
    }
    if let Some(category) = category {
        if let Some((cat, sub_cat)) = get_category_pair(category) {
            url.push_str(&format!("&category={}", cat));
            url.push_str(&format!("&sub_category={}", sub_cat));
        }
    }
    if let Some(sub_category) = sub_category {
        url.push_str(&format!("&sub_category={sub_category}"));
    }
    if let Some(sort) = sort {
        url.push_str(&format!("&sort={}", sort.as_str()));
    }
    if let Some(order) = order {
        url.push_str(&format!("&order={}", order.as_str()));
    }
    url.push_str("&do=search");
    Ok(url)
}

fn get_category_pair(category: usize) -> Option<(String, String)> {
    for cat in CATEGORIES_CACHE.get().unwrap().iter() {
        if cat.id == category {
            return Some((category.to_string(), "all".to_string()));
        }
        for sub_cat in &cat.sub_categories {
            if sub_cat.id == category {
                return Some((cat.id.to_string(), category.to_string()));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::get_ygg_domain;

    #[tokio::test]
    async fn test_build_query_url() {
        let domain = get_ygg_domain().await.unwrap_or_else(|_| {
            error!("Failed to get YGG domain");
            std::process::exit(1);
        });
        let mut domain_lock = DOMAIN.lock().unwrap();
        *domain_lock = domain.clone();
        drop(domain_lock);

        let url = build_query_url(
            "Vaiana",
            None,
            None,
            None,
            Some(Sort::Name),
            Some(Order::Ascending),
        )
        .unwrap();
        println!("URL: {}", url);
    }
}
