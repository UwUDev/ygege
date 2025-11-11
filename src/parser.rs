use crate::DOMAIN;
use chrono::NaiveDateTime;
use scraper::{Element, Html, Selector};
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
    pub info_url: String,
    pub link: String,
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
    if body.contains("Aucun résultat ") {
        debug!("No torrents found in the response");
        return Ok(Vec::new());
    }

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

        let info_url = columns[1]
            .select(&Selector::parse("a#torrent_name")?)
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| {
                let domain_lock = DOMAIN.lock().unwrap();
                let cloned_guard = domain_lock.clone();
                let domain = cloned_guard.as_str();
                drop(domain_lock);
                if !href.starts_with("http") {
                    format!("https://{}{}", domain, href)
                } else {
                    href.to_string()
                }
            });

        let link = match info_url.clone() {
            Some(url) => url,
            None => {
                warn!("Could not extract link for torrent id {}", id);
                String::new()
            }
        };

        let info_url = match info_url {
            Some(url) => {
                let url = url.split("/torrent/").collect::<Vec<&str>>()[1];
                format!("/torrent/info/{}", url)
            }
            None => {
                warn!("Could not extract info_url for torrent id {}", id);
                String::new()
            }
        };

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
            info_url,
            link,
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

fn parse_date_to_timestamp(date_str: &str) -> Option<i64> {
    NaiveDateTime::parse_from_str(date_str, "%d/%m/%Y %H:%M")
        .ok()
        .map(|dt| dt.and_utc().timestamp())
}

pub fn extract_partial_torrent_infos(document: &Html) -> Result<Value, Box<dyn std::error::Error>> {
    let section_selector = Selector::parse("section.content")?;
    let h2_selector = Selector::parse("h2")?;
    let table_selector = Selector::parse("table")?;
    let term_selector = Selector::parse("a.term")?;

    let mut keywords: Vec<String> = Vec::new();
    let mut seeders: usize = 0;
    let mut leechers: usize = 0;
    let mut completed: usize = 0;

    for section in document.select(&section_selector) {
        if let Some(h2) = section.select(&h2_selector).next() {
            let h2_text = h2.text().collect::<String>();

            if h2_text.contains("Téléchargement & Détails") {
                if let Some(table) = section.select(&table_selector).next() {
                    for row in table.select(&scraper::Selector::parse("tr")?) {
                        let cells: Vec<_> = row.select(&scraper::Selector::parse("td")?).collect();

                        if cells.len() > 0 {
                            let first_cell_text = cells[0].text().collect::<String>();

                            if first_cell_text.contains("Mots clés") {
                                // extract keywords from anchor tags with class "term"
                                for term_link in row.select(&term_selector) {
                                    let keyword =
                                        term_link.text().collect::<String>().trim().to_string();
                                    if !keyword.is_empty() {
                                        keywords.push(keyword);
                                    }
                                }
                            }
                        }

                        // handle rows with multiple key-value pairs (like Seeders/Leechers/Complétés)
                        // pattern: [key1, value1, key2, value2, key3, value3, ...]
                        let mut i = 0;
                        while i + 1 < cells.len() {
                            let key = cells[i].text().collect::<String>().trim().to_string();
                            let value_text =
                                cells[i + 1].text().collect::<String>().trim().to_string();

                            if key.contains("Seeders") {
                                let cleaned = value_text.replace(" ", "");
                                seeders = cleaned.parse().unwrap_or(0);
                            } else if key.contains("Leechers") {
                                let cleaned = value_text.replace(" ", "");
                                leechers = cleaned.parse().unwrap_or(0);
                            } else if key.contains("Complétés") {
                                let cleaned = value_text.replace(" ", "");
                                completed = cleaned.parse().unwrap_or(0);
                            }

                            i += 2;
                        }
                    }
                }
                break;
            }
        }
    }

    let mut created_at: u64 = 0;

    let tr_selector = Selector::parse("tr")?;
    for row in document.select(&tr_selector) {
        let cells: Vec<_> = row.select(&Selector::parse("td")?).collect();
        if cells.len() >= 2 {
            let first_cell_text = cells[0].text().collect::<String>();
            if first_cell_text.contains("Uploadé le") {
                let uploaded_date = cells[1].text().collect::<String>().trim().to_string();
                if let Some(date_part) = uploaded_date.split('(').next() {
                    let date_str = date_part.trim();
                    created_at = parse_date_to_timestamp(date_str).unwrap_or(0) as u64;
                }
                break;
            }
        }
    }

    //find a <a> in a <td> and the href contains "/profile/"
    let mut author_name: String = "Pirate Anonyme".to_string();
    let mut author_id: usize = 0;
    let a_selector = Selector::parse("a")?;

    'outer: for row in document.select(&tr_selector) {
        let cells: Vec<_> = row.select(&Selector::parse("td")?).collect();
        for cell in cells {
            for a in cell.select(&a_selector) {
                if let Some(href) = a.value().attr("href") {
                    if href.contains("/profile/") {
                        author_name = a.text().collect::<String>().trim().to_string();
                        if let Some(profile_pos) = href.find("/profile/") {
                            let after_profile = &href[profile_pos + "/profile/".len()..];
                            let id_str = after_profile.split('-').next().unwrap_or("");
                            author_id = id_str.parse::<usize>().unwrap_or(0);
                        }

                        break 'outer;
                    }
                }
            }
        }
    }

    let mut html_desc = String::new();
    let mut text_desc = String::new();

    // Extract description - the div right after <div class="description-header">
    let desc_header_selector = Selector::parse("div.description-header")?;
    if let Some(desc_header) = document.select(&desc_header_selector).next() {
        if let Some(parent_element) = desc_header.parent_element() {
            let div_selector = Selector::parse("div")?;
            let mut found_header = false;
            for div in parent_element.select(&div_selector) {
                if found_header {
                    html_desc = div.html();
                    text_desc = div.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    break;
                }

                if div.value().classes().any(|c| c == "description-header") {
                    found_header = true;
                }
            }
        }
    }

    Ok(serde_json::json!({
        "created_at": created_at,
        "completed": completed,
        "seed": seeders,
        "leech": leechers,
        "keywords": keywords,
        "author_name": author_name,
        "author_id": author_id,
        "html_description": html_desc,
        "text_description": text_desc,
    }))
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
