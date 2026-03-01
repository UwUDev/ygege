use crate::ygg_client::YggClient;
use crate::{DOMAIN, LOGIN_PAGE};

pub fn check_session_expired(status: u16, url: &str) -> bool {
    if status == 307 || status == 302 {
        warn!("Session expired...");
        return true;
    }

    if url.contains(LOGIN_PAGE) {
        warn!("Session expired...");
        return true;
    }

    false
}

pub async fn get_remaining_downloads(
    client: &YggClient,
) -> Result<u16, Box<dyn std::error::Error>> {
    debug!("Fetching remaining downloads information");

    let domain = {
        let domain_lock = DOMAIN.lock()?;
        domain_lock.clone()
    };

    let url = format!(
        "https://{}//torrent/application/windows/316475-microsoft-toolkit-v2-6-4-activateur-office-2016---2019-windows-10",
        domain
    );
    let response = client.get(&url).await?;

    if check_session_expired(response.status, &response.url) {
        return Err("Session expired".into());
    }

    let body = response.body;
    if body.contains("Limite atteinte") {
        return Ok(0);
    }

    let document = scraper::Html::parse_document(&body);

    let selector = scraper::Selector::parse("small[style=\"color: #888;\"]")
        .map_err(|_| "Invalid CSS selector")?;
    let small = document.select(&selector).next();

    let small = match small {
        Some(s) => s,
        None => return Ok(u16::MAX),
    };

    let strong_selector = scraper::Selector::parse("strong").map_err(|_| "Invalid CSS selector")?;
    let strong = small
        .select(&strong_selector)
        .next()
        .ok_or("Strong tag not found in remaining downloads info")?;
    let text = strong.text().collect::<String>();
    // split by /
    let parts: Vec<&str> = text.split('/').collect();
    if parts.len() != 2 {
        return Err("Invalid remaining downloads format".into());
    }

    let remaining: u16 = parts[0].trim().parse()?;
    Ok(remaining)
}
