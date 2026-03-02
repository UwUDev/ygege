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
                // Snapshot downloads dir BEFORE triggering FlareSolverr
                let dl_dir = std::path::Path::new(downloads_dir.as_str());
                let before: std::collections::HashSet<String> = list_torrent_files(dl_dir);
                log::debug!("get_bytes: triggering FlareSolverr GET + polling {} ({} existing files)", downloads_dir, before.len());

                // FlareSolverr triggers Chrome to navigate to the URL.
                // Chrome saves the .torrent file to its Downloads dir (shared volume).
                let _fs_response = flaresolverr
                    .get(url, Self::session_ref(session_id), None)
                    .await;

                // Poll downloads directory for the new .torrent file
                log::debug!("get_bytes: polling downloads dir for new .torrent file");
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
                let mut new_file: Option<std::path::PathBuf> = None;

                while std::time::Instant::now() < deadline {
                    let after = list_torrent_files(dl_dir);
                    let new_files: Vec<String> = after.difference(&before).cloned().collect();

                    for fname in &new_files {
                        if fname.ends_with(".crdownload") {
                            continue;
                        }
                        let path = dl_dir.join(fname);
                        let size1 = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                        let size2 = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                        if size1 > 0 && size1 == size2 {
                            log::debug!("get_bytes: found {} ({} bytes)", fname, size1);
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
                        "No torrent file appeared in {} within 30s for {}",
                        downloads_dir, url
                    )
                })?;

                let bytes = std::fs::read(&path)?;
                log::debug!("get_bytes: read {} bytes from {:?}", bytes.len(), path.file_name());

                // Best-effort cleanup
                if let Err(e) = std::fs::remove_file(&path) {
                    log::debug!("get_bytes: cleanup of {:?} skipped: {}", path.file_name(), e);
                }

                Ok((200, bytes))
            }
        }
    }

    pub fn get_cookies(&self, url: &wreq::Url) -> Option<wreq::header::HeaderValue> {
        self.as_wreq_client().and_then(|c| c.get_cookies(url))
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
