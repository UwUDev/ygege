use crate::categories::cat_id_to_nostr_tag;
use crate::nostr::NostrClient;
use crate::parser::Torrent;
use crate::rate_limiter::RateLimiter;
use std::str::FromStr;
use std::sync::OnceLock;

static RATE_LIMITER: OnceLock<RateLimiter> = OnceLock::new();

pub(crate) fn get_rate_limiter() -> &'static RateLimiter {
    RATE_LIMITER.get_or_init(|| RateLimiter::default())
}

pub async fn search(
    nostr: &NostrClient,
    name: &str,
    category: Option<usize>,
    sort: Option<Sort>,
    order: Option<Order>,
    ban_words: Option<Vec<String>>,
) -> Result<Vec<Torrent>, Box<dyn std::error::Error + Send + Sync>> {
    debug!(
        "Searching via Nostr (query: {:?}, category: {:?})",
        name, category
    );

    let _guard = get_rate_limiter().acquire().await;

    let tag_filter = category.and_then(|id| cat_id_to_nostr_tag(id));

    let start = std::time::Instant::now();
    let mut torrents = nostr.search(name, tag_filter, 100).await?;
    debug!("Got {} results in {:?}", torrents.len(), start.elapsed());

    if let Some(ban_words) = ban_words {
        torrents.retain(|t| {
            !ban_words
                .iter()
                .any(|w| t.name.to_lowercase().contains(&w.to_lowercase()))
        });
    }

    Torrent::sort(&mut torrents, sort, order);
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
