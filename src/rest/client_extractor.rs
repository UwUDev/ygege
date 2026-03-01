use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use wreq::{Client, Url};
use wreq_util::{Emulation, EmulationOS, EmulationOption};

use crate::DOMAIN;
use crate::domain::get_leaked_ip;
use crate::resolver::AsyncDNSResolverAdapter;
use crate::ygg_client::YggClient;

pub struct MaybeCustomClient {
    pub client: YggClient,
    pub is_custom: bool,
    pub cookies_header: Option<String>,
    pub shared_client: web::Data<YggClient>,
}

#[derive(Debug, serde::Deserialize)]
struct CookieQuery {
    cookie: Option<String>,
}

impl FromRequest for MaybeCustomClient {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            // Get the shared client from app data
            let shared_client = req
                .app_data::<web::Data<YggClient>>()
                .ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("Client not found in app data")
                })?
                .clone();

            // get query param
            let query = web::Query::<CookieQuery>::from_query(req.query_string())
                .ok()
                .and_then(|q| q.into_inner().cookie);

            if let Some(cookie_string) = query {
                // custom client
                match create_client_with_cookies(&cookie_string).await {
                    Ok(client) => {
                        let cookies_header = {
                            let domain = DOMAIN.lock().map_err(|_| {
                                actix_web::error::ErrorInternalServerError("Failed to lock domain")
                            })?;
                            let url = Url::parse(&format!("https://{}/", domain)).map_err(|_| {
                                actix_web::error::ErrorInternalServerError("Failed to parse URL")
                            })?;
                            client
                                .get_cookies(&url)
                                .and_then(|h| h.to_str().ok().map(|s| s.to_string()))
                        };

                        Ok(MaybeCustomClient {
                            client: YggClient::Direct(client),
                            is_custom: true,
                            cookies_header,
                            shared_client,
                        })
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to create custom client: {}, falling back to default",
                            e
                        );
                        // fallback to the default client
                        Ok(MaybeCustomClient {
                            client: shared_client.get_ref().clone(),
                            is_custom: false,
                            cookies_header: None,
                            shared_client,
                        })
                    }
                }
            } else {
                // default client
                Ok(MaybeCustomClient {
                    client: shared_client.get_ref().clone(),
                    is_custom: false,
                    cookies_header: None,
                    shared_client,
                })
            }
        })
    }
}

async fn create_client_with_cookies(
    cookie_string: &str,
) -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let emu = EmulationOption::builder()
        .emulation(Emulation::Chrome132)
        .emulation_os(EmulationOS::Windows)
        .build();

    let domain = {
        let domain_lock = DOMAIN
            .lock()
            .map_err(|e| format!("Failed to lock domain: {}", e))?;
        domain_lock.clone()
    };

    let leaked_ip = get_leaked_ip()
        .await
        .map_err(|e| format!("Failed to get leaked IP: {}", e))?;

    let client = Client::builder()
        .emulation(emu)
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .dns_resolver(Arc::new(
            AsyncDNSResolverAdapter::new()
                .map_err(|e| format!("Failed to create DNS resolver: {}", e))?,
        ))
        .cert_verification(false)
        .verify_hostname(false)
        .resolve(
            &domain,
            SocketAddr::new(IpAddr::from_str(leaked_ip.as_str())?, 443),
        )
        .build()?;

    let cookies: Vec<&str> = cookie_string.split(';').collect();
    for cookie in cookies {
        let cookie = cookie.trim();
        if cookie.is_empty() {
            continue;
        }
        let parts: Vec<&str> = cookie.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }
        let name = parts[0].trim();
        let value = parts[1].trim();
        let cookie = wreq::cookie::CookieBuilder::new(name, value)
            .domain(&domain)
            .path("/")
            .http_only(true)
            .secure(true)
            .build();
        let url = Url::parse(&format!("https://{}/", domain))?;
        client.set_cookie(&url, cookie);
    }

    log::debug!("Created custom client with injected cookies");

    Ok(client)
}