use crate::DOMAIN;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Torrent {
    pub category_id: usize,
    pub name: String,
    pub id: usize,
    pub comments_count: usize,
    pub age_stamp: usize,
    pub size: u64,
    pub completed: usize,
    pub seed: usize,
    pub leech: usize,
}

impl Torrent {
    pub fn get_url(&self) -> Result<String, Box<dyn std::error::Error>> {
        let domain_lock = DOMAIN.lock()?;
        let cloned_guard = domain_lock.clone();
        let domain = cloned_guard.as_str();
        drop(domain_lock);
        Ok(format!(
            "https://{}/engine/download_torrent?id={}",
            domain, self.id
        ))
    }

    pub fn get_download_url(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("/torrent/{}", self.id))
    }

    pub fn to_json(&self) -> Value {
        let mut value = serde_json::to_value(self).unwrap();
        value["url"] = Value::String(self.get_url().unwrap());
        value["download"] = Value::String(self.get_download_url().unwrap());
        value
    }
}

pub fn extract_torrents(body: &str) -> Result<Vec<Torrent>, Box<dyn std::error::Error>> {
    let mut torrents = Vec::new();
    let doc = Html::parse_document(body);

    let table_selector = Selector::parse("#\\#torrents div.table-responsive > table > tbody")?;
    let table = doc
        .select(&table_selector)
        .next()
        .ok_or("Unable to find table")?;
    debug!(
        "detected {} torrents",
        table.select(&Selector::parse("tr")?).count()
    );

    for row in table.select(&Selector::parse("tr")?) {
        let columns: Vec<_> = row.select(&Selector::parse("td")?).collect();
        if columns.len() < 9 {
            continue;
        }

        let category_id = row
            .select(&Selector::parse("div")?)
            .next()
            .and_then(|e| e.text().next())
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or_default();

        let name = columns[1]
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        let id = columns[2]
            .select(&Selector::parse("#get_nfo")?)
            .next()
            .and_then(|e| e.value().attr("target"))
            .and_then(|t| t.parse().ok())
            .unwrap_or_default();

        let comments_count = columns[3]
            .text()
            .next()
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or_default();

        let age_stamp = columns[4]
            .select(&Selector::parse("div.hidden")?)
            .next()
            .and_then(|e| e.text().next())
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or_default();

        let size = columns[5]
            .text()
            .next()
            .map(|t| human_readable_size_to_bytes(t.trim()))
            .transpose()?
            .unwrap_or_default();

        let completed = columns[6]
            .text()
            .next()
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or_default();

        let seed = columns[7]
            .text()
            .next()
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or_default();

        let leech = columns[8]
            .text()
            .next()
            .and_then(|t| t.trim().parse().ok())
            .unwrap_or_default();

        torrents.push(Torrent {
            category_id,
            name,
            id,
            comments_count,
            age_stamp,
            size,
            completed,
            seed,
            leech,
        });
    }

    debug!("Parsed {} torrents", torrents.len());

    Ok(torrents)
}

const SIZES: [&str; 5] = ["o", "ko", "Mo", "Go", "To"];

fn human_readable_size_to_bytes(size: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let size = size.trim();
    let mut split_index = 0;
    let mut chars = size.chars();

    // Trouve la séparation entre nombre et unité
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() || c == '.' {
            split_index += 1;
        } else {
            break;
        }
    }

    if split_index == 0 {
        return Err("Format invalide : partie numérique manquante".into());
    }

    let (num_str, unit) = size.split_at(split_index);
    let num: f64 = num_str.parse().map_err(|_| "Format numérique invalide")?;
    let unit = unit.trim();

    // Trouve l'index de l'unité
    let index = SIZES
        .iter()
        .position(|&u| u == unit)
        .ok_or_else(|| format!("Unité non supportée : {}", unit))?;

    // Calcule la valeur en octets
    let multiplier = 1024u64.pow(index as u32);
    let bytes = num * (multiplier as f64);

    if bytes < 0.0 || bytes > u64::MAX as f64 {
        return Err("La taille dépasse les limites de u64".into());
    }

    Ok(bytes.round() as u64)
}

#[cfg(test)]
pub mod test_parse {
    #[tokio::test]
    async fn test_extract_torrents() {
        // get file test.html
        let file = std::fs::read_to_string("test.html").unwrap();
        let result = super::extract_torrents(&file);
        let torrents = result.unwrap();
        for torrent in torrents {
            println!("{} : {}", torrent.name, torrent.get_url().unwrap());
        }
    }
}
