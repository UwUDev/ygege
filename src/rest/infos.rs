use crate::config::Config;
use crate::search::{Order, Sort, search};
use crate::{DOMAIN, resolver};
use actix_web::{HttpResponse, get, web};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use wreq::Client;
use wreq::dns::Resolve;

#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[get("/status")]
pub async fn status_check(data: web::Data<Client>, config: web::Data<Config>) -> HttpResponse {
    let domain_lock = DOMAIN.lock().unwrap();
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    let search = search(
        &data,
        &config,
        Some("Vaiana"),
        None,
        None,
        None,
        Some(Sort::Seed),
        Some(Order::Ascending),
        None,
    )
    .await;

    let auth: &str;
    let search_status: &str;
    let parsing: &str;
    match search {
        Ok(torrents) => {
            auth = "authenticated";
            search_status = "ok";
            if torrents.is_empty() {
                parsing = "failed";
            } else {
                parsing = "ok";
            }
        }
        Err(e) => {
            if e.to_string().contains("Session expired") {
                auth = "not_authenticated";
                search_status = "ok";
                parsing = "n/a";
            } else {
                error!("Status check auth error: {}", e);
                auth = "auth_error";
                search_status = "failed";
                parsing = "n/a";
            }
        }
    }

    let user = crate::user::get_account(&data).await;
    let user_status = match user.is_ok() {
        true => "ok",
        false => "failed",
    };

    // DNS lookup to check if domain resolves via 1.1.1.1
    let resolver = resolver::AsyncCloudflareResolverAdapter::new().unwrap();
    let mut domain_ping = "unreachable";
    let dns_lookup = match resolver
        .resolve(wreq::dns::Name::from_str(domain).unwrap())
        .await
    {
        Ok(ip) => {
            // convert wreq::dns::resolve::Addrs to core::net::ip_addr::IpAddr
            let ip = ip
                .into_iter()
                .next()
                .and_then(|socket_addr| Some(socket_addr.ip()));

            if ip.is_some() {
                let ip_addr = ip.unwrap();
                info!("Resolved IP: {}", ip_addr);

                let socket_addr = SocketAddr::new(ip_addr, 443);
                domain_ping =
                    match timeout(Duration::from_secs(5), TcpStream::connect(socket_addr)).await {
                        Ok(Ok(_)) => {
                            info!("TCP connection to {} successful", socket_addr);
                            "reachable"
                        }
                        Ok(Err(e)) => {
                            error!("TCP connection failed: {}", e);
                            "unreachable"
                        }
                        Err(_) => {
                            error!("TCP connection timeout");
                            "timeout"
                        }
                    };
            }

            "resolves"
        }
        Err(_) => "does_not_resolve",
    };

    let tmdb = match config.tmdb_token.is_some() {
        true => "enabled",
        false => "disabled",
    };

    let status = serde_json::json!({
        "domain": domain,
        "auth": auth,
        "search": search_status,
        "user_info": user_status,
        "domain_reachability": domain_ping,
        "domain_dns": dns_lookup,
        "parsing": parsing,
        "tmdb_integration": tmdb,
    });

    HttpResponse::Ok().json(status)
}
