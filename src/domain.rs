use crate::resolver::AsyncCloudflareResolverAdapter;
use std::sync::Arc;
use wreq::Client;
use wreq_util::{Emulation, EmulationOS, EmulationOption};

pub async fn get_ygg_domain() -> Result<String, Box<dyn std::error::Error>> {
    let emu = EmulationOption::builder()
        .emulation(Emulation::Chrome132) // no H3 check on CF before 133
        .emulation_os(EmulationOS::Windows)
        .build();

    // les fameux DNS menteurs
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

    // get https://ygg.to and get the redirect location domain
    debug!("Getting YGG current domain");

    let start = std::time::Instant::now();

    let response = client.get("https://yggtorrent.org").send().await?;
    let location = response
        .headers()
        .get("location")
        .ok_or("No location header")?;
    let location_str = location.to_str()?;
    let domain = location_str.split('/').nth(2).ok_or("No domain found")?;

    let stop = std::time::Instant::now();

    debug!(
        "Found current YGG domain: {} in {:?}",
        domain,
        stop.duration_since(start)
    );
    Ok(domain.to_string())
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
