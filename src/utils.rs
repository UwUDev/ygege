use crate::LOGIN_PAGE;
use sha1::{Digest, Sha1};
use serde_bencode::de;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Torrent {
    info: serde_bencode::value::Value,
}

pub fn calculate_torrent_hash(torrent_bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the torrent file
    let torrent: Torrent = de::from_bytes(torrent_bytes)?;

    let info_bencoded = serde_bencode::to_bytes(&torrent.info)?;

    let mut hasher = Sha1::new();
    hasher.update(&info_bencoded);
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

pub fn check_session_expired(response: &wreq::Response) -> bool {
    if !response.status().is_success() {
        let code = response.status();
        debug!("Response status code: {}", code);
        if code == 307 {
            warn!("Session expired...");
            return true;
        }
    }

    let final_url = response.url().as_str().to_string();
    if final_url.contains(LOGIN_PAGE) {
        warn!("Session expired...");
        return true;
    }

    false
}

#[cfg(test)]
mod test_utils {
    use super::*;
    #[tokio::test]
    async fn test_torrent_hash() {
        let torrent = include_bytes!("../tests/test.torrent");
        let hash = calculate_torrent_hash(torrent).expect("Failed to calculate hash");
        println!("Torrent info hash: {}", hash);

        assert_eq!(hash.len(), 40);
        assert_eq!(hash, "d984f67af9917b214cd8b6048ab5624c7df6a07a");
    }
}
