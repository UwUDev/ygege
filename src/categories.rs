use crate::DOMAIN;
use crate::search::get_rate_limiter;
use scraper::{Html, Selector};
use serde::Serialize;
use tokio::sync::OnceCell;
use wreq::Client;

pub static CATEGORIES_CACHE: OnceCell<Vec<Category>> = OnceCell::const_new();

#[derive(Debug, Serialize, Clone)]
pub struct Category {
    pub id: usize,
    pub name: String,
    pub sub_categories: Vec<Category>,
}

pub(crate) async fn scrape_categories(
    client: &Client,
) -> Result<Vec<Category>, Box<dyn std::error::Error>> {
    let domain_lock = DOMAIN.lock().unwrap();
    let domain = domain_lock.clone();
    drop(domain_lock);

    let _guard = get_rate_limiter().acquire().await;
    let url = format!("https://{}/", domain);

    let response = client.get(&url).send().await?;
    let body = response.text().await.unwrap_or_default();
    let document = Html::parse_document(&body);

    let mut categories_list = Vec::new();
    let cat_selector = Selector::parse("#cat > ul > li:not(.misc)").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    for cat_li in document.select(&cat_selector) {
        let links: Vec<_> = cat_li.select(&link_selector).collect();
        if links.is_empty() {
            continue;
        }

        // main category
        let main_href = links[0].value().attr("href").unwrap_or("");
        let main_name = links[0]
            .text()
            .collect::<String>()
            .trim()
            .to_string()
            .replace("\n\t\t\t\t\t\t\t", " ");

        if let Some(cat_id) = extract_param(main_href, "category") {
            let mut subs = Vec::new();

            // subcategories
            for link in links.iter().skip(1) {
                let href = link.value().attr("href").unwrap_or("");
                let name = link
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string()
                    .replace("\n\t\t\t\t\t\t\t", " ");

                if let Some(sub_id) = extract_param(href, "sub_category") {
                    subs.push(Category {
                        id: sub_id.parse::<usize>()?,
                        name,
                        sub_categories: vec![],
                    });
                }
            }

            categories_list.push(Category {
                id: cat_id.parse::<usize>()?,
                name: main_name,
                sub_categories: subs,
            });
        }
    }

    Ok(categories_list)
}

pub async fn init_categories(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Initializing categories cache...");
    let categories_data = scrape_categories(client).await?;
    debug!("Cached {} categories", categories_data.len());
    CATEGORIES_CACHE
        .set(categories_data)
        .map_err(|_| "Failed to set categories cache")?;
    Ok(())
}

fn extract_param(url: &str, param: &str) -> Option<String> {
    url.split('&')
        .find(|s| s.contains(param))
        .and_then(|s| s.split('=').nth(1))
        .map(|s| s.to_string())
}
