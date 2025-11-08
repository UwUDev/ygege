use crate::DOMAIN;
use crate::utils::check_session_expired;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct UserAccount {
    pub username: String,
    pub rank: String,
    pub join_date: String,
    pub last_activity: String,
    pub torrents_count: u16,
    pub comments_count: u16,
    pub reputation_score: i32,
    pub passkey: String,
    pub uploaded: u128,
    pub downloaded: u128,
    pub ratio: f32,
    pub avatar_url: String,
    pub email: String,
    pub age: Option<u16>,
    pub gender: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
}

pub async fn get_account(client: &wreq::Client) -> Result<UserAccount, Box<dyn std::error::Error>> {
    debug!("Fetching user account information");

    let domain = {
        let domain_lock = DOMAIN.lock()?;
        domain_lock.clone()
    };

    let url = format!("https://{}/user/account", domain);
    let response = client.get(&url).send().await?;

    if check_session_expired(&response) {
        return Err("Session expired".into());
    }

    if !response.status().is_success() {
        return Err(format!("Failed to fetch account info: {}", response.status()).into());
    }

    let body = response.text().await?;
    let document = scraper::Html::parse_document(&body);
    let mut account = UserAccount::default();

    parse_base_infos(&document, &mut account)?;
    debug!("Got base infos");
    parse_tracker_stats(&document, &mut account)?;
    debug!("Got tracker stats");
    parse_misc_infos(&document, &mut account)?;
    debug!("Got misc infos");

    Ok(account)
}

fn parse_base_infos(
    document: &scraper::Html,
    account: &mut UserAccount,
) -> Result<(), Box<dyn std::error::Error>> {
    let table_selector =
        scraper::Selector::parse("table.detail-account").map_err(|_| "Invalid CSS selector")?;
    let table = document
        .select(&table_selector)
        .next()
        .ok_or("Account table not found")?;

    let td_selector = scraper::Selector::parse("td").map_err(|_| "Invalid CSS selector")?;

    for row in table.select(&scraper::Selector::parse("tr").unwrap()) {
        let cells: Vec<_> = row.select(&td_selector).collect();

        if let [key_cell, value_cell, ..] = cells.as_slice() {
            let key = key_cell.text().collect::<String>();
            let value = value_cell.text().collect::<String>().trim().to_string();

            if key.contains("Pseudo") {
                account.username = value.split('(').next().unwrap_or("").trim().to_string();
                account.rank = value
                    .split('(')
                    .nth(1)
                    .and_then(|s| s.strip_suffix(')'))
                    .unwrap_or("")
                    .trim()
                    .to_string();
            } else if key.contains("Date d'inscription") {
                account.join_date = value;
            } else if key.contains("Dernière activité") {
                account.last_activity = value;
            } else if key.contains("Mes torrents") {
                account.torrents_count = value.parse().unwrap_or(0);
            } else if key.contains("Commentaires") {
                account.comments_count = value.parse().unwrap_or(0);
            } else if key.contains("Réputation") {
                account.reputation_score = value.parse().unwrap_or(0);
            }
        }
    }
    Ok(())
}

fn parse_tracker_stats(
    document: &scraper::Html,
    account: &mut UserAccount,
) -> Result<(), Box<dyn std::error::Error>> {
    let section_selector = scraper::Selector::parse("section.content")?;
    let h2_selector = scraper::Selector::parse("h2")?;
    let table_selector = scraper::Selector::parse("table")?;

    for section in document.select(&section_selector) {
        if let Some(h2) = section.select(&h2_selector).next() {
            let h2_text = h2.text().collect::<String>();

            if h2_text.contains("Informations relatives au Tracker") {
                if let Some(table) = section.select(&table_selector).next() {
                    for row in table.select(&scraper::Selector::parse("tr")?) {
                        let cells: Vec<_> = row.select(&scraper::Selector::parse("td")?).collect();

                        if let [key_cell, value_cell, ..] = cells.as_slice() {
                            let key = key_cell.text().collect::<String>().trim().to_string();
                            let value = value_cell.text().collect::<String>().trim().to_string();

                            if key.contains("Passkey") {
                                account.passkey = value;
                            } else if key.contains("Qtt uploadée") {
                                account.uploaded = convert_size_to_bytes(&value.trim())?;
                            } else if key.contains("Qtt téléchargée") {
                                account.downloaded = convert_size_to_bytes(&value.trim())?;
                            }
                            let ratio = account.downloaded as f32;
                            if ratio == 0.0 {
                                account.ratio = 1000.0;
                            } else {
                                account.ratio = account.uploaded as f32 / ratio;
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    Ok(())
}

fn parse_misc_infos(
    document: &scraper::Html,
    account: &mut UserAccount,
) -> Result<(), Box<dyn std::error::Error>> {
    let img_selector = scraper::Selector::parse("img.card-img-top")?;
    if let Some(img) = document.select(&img_selector).next() {
        if let Some(src) = img.value().attr("src") {
            account.avatar_url = src.to_string();
        }
    }

    let input_selector = scraper::Selector::parse("input[name=\"email\"]")?;
    if let Some(input) = document.select(&input_selector).next() {
        if let Some(value) = input.value().attr("value") {
            account.email = value.to_string();
        }
    }

    let age_input_selector = scraper::Selector::parse("input[name=\"age\"]")?;
    if let Some(input) = document.select(&age_input_selector).next() {
        if let Some(value) = input.value().attr("value") {
            let age: u16 = value.parse().unwrap_or(0);
            if age > 0 {
                account.age = Some(age);
            }
        }
    }

    let gender_input_selector = scraper::Selector::parse("input[name=\"gender\"][checked]")?;
    if let Some(input) = document.select(&gender_input_selector).next() {
        if let Some(value) = input.value().attr("value") {
            account.gender = Some(value.to_string());
        }
    }

    let country_select_selector = scraper::Selector::parse("select[name=\"country\"]")?;
    if let Some(select) = document.select(&country_select_selector).next() {
        let option_selector = scraper::Selector::parse("option[selected]:not([disabled])")?;
        if let Some(option) = select.select(&option_selector).next() {
            let country = option.text().collect::<String>();
            account.country = Some(country.trim().to_string());
            if let Some(code) = option.value().attr("value") {
                account.country_code = Some(code.to_string());
            }
        }
    }

    Ok(())
}

const SIZES: [&str; 6] = ["o", "Ko", "Mo", "Go", "To", "Po"];

fn convert_size_to_bytes(size_str: &str) -> Result<u128, Box<dyn std::error::Error>> {
    for (i, &unit) in SIZES.iter().enumerate().rev() {
        if size_str.ends_with(unit) {
            let number_part = &size_str[..size_str.len() - unit.len()];
            let number: f64 = number_part.replace(",", ".").trim().parse()?;
            let bytes = number * 1024f64.powi(i as i32);
            return Ok(bytes as u128);
        }
    }

    Err(format!("Unknown size format: {}", size_str).into())
}

#[cfg(test)]
mod tests_user {
    use super::*;
    use crate::auth::login;
    use crate::config;
    use crate::domain::get_ygg_domain;

    #[tokio::test]
    async fn test_get_account() -> Result<(), Box<dyn std::error::Error>> {
        let domain = get_ygg_domain().await.unwrap_or_else(|_| {
            error!("Failed to get YGG domain");
            std::process::exit(1);
        });
        let mut domain_lock = DOMAIN.lock().unwrap();
        *domain_lock = domain.clone();
        drop(domain_lock);

        let config = config::load_config()?;

        std::fs::create_dir_all("sessions")?;
        let client = login(config.username.as_str(), config.password.as_str(), true).await?;

        let account = get_account(&client).await?;
        println!("Account : {:?}", account);
        Ok(())
    }
}
