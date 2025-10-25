use wreq::dns::{Addrs, Name, Resolve};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use trust_dns_resolver::{
    TokioAsyncResolver,
    config::{ResolverConfig, ResolverOpts},
};

pub struct AsyncCloudflareResolverAdapter {
    resolver: TokioAsyncResolver,
}

impl AsyncCloudflareResolverAdapter {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let resolver =
            TokioAsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default());

        Ok(AsyncCloudflareResolverAdapter { resolver })
    }
}

impl Resolve for AsyncCloudflareResolverAdapter {
    fn resolve(
        &self,
        domain: Name,
    ) -> Pin<Box<dyn Future<Output = Result<Addrs, Box<dyn std::error::Error + Send + Sync>>> + Send>>
    {
        let resolver = self.resolver.clone();
        let domain_str = domain.as_str().to_string();

        Pin::from(Box::new(async move {
            match resolver.lookup_ip(domain_str).await {
                Ok(lookup) => {
                    let socket_addrs: Vec<SocketAddr> =
                        lookup.iter().map(|ip| SocketAddr::new(ip, 443)).collect();

                    Ok(Box::new(socket_addrs.into_iter()) as Addrs)
                }
                Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
            }
        }))
    }
}
