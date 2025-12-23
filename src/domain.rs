use crate::resolver::AsyncCloudflareResolverAdapter;
use std::sync::{Arc, OnceLock};
use wreq::Client;
use wreq_util::{Emulation, EmulationOS, EmulationOption};

const CURRENT_REDIRECT_DOMAINS: [&str; 4] =
    ["yggtorrent.ch", "ygg.to", "yggtorrent.to", "yggtorrent.is"];

pub static OWN_IP: OnceLock<String> = OnceLock::new();

pub async fn get_ygg_domain() -> Result<String, Box<dyn std::error::Error>> {
    let emu = EmulationOption::builder()
        .emulation(Emulation::Chrome132) // no H3 check on CF before 133
        .emulation_os(EmulationOS::Windows)
        .build();

    // les fameux DNS menteurs
    let cloudflare_dns = Arc::new(AsyncCloudflareResolverAdapter::new()?);

    debug!("Getting YGG current domain by trying all base domains in parallel");

    let start = std::time::Instant::now();

    let mut tasks = Vec::new();
    for &base_domain in &CURRENT_REDIRECT_DOMAINS {
        let cloudflare_dns = Arc::clone(&cloudflare_dns);
        let emu = emu.clone();
        let task = tokio::spawn(async move {
            let client = Client::builder()
                .emulation(emu)
                .gzip(true)
                .deflate(true)
                .brotli(true)
                .zstd(true)
                .cookie_store(true)
                .dns_resolver(cloudflare_dns)
                .build()?;

            let response = client
                .get(format!("https://{}", base_domain))
                .send()
                .await?;

            let domain = if let Some(location) = response.headers().get("location") {
                let location_str = location.to_str()?;
                location_str
                    .split('/')
                    .nth(2)
                    .ok_or("No domain found")?
                    .to_string()
            } else {
                base_domain.to_string()
            };

            Ok::<String, Box<dyn std::error::Error + Send + Sync>>(domain)
        });
        tasks.push(task);
    }

    let mut last_error = None;
    while !tasks.is_empty() {
        let (result, _idx, remaining) = futures::future::select_all(tasks).await;
        tasks = remaining;

        match result {
            Ok(Ok(domain)) => {
                let stop = std::time::Instant::now();
                debug!(
                    "Found current YGG domain: {} in {:?}",
                    domain,
                    stop.duration_since(start)
                );
                return Ok(domain);
            }
            Ok(Err(e)) => {
                debug!("Failed to get domain from one source: {}", e);
                last_error = Some(e);
            }
            Err(e) => {
                debug!("Task panicked: {}", e);
                last_error = Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "All domain checks failed".into()))
}

pub async fn get_own_ip() -> Result<String, Box<dyn std::error::Error>> {
    let emu = EmulationOption::builder()
        .emulation(Emulation::Chrome132) // no H3 check on CF before 133
        .emulation_os(EmulationOS::Windows)
        .build();

    let cloudflare_dns = Arc::new(AsyncCloudflareResolverAdapter::new()?);

    let client = Client::builder()
        .emulation(emu)
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .dns_resolver(cloudflare_dns)
        .build()?;

    let response = client
        .get("https://api64.ipify.org?format=text")
        .send()
        .await?;

    let ip = response.text().await?;
    Ok(ip)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ygg_domain() {
        let result = get_ygg_domain().await;
        assert!(result.is_ok());
        let domain = result.unwrap();
        assert!(!domain.is_empty());
        println!("YGG Domain: {}", domain);
    }
}
