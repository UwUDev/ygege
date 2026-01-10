use crate::client::build_simple_client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wreq::Client;

#[derive(Debug, Serialize)]
struct FlareSolverrRequest {
    cmd: String,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<Vec<Cookie>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cookie {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct FlareSolverrResponse {
    status: String,
    message: String,
    solution: Option<Solution>,
}

#[derive(Debug, Deserialize)]
struct Solution {
    url: String,
    status: u16,
    cookies: Vec<Cookie>,
    #[serde(rename = "userAgent")]
    user_agent: String,
    response: Option<String>,
}

pub struct FlareSolverrClient {
    client: Client,
    base_url: String,
    session_id: Option<String>,
}

impl FlareSolverrClient {
    pub fn new(base_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = build_simple_client()?;
        Ok(Self {
            client,
            base_url,
            session_id: None,
        })
    }

    pub async fn get_with_cookies(
        &mut self,
        url: &str,
        cookies: Option<&[wreq::cookie::Cookie<'_>]>,
    ) -> Result<(String, Vec<wreq::cookie::Cookie<'static>>), Box<dyn std::error::Error>> {
        let flare_cookies = cookies.map(|c| {
            c.iter()
                .map(|cookie| Cookie {
                    name: cookie.name().to_string(),
                    value: cookie.value().to_string(),
                })
                .collect()
        });

        let request = FlareSolverrRequest {
            cmd: "request.get".to_string(),
            url: url.to_string(),
            session: self.session_id.clone(),
            cookies: flare_cookies,
            post_data: None,
        };

        let response = self
            .client
            .post(&format!("{}/v1", self.base_url))
            .json(&request)
            .send()
            .await?;

        let flare_response: FlareSolverrResponse = response.json().await?;

        if flare_response.status != "ok" {
            return Err(format!("FlareSolverr error: {}", flare_response.message).into());
        }

        let solution = flare_response.solution.ok_or("No solution in response")?;

        let wreq_cookies = solution
            .cookies
            .into_iter()
            .map(|c| {
                wreq::cookie::CookieBuilder::new(c.name, c.value)
                    .build()
            })
            .collect();

        // Return the response body if available, otherwise empty string
        let body = solution.response.unwrap_or_default();
        Ok((body, wreq_cookies))
    }

    pub async fn post_form(
        &mut self,
        url: &str,
        form_data: &HashMap<String, String>,
        cookies: Option<&[wreq::cookie::Cookie<'_>]>,
    ) -> Result<Vec<wreq::cookie::Cookie<'static>>, Box<dyn std::error::Error>> {
        let flare_cookies = cookies.map(|c| {
            c.iter()
                .map(|cookie| Cookie {
                    name: cookie.name().to_string(),
                    value: cookie.value().to_string(),
                })
                .collect()
        });

        let post_data = form_data
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let request = FlareSolverrRequest {
            cmd: "request.post".to_string(),
            url: url.to_string(),
            session: self.session_id.clone(),
            cookies: flare_cookies,
            post_data: Some(post_data),
        };

        let response = self
            .client
            .post(&format!("{}/v1", self.base_url))
            .json(&request)
            .send()
            .await?;

        let flare_response: FlareSolverrResponse = response.json().await?;

        if flare_response.status != "ok" {
            return Err(format!("FlareSolverr error: {}", flare_response.message).into());
        }

        let solution = flare_response.solution.ok_or("No solution in response")?;

        let wreq_cookies = solution
            .cookies
            .into_iter()
            .map(|c| {
                wreq::cookie::CookieBuilder::new(c.name, c.value)
                    .build()
            })
            .collect();

        Ok(wreq_cookies)
    }
}