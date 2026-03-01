use crate::flaresolverr::FlareSolverr;
use std::sync::Arc;

#[derive(Clone)]
pub enum YggClient {
    Direct(wreq::Client),
    Proxied {
        flaresolverr: Arc<FlareSolverr>,
        session_id: String,
        cdn_client: wreq::Client,
        fs_user_agent: String,
        /// Windows Downloads folder where Chrome saves torrent files
        downloads_dir: String,
    },
}

pub struct YggResponse {
    pub status: u16,
    pub body: String,
    pub url: String,
}

impl YggClient {
    fn session_ref(session_id: &str) -> Option<&str> {
        if session_id.is_empty() { None } else { Some(session_id) }
    }

    pub async fn get(&self, url: &str) -> Result<YggResponse, Box<dyn std::error::Error>> {
        match self {
            YggClient::Direct(client) => {
                let response = client.get(url).send().await?;
                let status = response.status().as_u16();
                let final_url = response.url().to_string();
                let body = response.text().await?;
                Ok(YggResponse { status, body, url: final_url })
            }
            YggClient::Proxied { flaresolverr, session_id, .. } => {
                let response = flaresolverr
                    .get(url, Self::session_ref(session_id), None)
                    .await?;
                let solution = response.solution.ok_or("No solution in FlareSolverr response")?;
                Ok(YggResponse {
                    status: solution.status,
                    body: solution.response,
                    url: solution.url,
                })
            }
        }
    }

    pub async fn post_form(
        &self,
        url: &str,
        form_data: &str,
    ) -> Result<YggResponse, Box<dyn std::error::Error>> {
        match self {
            YggClient::Direct(client) => {
                let response = client
                    .post(url)
                    .body(form_data.to_string())
                    .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
                    .send()
                    .await?;
                let status = response.status().as_u16();
                let final_url = response.url().to_string();
                let body = response.text().await?;
                Ok(YggResponse { status, body, url: final_url })
            }
            YggClient::Proxied { flaresolverr, session_id, .. } => {
                let response = flaresolverr
                    .post(url, form_data, Self::session_ref(session_id), None)
                    .await?;
                let solution = response.solution.ok_or("No solution in FlareSolverr response")?;
                Ok(YggResponse {
                    status: solution.status,
                    body: solution.response,
                    url: solution.url,
                })
            }
        }
    }

    pub async fn get_bytes(&self, url: &str) -> Result<(u16, Vec<u8>), Box<dyn std::error::Error>> {
        match self {
            YggClient::Direct(client) => {
                let response = client.get(url).send().await?;
                let status = response.status().as_u16();
                let bytes = response.bytes().await?.to_vec();
                Ok((status, bytes))
            }
            YggClient::Proxied { flaresolverr, session_id, downloads_dir, .. } => {
                // Chrome (FlareSolverr) handles Content-Disposition:attachment by saving
                // the file to disk. We need to:
                // 1. Record the most recent .torrent file before the download
                // 2. Trigger the download via FlareSolverr
                // 3. Wait for a NEW .torrent file to appear in the Downloads folder
                // 4. Read and return its bytes

                let dl_dir = std::path::Path::new(downloads_dir);

                // Snapshot existing .torrent files before download
                let before: std::collections::HashSet<String> = list_torrent_files(dl_dir);
                debug!("Downloads dir: {} — {} existing .torrent files", downloads_dir, before.len());

                // Trigger the download via FlareSolverr (Chrome will save to disk)
                debug!("Triggering download via FlareSolverr: {}", url);
                let _ = flaresolverr
                    .get(url, Self::session_ref(session_id), None)
                    .await?;
                // FlareSolverr returns immediately after Chrome starts the download.
                // The file may not be complete yet — we poll for it below.

                // Wait up to 30s for a new .torrent file to appear
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
                let mut new_file: Option<std::path::PathBuf> = None;

                while std::time::Instant::now() < deadline {
                    let after = list_torrent_files(dl_dir);
                    let new_files: Vec<String> = after.difference(&before).cloned().collect();

                    for fname in &new_files {
                        // Skip .crdownload (incomplete Chrome download)
                        if fname.ends_with(".crdownload") {
                            continue;
                        }
                        let path = dl_dir.join(fname);
                        // Wait until file size stabilizes (not still being written)
                        let size1 = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                        let size2 = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                        if size1 > 0 && size1 == size2 {
                            debug!("New torrent file detected: {} ({} bytes)", fname, size1);
                            new_file = Some(path);
                            break;
                        }
                    }

                    if new_file.is_some() {
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }

                let path = new_file.ok_or_else(|| {
                    format!(
                        "No new .torrent file appeared in {} within 30s after download trigger",
                        downloads_dir
                    )
                })?;

                let bytes = std::fs::read(&path)?;
                debug!("Read {} bytes from {:?}", bytes.len(), path.file_name());

                // Clean up the downloaded file
                if let Err(e) = std::fs::remove_file(&path) {
                    warn!("Could not delete torrent file {:?}: {}", path, e);
                }

                Ok((200, bytes))
            }
        }
    }

    pub fn as_wreq_client(&self) -> Option<&wreq::Client> {
        match self {
            YggClient::Direct(client) => Some(client),
            YggClient::Proxied { cdn_client, .. } => Some(cdn_client),
        }
    }
}

fn list_torrent_files(dir: &std::path::Path) -> std::collections::HashSet<String> {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|n| n.ends_with(".torrent") || n.ends_with(".crdownload"))
                .collect()
        })
        .unwrap_or_default()
}
