use crate::{parser, DOMAIN};
use crate::parser::Torrent;

pub async fn search(
    client: &rquest::Client,
    name: Option<&str>,
    offset: Option<usize>,
    category: Option<usize>,
    sub_category: Option<usize>,
    sort: Option<Sort>,
    order: Option<Order>,
) -> Result<Vec<Torrent>, Box<dyn std::error::Error>> {
    debug!("Searching for torrents (name: {:?}, offset: {:?}, category: {:?}, sub_category: {:?}, sort: {:?}, order: {:?})", name, offset, category, sub_category, sort, order);
    let url = build_query_url(name, offset, category, sub_category, sort, order)?;
    let start = std::time::Instant::now();
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch search results: {}", response.status()).into());
    }
    debug!("Search response: {}", response.status());
    let body = response.text().await?;
    let torrents = parser::extract_torrents(&body)?;
    let stop = std::time::Instant::now();
    debug!("Found {} torrents in {:?}", torrents.len(), stop.duration_since(start));
    Ok(torrents)
}

#[derive(Debug)]
pub enum Sort {
    Name,
    Seed,
    Comments,
    PublishDate,
    Completed,
    Leech,
}

#[derive(Debug)]
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

fn build_query_url(
    name: Option<&str>,
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

    let name = name.unwrap_or("");

    let mut url = format!("https://{domain}/engine/search?name={name}");
    if let Some(offset) = offset {
        url.push_str(&format!("&page={offset}"));
    }
    if let Some(category) = category {
        url.push_str(&format!("&category={category}"));
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
            Some("Vaiana"),
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
