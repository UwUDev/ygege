use crate::categories::nostr_tag_to_cat_id;
use crate::parser::Torrent;
use futures::{SinkExt, StreamExt};
use secp256k1::{Secp256k1, XOnlyPublicKey};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use urlencoding::encode;
use uuid::Uuid;

pub struct NostrClient {
    relay_url: String,
}

impl NostrClient {
    pub fn new(relay_url: &str) -> Self {
        NostrClient {
            relay_url: relay_url.to_string(),
        }
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
            "Nostr REQ to {}: {}",
            self.relay_url,
            req.to_string().chars().take(200).collect::<String>()
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

    /// Open a WebSocket, send a REQ, collect EVENTs until EOSE or timeout, then close.
    async fn send_req(
        &self,
        sub_id: &str,
        req: Value,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(&self.relay_url).await?;
        let (mut write, mut read) = ws_stream.split();

        write.send(Message::Text(req.to_string().into())).await?;

        let mut events: Vec<Value> = Vec::new();

        let timeout = tokio::time::timeout(std::time::Duration::from_secs(30), async {
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
                                        if verify_event(event) {
                                            events.push(event.clone());
                                        } else {
                                            warn!("Dropped event with invalid signature: {:?}", event["id"]);
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

        // Close subscription
        let close_msg = json!(["CLOSE", sub_id]);
        let _ = write
            .send(Message::Text(close_msg.to_string().into()))
            .await;
        let _ = write.close().await;

        Ok(events)
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
        debug!("Nostr event id mismatch: expected {} got {}", id_hex, computed_id_hex);
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
