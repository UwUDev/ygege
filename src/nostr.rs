use crate::categories::nostr_tag_to_cat_id;
use crate::parser::Torrent;
use futures::{SinkExt, StreamExt};
use secp256k1::{Secp256k1, XOnlyPublicKey};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use futures_util::stream::FuturesUnordered;
use tokio_socks::tcp::Socks5Stream;
use tokio_tungstenite::{client_async_tls, connect_async, tungstenite::Message};
use urlencoding::encode;
use uuid::Uuid;

// Ygg migration pub key
const ALLOWED_PUBKEY: &str = "6aeb55064ea8b777591055e5704612e0e863fcc00bb211741781be299473c54e";

/// All known Nostr relays hosting NIP-35 torrent events.
pub const KNOWN_CLEARNET_RELAYS: &[&str] = &[
    "wss://relay.ygg.gratis",
    "wss://u2prelais.eliottb.dev",
    "wss://u2p.my-p2p.com",
    "wss://u2p.notrusted.me",
    "wss://u2p.marrant.fun",
    "wss://u2p.anhkagi.net",
];

pub const KNOWN_ONION_RELAYS: &[&str] = &[
    "ws://ehgh3n5sv6ksl2gfl7q36mhg55wb6xn2rynzj4fko6dnhssfe4zawtad.onion",
    "ws://ibkeeavvjqrkpxd2vfyonz7hvm7jmjca5xswge7bbif5wdhdb5iq5ead.onion",
    "ws://ayxzs7ln5hklavucyp5rm2pwqjvtfxgeip22qkygcjmvlpwke55b3myd.onion",
    "ws://n5prt5v7sk7bjbgzouz7pgetcfjsyvobnhgpoxo2j3yot4th3slahlad.onion",
    "ws://5on77qzcevmbiwdb7ixtu6biyomaulugxlqhwhfq2jmkszz63u7v26id.onion",
    "ws://gc6kygpxpfcbikv7kgsgwaq5gllcjgcqtktaquzrre4sbxhk7mmezsyd.onion",
    "ws://dsw3jecy6ls4voaq5jhukiyhmeutkksgsdpwts52jqgveojyyf57g5yd.onion",
    "ws://nbgni4suldceu4igjcohkd6kytnk5iirjo4qj5xw3j2czm5mzgzlb3yd.onion",
    "ws://tfhjdlp5pmlafn6zrmcssf4kjwye6g4dlw32d455vqujpqrlfccz44yd.onion",
    "ws://7mymq7lp5s2cfihnoqc7goj5uqtualpwda2qllutidekta2wuebmlxyd.onion",
    "ws://fss2g3e2nlc654f6bme7gdtktlxic5fttairrhndxntemwkt6aqac2yd.onion",
    "ws://iz3vvv47hjzani62kr22bj4uiicpqqy7shizz242tldwis6xqv2gdwqd.onion",
    "ws://cpw7arhzquvkk5yn6ce4pisgpnspidxy76caozphygsdry6orrmiueid.onion",
    "ws://3iplzxfek5vreyopioarb2blg2zzauftwv7s63wmeu5piqkhniftlfid.onion",
    "ws://ac3pr3hfndqm5yb2pvhm4djim65ol2ksssa6fff2n74k2coxfzclpxqd.onion",
    "ws://docxxx4eyz23l23vaxzckswkelj3tlqqb76jnqe6sd7kipre4yqlzhyd.onion",
    "ws://hzotrtsrraa4n5gdyvkqz2jmwtdho672o7ry7qak4ty6mpbdvqdb6rid.onion",
    "ws://cfivia2kva5jbivxvmjqztoj7beq3y3egep7cmq5iqiocmetcb4hdbqd.onion",
    "ws://bsawwivx7ohk6dlnw4cliif346yf3upter3wnwfxf7f7bxpvb5o7ewid.onion",
    "ws://zkw53xww3kxwdynt223rpvofgaslefcjmn7fknyqvrwxzu7owb3ujpyd.onion",
    "ws://vkjh7ohfbnzpxthxignkwrz5bomr36drepbl5m2uoamoftfldjb6v4yd.onion",
    "ws://4w5t5wrtko2vw46vmgu4elyn4yttrb47nmfn7nkxqql23di2ktudtvad.onion",
    "ws://4oikbtj62fyf4cymkc22ih4oouremp7cnw6x5rulnvgafvg3mnwfy7id.onion",
];

pub async fn rank_relays(use_tor: bool, tor_proxy: Option<&str>) -> Vec<String> {
    let timeout_dur = match tor_proxy {
        Some(_) if use_tor => Duration::from_secs(20),
        _ => Duration::from_secs(5),
    };

    let relays_to_probe = match use_tor {
        true => KNOWN_ONION_RELAYS,
        false => KNOWN_CLEARNET_RELAYS,
    };

    let mut futures_set: FuturesUnordered<_> = relays_to_probe
        .iter()
        .map(|url| {
            let url = url.to_string();
            let proxy = tor_proxy.map(|s| s.to_string());
            async move {
                let latency = probe_relay(&url, timeout_dur, use_tor, proxy.as_deref()).await;
                match latency {
                    Some(d) => info!("Relay {} responded in {}ms", url, d.as_millis()),
                    None => warn!("Relay {} failed probe", url),
                }
                (url, latency)
            }
        })
        .collect();

    let mut results: Vec<(String, Duration)> = Vec::with_capacity(5);

    while let Some((url, latency)) = futures_set.next().await {
        if let Some(d) = latency {
            results.push((url, d));
            if results.len() >= 5 {
                break;
            }
        }
    }

    results.sort_by_key(|(_, d)| *d);
    results.into_iter().map(|(url, _)| url).collect()
}

async fn probe_relay(url: &str, timeout_dur: Duration, use_tor: bool, tor_proxy: Option<&str>) -> Option<Duration> {
    let start = Instant::now();

    let sub_id = Uuid::new_v4().to_string();
    let req = json!(["REQ", sub_id, {
        "kinds": [2003],
        "search": "vaiana",
        "limit": 1
    }]);
    let req_text = req.to_string();

    if use_tor {
        let proxy_addr = tor_proxy.unwrap_or("127.0.0.1:9050");
        let parsed = url::Url::parse(url).ok()?;
        let host = parsed.host_str()?.to_string();
        let port = parsed.port().unwrap_or(80);

        let socks_stream = match tokio::time::timeout(
            timeout_dur,
            Socks5Stream::connect(proxy_addr, (host.as_str(), port)),
        )
        .await
        {
            Ok(Ok(s)) => s,
            Ok(Err(e)) => {
                debug!("Probe Tor connect error {}: {}", url, e);
                return None;
            }
            Err(_) => {
                debug!("Probe Tor connect timeout {}", url);
                return None;
            }
        };

        let ws = match tokio::time::timeout(timeout_dur.saturating_sub(start.elapsed()), client_async_tls(url, socks_stream)).await {
            Ok(Ok((ws, _))) => ws,
            Ok(Err(e)) => {
                debug!("Probe WS handshake error {}: {}", url, e);
                return None;
            }
            Err(_) => {
                debug!("Probe WS handshake timeout {}", url);
                return None;
            }
        };

        let (mut write, mut read) = ws.split();
        if write.send(Message::Text(req_text.into())).await.is_err() {
            return None;
        }
        let remaining = timeout_dur.saturating_sub(start.elapsed());
        let result = tokio::time::timeout(remaining, async {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let Ok(parsed) = serde_json::from_str::<Value>(&text) else { continue };
                        let Some(arr) = parsed.as_array() else { continue };
                        match arr.first().and_then(|v| v.as_str()) {
                            Some("EVENT") | Some("EOSE") => return Some(start.elapsed()),
                            _ => continue,
                        }
                    }
                    Ok(Message::Close(_)) | Err(_) => return None,
                    _ => continue,
                }
            }
            None
        }).await;
        let _ = write.close().await;
        result.ok().flatten()
    } else {
        let ws = match tokio::time::timeout(timeout_dur, connect_async(url)).await {
            Ok(Ok((ws, _))) => ws,
            Ok(Err(e)) => {
                debug!("Probe connect error {}: {}", url, e);
                return None;
            }
            Err(_) => {
                debug!("Probe connect timeout {}", url);
                return None;
            }
        };

        let (mut write, mut read) = ws.split();
        if write.send(Message::Text(req_text.into())).await.is_err() {
            return None;
        }
        let remaining = timeout_dur.saturating_sub(start.elapsed());
        let result = tokio::time::timeout(remaining, async {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let Ok(parsed) = serde_json::from_str::<Value>(&text) else { continue };
                        let Some(arr) = parsed.as_array() else { continue };
                        match arr.first().and_then(|v| v.as_str()) {
                            Some("EVENT") | Some("EOSE") => return Some(start.elapsed()),
                            _ => continue,
                        }
                    }
                    Ok(Message::Close(_)) | Err(_) => return None,
                    _ => continue,
                }
            }
            None
        }).await;
        let _ = write.close().await;
        result.ok().flatten()
    }
}

pub struct NostrClient {
    relays: Arc<Mutex<Vec<String>>>,
    use_tor: bool,
    tor_proxy: Option<String>,
}

impl NostrClient {
    pub fn new(relays: Vec<String>, use_tor: bool, tor_proxy: Option<String>) -> Self {
        NostrClient {
            relays: Arc::new(Mutex::new(relays)),
            use_tor,
            tor_proxy,
        }
    }

    pub fn relays(&self) -> Vec<String> {
        self.relays.lock().unwrap().clone()
    }

    async fn remove_first_relay(&self) -> bool {
        {
            let mut relays = self.relays.lock().unwrap();
            if !relays.is_empty() {
                let dead = relays.remove(0);
                warn!("Removed dead relay: {}", dead);
            }
            if !relays.is_empty() {
                return true;
            }
        }

        warn!("All relays died, re-ranking...");
        let fresh = rank_relays(self.use_tor, self.tor_proxy.as_deref()).await;
        if fresh.is_empty() {
            error!("Re-ranking returned no reachable relays, try again later. Exiting.");
            std::process::exit(1);
        }
        info!(
            "Re-ranked relay order: {}",
            fresh
                .iter()
                .enumerate()
                .map(|(i, u)| format!("{}. {}", i + 1, u))
                .collect::<Vec<_>>()
                .join(", ")
        );
        *self.relays.lock().unwrap() = fresh;
        true
    }

    /// Search for NIP-35 (Kind 2003) torrent events on the relay.
    /// Uses NIP-50 full-text search when `query` is non-empty.
    /// Optionally filters by a single `#t` tag (category).
    pub async fn search(
        &self,
        query: &str,
        tag_filter: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Torrent>, Box<dyn std::error::Error + Send + Sync>> {
        let sub_id = Uuid::new_v4().to_string();

        let mut filter = json!({
            "kinds": [2003],
            "limit": limit
        });

        if !query.is_empty() {
            filter["search"] = json!(query);
        }

        if let Some(tag) = tag_filter {
            filter["#t"] = json!([tag]);
        }

        let req = json!(["REQ", sub_id, filter]);

        debug!(
            "Nostr REQ: {} from relay {}",
            req.to_string().chars().take(200).collect::<String>(),
            self.relays
                .lock()
                .unwrap()
                .first()
                .cloned()
                .unwrap_or_default()
        );

        let events = self.send_req(&sub_id, req).await?;
        let torrents = events.into_iter().filter_map(parse_nip35_event).collect();
        Ok(torrents)
    }

    /// Fetch a single event by ID (used by /torrent/{id}).
    pub async fn get_event(
        &self,
        event_id: &str,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let sub_id = Uuid::new_v4().to_string();

        let filter = json!({
            "kinds": [2003],
            "ids": [event_id],
            "limit": 1
        });

        let req = json!(["REQ", sub_id, filter]);
        let events = self.send_req(&sub_id, req).await?;
        Ok(events.into_iter().next())
    }

    /// Try the best relay. On failure, remove it and try the next one.
    /// Re-ranks if all relays are consumed.
    async fn send_req(
        &self,
        sub_id: &str,
        req: Value,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
        loop {
            let relay_url = {
                self.relays
                    .lock()
                    .unwrap()
                    .first()
                    .cloned()
                    .unwrap_or_default()
            };

            if relay_url.is_empty() {
                // shouldn't happen
                self.remove_first_relay().await;
                continue;
            }

            match self.send_req_to(&relay_url, sub_id, &req).await {
                Ok(events) => {
                    debug!("Got {} events from {}", events.len(), relay_url);
                    return Ok(events);
                }
                Err(e) => {
                    warn!("Relay {} failed: {}", relay_url, e);
                    self.remove_first_relay().await;
                }
            }
        }
    }

    /// Open a WebSocket to a single relay, send a REQ, collect EVENTs until EOSE or timeout.
    async fn send_req_to(
        &self,
        relay_url: &str,
        sub_id: &str,
        req: &Value,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("Connecting to relay: {}", relay_url);

        let url = url::Url::parse(relay_url)?;
        let host = url.host_str().unwrap().to_string();
        let port = url.port().unwrap_or(80);

        let req_text = req.to_string();

        macro_rules! collect_events {
            ($write:expr, $read:expr) => {{
                let mut write = $write;
                let mut read = $read;
                write.send(Message::Text(req_text.clone().into())).await?;

                let mut events: Vec<Value> = Vec::new();

                let timeout = tokio::time::timeout(Duration::from_secs(30), async {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                let Ok(parsed) = serde_json::from_str::<Value>(&text) else {
                                    continue;
                                };
                                let arr = match parsed.as_array() {
                                    Some(a) => a,
                                    None => continue,
                                };
                                match arr.first().and_then(|v| v.as_str()) {
                                    Some("EVENT") => {
                                        if arr.get(1).and_then(|v| v.as_str()) == Some(sub_id) {
                                            if let Some(event) = arr.get(2) {
                                                let pubkey = event["pubkey"].as_str().unwrap_or("");
                                                if pubkey != ALLOWED_PUBKEY {
                                                    debug!(
                                                        "Dropped event from unauthorized pubkey: {}",
                                                        pubkey
                                                    );
                                                } else if verify_event(event) {
                                                    events.push(event.clone());
                                                } else {
                                                    warn!(
                                                        "Dropped event with invalid signature: {:?}",
                                                        event["id"]
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Some("EOSE") => break,
                                    _ => {}
                                }
                            }
                            Ok(Message::Close(_)) => break,
                            Err(e) => {
                                debug!("WebSocket error: {}", e);
                                break;
                            }
                            _ => {}
                        }
                    }
                })
                .await;

                if timeout.is_err() {
                    debug!(
                        "Nostr relay timeout after 30s, returning {} events collected so far",
                        events.len()
                    );
                }

                let close_msg = json!(["CLOSE", sub_id]);
                let _ = write.send(Message::Text(close_msg.to_string().into())).await;
                let _ = write.close().await;

                events
            }};
        }

        if self.use_tor {
            let proxy_addr = self.tor_proxy.as_deref().unwrap_or("127.0.0.1:9050");
            info!("Connecting to {} via Tor proxy {}", relay_url, proxy_addr);

            let socks_stream = Socks5Stream::connect(proxy_addr, (host.as_str(), port))
                .await
                .map_err(|e| format!("Failed to connect via Tor to {}: {}", relay_url, e))?;

            let (ws_stream, _) = client_async_tls(relay_url, socks_stream)
                .await
                .map_err(|e| format!("WebSocket handshake failed for {}: {}", relay_url, e))?;

            let (write, read) = ws_stream.split();
            Ok(collect_events!(write, read))
        } else {
            debug!("Connecting to {} directly (Tor disabled)", relay_url);
            let (ws_stream, _) = connect_async(relay_url)
                .await
                .map_err(|e| format!("Failed to connect to {}: {}", relay_url, e))?;

            let (write, read) = ws_stream.split();
            Ok(collect_events!(write, read))
        }
    }
}

/// Verify a Nostr event:
/// 1. Recompute id = SHA-256([0, pubkey, created_at, kind, tags, content])
/// 2. Verify the Schnorr signature (BIP-340) of id with pubkey.
fn verify_event(event: &Value) -> bool {
    let pubkey_hex = match event["pubkey"].as_str() {
        Some(s) => s,
        None => return false,
    };
    let id_hex = match event["id"].as_str() {
        Some(s) => s,
        None => return false,
    };
    let sig_hex = match event["sig"].as_str() {
        Some(s) => s,
        None => return false,
    };

    // 1. Recompute event id
    let serialized = json!([
        0,
        event["pubkey"],
        event["created_at"],
        event["kind"],
        event["tags"],
        event["content"]
    ]);
    let serialized_str = serialized.to_string();
    let mut hasher = Sha256::new();
    hasher.update(serialized_str.as_bytes());
    let computed_id = hasher.finalize();
    let computed_id_hex = hex::encode(computed_id);

    if computed_id_hex != id_hex {
        debug!(
            "Nostr event id mismatch: expected {} got {}",
            id_hex, computed_id_hex
        );
        return false;
    }

    // 2. Verify Schnorr signature
    let id_bytes = match hex::decode(id_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let sig_bytes = match hex::decode(sig_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let pubkey_bytes = match hex::decode(pubkey_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };

    let secp = Secp256k1::verification_only();

    if pubkey_bytes.len() != 32 {
        debug!("Pubkey is not 32 bytes");
        return false;
    }
    let pubkey_array: [u8; 32] = pubkey_bytes.try_into().unwrap();
    let xonly_pubkey = match XOnlyPublicKey::from_byte_array(pubkey_array) {
        Ok(k) => k,
        Err(e) => {
            debug!("Invalid pubkey: {}", e);
            return false;
        }
    };

    if id_bytes.len() != 32 {
        debug!("Event id is not 32 bytes");
        return false;
    }
    let id_array: [u8; 32] = id_bytes.try_into().unwrap();

    if sig_bytes.len() != 64 {
        debug!("Signature is not 64 bytes");
        return false;
    }
    let sig_array: [u8; 64] = sig_bytes.try_into().unwrap();

    let sig = secp256k1::schnorr::Signature::from_byte_array(sig_array);

    match secp.verify_schnorr(&sig, &id_array, &xonly_pubkey) {
        Ok(_) => true,
        Err(e) => {
            debug!("Signature verification failed: {}", e);
            false
        }
    }
}

/// Parse a NIP-35 Kind 2003 Nostr event into a Torrent struct.
fn parse_nip35_event(event: Value) -> Option<Torrent> {
    let tags = event["tags"].as_array()?;
    let event_id = event["id"].as_str()?.to_string();
    let created_at = event["created_at"].as_u64().unwrap_or(0) as usize;

    let get_tag = |name: &str| -> Option<String> {
        tags.iter().find_map(|t| {
            let arr = t.as_array()?;
            if arr.first()?.as_str()? == name {
                arr.get(1)?.as_str().map(|s| s.to_string())
            } else {
                None
            }
        })
    };

    let name = get_tag("title").or_else(|| get_tag("name"))?;
    let infohash = get_tag("x")?;

    let size: u64 = get_tag("size").and_then(|s| s.parse().ok()).unwrap_or(0);

    // ygg.gratis stores seed/leech/completed in "l" tags with "u2p." prefixes
    let get_l_tag = |prefix: &str| -> usize {
        tags.iter()
            .find_map(|t| {
                let arr = t.as_array()?;
                if arr.first()?.as_str()? == "l" {
                    let val = arr.get(1)?.as_str()?;
                    val.strip_prefix(prefix)?.parse().ok()
                } else {
                    None
                }
            })
            .unwrap_or(0)
    };

    let seed = get_l_tag("u2p.seed:");
    let leech = get_l_tag("u2p.leech:");
    let completed = get_l_tag("u2p.completed:");

    // Count files
    let file_count = tags
        .iter()
        .filter(|t| {
            t.as_array()
                .and_then(|a| a.first())
                .and_then(|v| v.as_str())
                == Some("file")
        })
        .count();

    // Category: prefer numeric ID from "l" "u2p.cat:{id}" tag, fall back to #t tag mapping
    let category_id: usize = tags
        .iter()
        .find_map(|t| {
            let arr = t.as_array()?;
            if arr.first()?.as_str()? == "l" {
                arr.get(1)?.as_str()?.strip_prefix("u2p.cat:")?.parse().ok()
            } else {
                None
            }
        })
        .or_else(|| {
            tags.iter().find_map(|t| {
                let arr = t.as_array()?;
                if arr.first()?.as_str()? == "t" {
                    nostr_tag_to_cat_id(arr.get(1)?.as_str()?)
                } else {
                    None
                }
            })
        })
        .unwrap_or(0);

    // Build magnet link using the same hardcoded tracker list as ygg.gratis
    const MAGNET_TRACKERS: &[&str] = &[
        "https://tracker.yggleak.top/announce",
        "udp://tracker.opentrackr.org:1337/announce",
        "udp://open.demonii.com:1337/announce",
        "udp://open.stealth.si:80/announce",
        "udp://exodus.desync.com:6969/announce",
        "https://torrent.tracker.durukanbal.com:443/announce",
        "udp://tracker1.myporn.club:9337/announce",
        "udp://tracker.torrent.eu.org:451/announce",
        "udp://tracker.theoks.net:6969/announce",
        "udp://tracker.srv00.com:6969/announce",
        "udp://tracker.filemail.com:6969/announce",
        "udp://tracker.dler.org:6969/announce",
        "udp://tracker.corpscorp.online:80/announce",
        "udp://tracker.alaskantf.com:6969/announce",
        "udp://tracker-udp.gbitt.info:80/announce",
        "udp://t.overflow.biz:6969/announce",
        "udp://open.dstud.io:6969/announce",
        "udp://leet-tracker.moe:1337/announce",
        "udp://explodie.org:6969/announce",
        "udp://bittorrent-tracker.e-n-c-r-y-p-t.net:1337/announce",
        "udp://6ahddutb1ucc3cp.ru:6969/announce",
        "udp://94.23.207.177:6969/announce",
        "udp://37.59.48.81:6969/announce",
        "udp://54.36.179.216:6969/announce",
        "udp://193.42.111.57:9337/announce",
        "udp://43.250.54.137:6969/announce",
        "udp://91.216.110.53:451/announce",
        "udp://45.134.88.121:6969/announce",
        "udp://135.125.236.64:6969/announce",
        "udp://5.255.124.190:6969/announce",
        "udp://93.158.213.92:1337/announce",
        "udp://107.189.4.235:1337/announce",
        "udp://tracker.qu.ax:6969/announce",
        "udp://107.189.7.165:6969/announce",
        "udp://103.251.166.126:6969/announce",
        "udp://185.243.218.213:80/announce",
        "http://tracker.zhuqiy.com:80/announce",
        "udp://81.230.84.201:6969/announce",
        "udp://212.42.38.197:6969/announce",
        "http://193.31.26.113:6969/announce",
        "udp://176.99.7.59:6969/announce",
        "http://tr.nyacat.pw:80/announce",
    ];

    let mut magnet = format!("magnet:?xt=urn:btih:{}&dn={}", infohash, encode(&name));
    for tracker in MAGNET_TRACKERS {
        magnet.push_str(&format!("&tr={}", encode(tracker)));
    }

    let link = format!("https://ygg.gratis/#/torrent/{}", event_id);

    // Prefer published_at tag over event created_at (mirrors ygg.gratis behaviour)
    let age_stamp = get_tag("published_at")
        .and_then(|s| s.parse().ok())
        .unwrap_or(created_at);

    Some(Torrent {
        id: event_id.clone(),
        name,
        category_id,
        age_stamp,
        size,
        seed,
        leech,
        completed,
        magnet,
        link,
        file_count,
    })
}

/// Build a magnet URI from a raw NIP-35 event Value (used by /torrent/{id}).
pub fn magnet_from_event(event: &Value) -> Option<String> {
    parse_nip35_event(event.clone()).map(|t| t.magnet)
}
