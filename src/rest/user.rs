use crate::config::Config;
use crate::rest::client_extractor::MaybeCustomClient;
use actix_web::{HttpResponse, get, web};

#[get("/user")]
pub async fn get_user_info(
    data: MaybeCustomClient,
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let user = crate::user::get_account(&data.client).await;
    // check if error is session expired
    if let Err(e) = &user {
        if e.to_string().contains("Session expired") && !data.is_custom {
            info!("Trying to renew session...");
            let new_client =
                crate::auth::login(config.username.as_str(), config.password.as_str(), true, config.flaresolverr_url.as_deref(), config.flaresolverr_downloads_dir.as_deref())
                    .await?;

            // Copy cookies from new client to shared client
            let domain = crate::DOMAIN.lock()?;
            let url = wreq::Url::parse(&format!("https://{}/", domain))?;
            if let Some(cookies) = new_client.get_cookies(&url) {
                data.shared_client.as_wreq_client().unwrap().clear_cookies();
                for cookie_str in cookies.to_str().unwrap_or("").split(';') {
                    let cookie_str = cookie_str.trim();
                    if cookie_str.is_empty() {
                        continue;
                    }
                    let parts: Vec<&str> = cookie_str.splitn(2, '=').collect();
                    if parts.len() != 2 {
                        continue;
                    }
                    let cookie = wreq::cookie::CookieBuilder::new(parts[0].trim(), parts[1].trim())
                        .domain(domain.as_str())
                        .path("/")
                        .http_only(true)
                        .secure(true)
                        .build();
                    data.shared_client.as_wreq_client().unwrap().set_cookie(&url, cookie);
                }
            }
            drop(domain);

            info!("Session renewed, retrying to get user info...");
            let user = crate::user::get_account(&new_client).await?;
            let json = serde_json::to_value(&user)?;
            let mut response = HttpResponse::Ok();
            if let Some(cookies) = data.cookies_header {
                response.insert_header(("X-Session-Cookies", cookies));
            }
            return Ok(response.json(json));
        }
    }

    let user = user?;
    let json = serde_json::to_value(&user)?;
    let mut response = HttpResponse::Ok();
    if let Some(cookies) = data.cookies_header {
        response.insert_header(("X-Session-Cookies", cookies));
    }
    Ok(response.json(json))
}
