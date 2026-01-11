use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use trust_dns_resolver::{
    TokioAsyncResolver,
    config::{ResolverConfig, ResolverOpts},
};
use wreq::dns::{Addrs, Name, Resolve};

pub struct AsyncDNSResolverAdapter {
    system_resolver: TokioAsyncResolver,
    cloudflare_resolver: TokioAsyncResolver,
}

impl AsyncDNSResolverAdapter {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let system_resolver = TokioAsyncResolver::tokio_from_system_conf()?;
        let cloudflare_resolver =
            TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default());

        Ok(AsyncDNSResolverAdapter {
            system_resolver,
            cloudflare_resolver,
        })
    }
}

impl Resolve for AsyncDNSResolverAdapter {
    fn resolve(
        &self,
        domain: Name,
    ) -> Pin<Box<dyn Future<Output = Result<Addrs, Box<dyn std::error::Error + Send + Sync>>> + Send>>
    {
        let system_resolver = self.system_resolver.clone();
        let cloudflare_resolver = self.cloudflare_resolver.clone();
        let domain_str = domain.as_str().to_string();

        Pin::from(Box::new(async move {
            // Try system DNS first
            match system_resolver.lookup_ip(&domain_str).await {
                Ok(lookup) => {
                    let socket_addrs: Vec<SocketAddr> =
                        lookup.iter().map(|ip| SocketAddr::new(ip, 443)).collect();
                    Ok(Box::new(socket_addrs.into_iter()) as Addrs)
                }
                Err(_) => {
                    // Fall back to Cloudflare DNS if system DNS fails
                    match cloudflare_resolver.lookup_ip(&domain_str).await {
                        Ok(lookup) => {
                            let socket_addrs: Vec<SocketAddr> =
                                lookup.iter().map(|ip| SocketAddr::new(ip, 443)).collect();
                            Ok(Box::new(socket_addrs.into_iter()) as Addrs)
                        }
                        Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
                    }
                }
            }
        }))
    }
}
