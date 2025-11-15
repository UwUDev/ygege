use wreq::Client;

pub async fn get_account_username(token: &String) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Fetching TMDB account username");
    let client = Client::new();
    let response = client
        .get("https://api.themoviedb.org/3/account")
        .header("Authorization", format!("Bearer {}", token))
        .header("accept", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch TMDB account info: {}", response.status()).into());
    }

    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;

    if let Some(username) = json.get("username").and_then(|u| u.as_str()) {
        Ok(username.to_string())
    } else {
        Err("Username not found in TMDB response".into())
    }
}

pub async fn get_queries(
    id: u32,
    token: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    debug!("Fetching TMDB titles for ID: {}", id);
    let client = Client::new();
    let url = format!("https://api.themoviedb.org/3/movie/{}", id);
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("accept", "application/json")
        .send()
        .await?;

    if !response.status().is_success() {
        // 404
        if response.status().as_u16() == 404 {
            return Err("TMDB movie not found".into());
        }

        return Err(format!("Failed to fetch TMDB movie info: {}", response.status()).into());
    }

    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;

    let year = json
        .get("release_date")
        .and_then(|rd| rd.as_str())
        .and_then(|date_str| date_str.split('-').next())
        .and_then(|year_str| year_str.parse::<u32>().ok())
        .unwrap_or(0);

    let original_title = json
        .get("original_title")
        .and_then(|ot| ot.as_str())
        .ok_or("Original title not found in TMDB response")?
        .to_string();

    let title = json
        .get("title")
        .and_then(|t| t.as_str())
        .ok_or("Title not found in TMDB response")?
        .to_string();

    let mut titles = Vec::new();
    if original_title != title {
        titles.push(format!("{} {}", original_title, year));
    }
    titles.push(format!("{} {}", title, year));

    let alt_url = format!(
        "https://api.themoviedb.org/3/movie/{}/alternative_titles",
        id
    );
    let alt_response = client
        .get(&alt_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("accept", "application/json")
        .send()
        .await?;

    if !alt_response.status().is_success() {
        return Err(format!(
            "Failed to fetch TMDB alternative titles: {}",
            alt_response.status()
        )
        .into());
    }

    let body = alt_response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    if let Some(titles_array) = json.get("titles").and_then(|t| t.as_array()) {
        for title_entry in titles_array {
            if let Some(iso_3166_1) = title_entry.get("iso_3166_1").and_then(|c| c.as_str()) {
                if iso_3166_1 == "FR"
                    || iso_3166_1 == "US"
                    || iso_3166_1 == "GB"
                    || iso_3166_1 == "EN"
                {
                    if let Some(title) = title_entry.get("title").and_then(|t| t.as_str()) {
                        if !titles.contains(&title.to_string()) {
                            titles.push(format!("{} {}", title, year));
                        }
                    }
                }
            }
        }
    }

    Ok(titles)
}
