use crate::DOMAIN;
use crate::auth::login;
use crate::config::Config;
use actix_web::{HttpRequest, HttpResponse, get, web};

#[get("/auth")]
pub async fn auth(req_data: HttpRequest, config: web::Data<Config>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let query = req_data.query_string();
    let qs = qstring::QString::from(query);
    let user: String = match qs.get("user") {
        Some(u) => u.to_string(),
        None => return Ok(HttpResponse::BadRequest().body("Missing 'user' parameter")),
    };
    let pass: String = match qs.get("pass") {
        Some(p) => p.to_string(),
        None => return Ok(HttpResponse::BadRequest().body("Missing 'pass' parameter")),
    };

    let client = login(&user, &pass, false, config.flaresolverr_url.as_deref(), config.flaresolverr_downloads_dir.as_deref()).await;
    match client {
        Ok(client) => {
            let domain_lock = DOMAIN.lock()?;
            let cloned_guard = domain_lock.clone();
            let domain = cloned_guard.as_str();
            drop(domain_lock);

            let url = wreq::Url::parse(&format!("https://{}/", domain)).unwrap();
            let cookies = client.get_cookies(&url);
            match cookies {
                Some(cookies_header) => {
                    let cookie_str = cookies_header.to_str().unwrap_or("").to_string();
                    log::info!("Login successful for user {}: cookies={}", user, cookie_str);
                    let mut response = HttpResponse::Ok();
                    response.insert_header(("X-Session-Cookies", cookie_str.clone()));
                    Ok(response.body(cookie_str))
                }
                None => Ok(HttpResponse::Ok().body("Login successful, but no cookies found")),
            }
        }
        Err(e) => {
            log::error!("Login failed for user {}: {}", user, e);
            Ok(HttpResponse::Unauthorized().body(format!("Login failed: {}", e)))
        }
    }
}
