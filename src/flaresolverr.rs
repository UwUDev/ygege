use serde::{Deserialize, Serialize};
use wreq::Client;

pub struct FlareSolverr {
    base_url: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct FlareSolverrGetRequest {
    cmd: String,
    url: String,
    #[serde(rename = "maxTimeout")]
    max_timeout: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<Vec<FlareSolverrCookieInput>>,
}

#[derive(Debug, Serialize)]
struct FlareSolverrPostRequest {
    cmd: String,
    url: String,
    #[serde(rename = "maxTimeout")]
    max_timeout: u64,
    #[serde(rename = "postData")]
    post_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<Vec<FlareSolverrCookieInput>>,
}

#[derive(Debug, Serialize)]
struct FlareSolverrSessionRequest {
    cmd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FlareSolverrCookieInput {
    pub name: String,
    pub value: String,
    pub domain: String,
}

#[derive(Debug, Deserialize)]
pub struct FlareSolverrResponse {
    pub status: String,
    pub message: String,
    pub solution: Option<FlareSolverrSolution>,
    pub session: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FlareSolverrSolution {
    pub url: String,
    pub status: u16,
    pub response: String,
    pub cookies: Vec<FlareSolverrCookie>,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FlareSolverrCookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
}

impl FlareSolverr {
    pub fn new(base_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder().build()?;
        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        })
    }

    fn endpoint(&self) -> String {
        format!("{}/v1", self.base_url)
    }

    pub async fn create_session(&self) -> Result<String, Box<dyn std::error::Error>> {
        let request_body = FlareSolverrSessionRequest {
            cmd: "sessions.create".to_string(),
            session: None,
        };

        let mut last_err = None;
        for attempt in 1..=3 {
            match self
                .client
                .post(self.endpoint())
                .json(&request_body)
                .send()
                .await
            {
                Ok(response) => match self.parse_response(response).await {
                    Ok(parsed) => {
                        return parsed
                            .session
                            .ok_or_else(|| "No session ID in FlareSolverr response".into());
                    }
                    Err(e) => {
                        warn!(
                            "FlareSolverr sessions.create attempt {}/3 failed: {}",
                            attempt, e
                        );
                        last_err = Some(e);
                    }
                },
                Err(e) => {
                    warn!(
                        "FlareSolverr sessions.create attempt {}/3 connection error: {}",
                        attempt, e
                    );
                    last_err = Some(e.into());
                }
            }
            if attempt < 3 {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
        Err(last_err
            .unwrap_or_else(|| "FlareSolverr sessions.create failed after 3 attempts".into()))
    }

    pub async fn destroy_session(
        &self,
        session_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request_body = FlareSolverrSessionRequest {
            cmd: "sessions.destroy".to_string(),
            session: Some(session_id.to_string()),
        };

        let response = self
            .client
            .post(self.endpoint())
            .json(&request_body)
            .send()
            .await?;

        self.parse_response(response).await?;
        Ok(())
    }

    pub async fn get(
        &self,
        target_url: &str,
        session: Option<&str>,
        cookies: Option<Vec<FlareSolverrCookieInput>>,
    ) -> Result<FlareSolverrResponse, Box<dyn std::error::Error>> {
        let request_body = FlareSolverrGetRequest {
            cmd: "request.get".to_string(),
            url: target_url.to_string(),
            max_timeout: 60000,
            session: session.map(|s| s.to_string()),
            cookies,
        };

        let response = self
            .client
            .post(self.endpoint())
            .json(&request_body)
            .send()
            .await?;

        self.parse_response(response).await
    }

    pub async fn post(
        &self,
        target_url: &str,
        post_data: &str,
        session: Option<&str>,
        cookies: Option<Vec<FlareSolverrCookieInput>>,
    ) -> Result<FlareSolverrResponse, Box<dyn std::error::Error>> {
        let request_body = FlareSolverrPostRequest {
            cmd: "request.post".to_string(),
            url: target_url.to_string(),
            max_timeout: 60000,
            post_data: post_data.to_string(),
            session: session.map(|s| s.to_string()),
            cookies,
        };

        let response = self
            .client
            .post(self.endpoint())
            .json(&request_body)
            .send()
            .await?;

        self.parse_response(response).await
    }

    /// Downloads a binary file using the Chrome session inside FlareSolverr.
    /// 
    /// Chrome downloads files with Content-Disposition:attachment to disk.
    /// FlareSolverr cannot intercept this. Instead, we use a data: URL that
    /// runs JavaScript fetch() inside Chrome's context, which HAS the valid
    /// CF cookies and TLS fingerprint. The fetch result is base64-encoded and
    /// embedded in the page DOM, which FlareSolverr returns in solution.response.


    async fn parse_response(
        &self,
        response: wreq::Response,
    ) -> Result<FlareSolverrResponse, Box<dyn std::error::Error>> {
        let status = response.status();
        let body = response.text().await?;

        match serde_json::from_str::<FlareSolverrResponse>(&body) {
            Ok(fs_response) => {
                if fs_response.status != "ok" {
                    return Err(format!("FlareSolverr error: {}", fs_response.message).into());
                }
                Ok(fs_response)
            }
            Err(parse_err) => {
                if !status.is_success() {
                    Err(format!("FlareSolverr HTTP {}: {}", status, body).into())
                } else {
                    Err(format!(
                        "FlareSolverr response parse error: {} - body: {}",
                        parse_err, body
                    )
                    .into())
                }
            }
        }
    }
}
