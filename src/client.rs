use crate::resolver::AsyncCloudflareResolverAdapter;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use wreq::Client;
use wreq_util::{Emulation, EmulationOS, EmulationOption};

fn build_emulation() -> EmulationOption {
    EmulationOption::builder()
        .emulation(Emulation::Chrome132)
        .emulation_os(EmulationOS::Windows)
        .build()
}

fn build_dns_resolver() -> Result<Arc<AsyncCloudflareResolverAdapter>, Box<dyn std::error::Error>> {
    Ok(Arc::new(AsyncCloudflareResolverAdapter::new()?))
}

pub fn build_client(domain: &str, leaked_ip: &str) -> Result<Client, wreq::Error> {
    Client::builder()
        .emulation(build_emulation())
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .dns_resolver(build_dns_resolver().unwrap())
        .cert_verification(false)
        .verify_hostname(false)
        .resolve(
            domain,
            SocketAddr::new(IpAddr::from_str(leaked_ip).unwrap(), 443),
        )
        .build()
}

pub fn build_simple_client() -> Result<Client, wreq::Error> {
    Client::builder()
        .emulation(build_emulation())
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .dns_resolver(build_dns_resolver().unwrap())
        .build()
}