use crate::domain::get_leaked_ip;
use crate::flaresolverr::FlareSolverrCookieInput;
use crate::resolver::AsyncDNSResolverAdapter;
use crate::ygg_client::YggClient;
use crate::{DOMAIN, LOGIN_PAGE, LOGIN_PROCESS_PAGE};
use std::fs::File;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, OnceLock};
use wreq::header::HeaderMap;
use wreq::{Client, Url};
use wreq_util::{Emulation, EmulationOS, EmulationOption};

pub static KEY: OnceLock<String> = OnceLock::new();

pub async fn login(
    username: &str,
    password: &str,
    use_sessions: bool,
    flaresolverr_url: Option<&str>,
    flaresolverr_downloads_dir: Option<&str>,
) -> Result<YggClient, Box<dyn std::error::Error>> {
    debug!("Logging in with username: {}", username);

    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    if let Some(fs_url) = flaresolverr_url {
        return login_via_flaresolverr(fs_url, domain, username, password, flaresolverr_downloads_dir).await;
    }

    login_direct(domain, username, password, use_sessions).await
}

async fn login_via_flaresolverr(
    fs_url: &str,
    domain: &str,
    username: &str,
    password: &str,
    downloads_dir_override: Option<&str>,
) -> Result<YggClient, Box<dyn std::error::Error>> {
    debug!("Using FlareSolverr at {} to bypass Cloudflare", fs_url);

    let flaresolverr = crate::flaresolverr::FlareSolverr::new(fs_url)?;

    // Try to create a persistent session; fall back to sessionless mode
    let session_id = match flaresolverr.create_session().await {
        Ok(id) => {
            debug!("Created FlareSolverr session: {}", id);
            Some(id)
        }
        Err(e) => {
            warn!(
                "FlareSolverr session creation failed ({}), continuing without session",
                e
            );
            None
        }
    };

    let session_ref = session_id.as_deref();
    let start = std::time::Instant::now();

    // GET login page with account_created cookie
    let cookies = vec![FlareSolverrCookieInput {
        name: "account_created".to_string(),
        value: "true".to_string(),
        domain: domain.to_string(),
    }];
    let login_page_response = flaresolverr
        .get(
            &format!("https://{}{}", domain, LOGIN_PAGE),
            session_ref,
            Some(cookies),
        )
        .await?;

    let solution = login_page_response
        .solution
        .ok_or("FlareSolverr returned no solution for login page")?;

    // Check for ygg_ cookie
    let has_ygg_cookie = solution.cookies.iter().any(|c| c.name == "ygg_");
    if !has_ygg_cookie {
        if let Some(sid) = &session_id {
            let _ = flaresolverr.destroy_session(sid).await;
        }
        return Err("No ygg_ cookie found via FlareSolverr".into());
    }

    debug!(
        "FlareSolverr got ygg_ cookie, {} cookies total",
        solution.cookies.len()
    );

    // POST login
    let post_data = format!(
        "id={}&pass={}",
        urlencoding::encode(username),
        urlencoding::encode(password)
    );
    let login_result = flaresolverr
        .post(
            &format!("https://{}{}", domain, LOGIN_PROCESS_PAGE),
            &post_data,
            session_ref,
            None,
        )
        .await?;

    let solution = login_result
        .solution
        .ok_or("FlareSolverr returned no solution for login POST")?;

    if solution.status == 401 {
        if let Some(sid) = &session_id {
            let _ = flaresolverr.destroy_session(sid).await;
        }
        error!("Invalid username or password");
        return Err("Invalid username or password".into());
    }
    if solution.status >= 400 {
        if let Some(sid) = &session_id {
            let _ = flaresolverr.destroy_session(sid).await;
        }
        return Err(format!("Failed to login via FlareSolverr: {}", solution.status).into());
    }

    // GET root page to finalize session and collect all CF + YGG cookies
    let final_response = flaresolverr
        .get(&format!("https://{}/", domain), session_ref, None)
        .await?;

    let stop = std::time::Instant::now();
    debug!(
        "Logged in via FlareSolverr in {:?}",
        stop.duration_since(start)
    );

    // Build a wreq client pre-loaded with all cookies from the FlareSolverr session.
    // This includes cf_clearance (Cloudflare bypass token) and ygg_ (YGG session).
    // cf_clearance is tied to the IP address, not the TLS fingerprint, so wreq
    // can use it directly without needing to emulate Chrome's TLS stack.
    let final_solution = final_response.solution.ok_or("No solution from root page")?;
    let fs_user_agent = final_solution.user_agent.clone();

    let leaked_ip = get_leaked_ip().await?;
    let emu = EmulationOption::builder()
        .emulation(Emulation::Chrome132)
        .emulation_os(EmulationOS::Windows)
        .build();
    let cdn_client = Client::builder()
        .emulation(emu)
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .dns_resolver(Arc::new(AsyncDNSResolverAdapter::new()?))
        .cert_verification(false)
        .verify_hostname(false)
        .resolve(
            domain,
            SocketAddr::new(IpAddr::from_str(leaked_ip.as_str())?, 443),
        )
        .build()?;

    let parsed_url = Url::parse(&format!("https://{}/", domain))?;
    let all_cookies = final_solution.cookies;
    debug!("Injecting {} cookies into cdn_client: {:?}",
        all_cookies.len(),
        all_cookies.iter().map(|c| &c.name).collect::<Vec<_>>()
    );
    for cookie in &all_cookies {
        let c = wreq::cookie::CookieBuilder::new(&cookie.name, &cookie.value)
            .domain(domain)
            .path("/")
            .http_only(false)
            .secure(true)
            .build();
        cdn_client.set_cookie(&parsed_url, c);
    }

    // Determine the Windows Downloads folder where Chrome saves torrent files.
    // Priority: config value > FLARESOLVERR_DOWNLOADS_DIR env var > %USERPROFILE%\Downloads
    let downloads_dir = downloads_dir_override
        .map(|s| s.to_string())
        .or_else(|| std::env::var("FLARESOLVERR_DOWNLOADS_DIR").ok())
        .unwrap_or_else(|| {
            let userprofile = std::env::var("USERPROFILE")
                .unwrap_or_else(|_| {
                    let homedrive = std::env::var("HOMEDRIVE").unwrap_or_else(|_| "C:".to_string());
                    let homepath = std::env::var("HOMEPATH").unwrap_or_else(|_| "\\Users\\Default".to_string());
                    format!("{}{}", homedrive, homepath)
                });
            format!("{}\\Downloads", userprofile)
        });
    debug!("FlareSolverr downloads dir: {}", downloads_dir);

    Ok(YggClient::Proxied {
        flaresolverr: Arc::new(flaresolverr),
        session_id: session_id.unwrap_or_default(),
        cdn_client,
        fs_user_agent,
        downloads_dir,
    })
}

async fn login_direct(
    domain: &str,
    username: &str,
    password: &str,
    use_sessions: bool,
) -> Result<YggClient, Box<dyn std::error::Error>> {
    let emu = EmulationOption::builder()
        .emulation(Emulation::Chrome132)
        .emulation_os(EmulationOS::Windows)
        .build();

    let leaked_ip = get_leaked_ip().await?;

    let client = Client::builder()
        .emulation(emu)
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .dns_resolver(Arc::new(AsyncDNSResolverAdapter::new()?))
        .cert_verification(false)
        .verify_hostname(false)
        .resolve(
            domain,
            SocketAddr::new(IpAddr::from_str(leaked_ip.as_str())?, 443),
        )
        .build()?;

    let mut headers = HeaderMap::new();
    add_bypass_headers(&mut headers);

    let start = std::time::Instant::now();

    if use_sessions {
        // check if the session file exists
        let session_file = format!("sessions/{}.cookies", username);
        if std::path::Path::new(&session_file.clone()).exists() {
            debug!("Session file found: {}", session_file);
            // load the session from the file
            let cookies = std::fs::read_to_string(&session_file)?;
            let cookies = cookies.split(";").collect::<Vec<&str>>();
            let cookies_len = cookies.len();
            for cookie in cookies {
                let cookie = cookie.trim();
                if cookie.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = cookie.split('=').collect();
                if parts.len() != 2 {
                    continue;
                }
                let name = parts[0].trim();
                let value = parts[1].trim();
                let cookie = wreq::cookie::CookieBuilder::new(name, value)
                    .domain(domain)
                    .path("/")
                    .http_only(true)
                    .secure(true)
                    .build();
                let url = Url::parse(format!("https://{domain}/").as_str())?;
                client.set_cookie(&url, cookie);
            }
            debug!("Restored {} cookies from session file", cookies_len);
        }

        // check if the session is still valid
        let response = client
            .get(format!("https://{domain}/"))
            .headers(headers.clone())
            .send()
            .await?;
        if response.status().is_success() {
            let stop = std::time::Instant::now();
            debug!(
                "Successfully resumed session in {:?}",
                stop.duration_since(start)
            );
            return Ok(YggClient::Direct(client));
        } else {
            debug!(
                "Session is not valid, deleting session file (code {})",
                response.status()
            );
            // session is not valid, delete the file
            let session_file = format!("sessions/{}.cookies", username);
            let _ = std::fs::remove_file(&session_file);
            debug!("Session file deleted");
        }
    }

    client.clear_cookies();

    // inject account_created=true cookie (cookie magique)
    let cookie = wreq::cookie::CookieBuilder::new("account_created", "true")
        .domain(domain)
        .path("/")
        .http_only(true)
        .secure(true)
        .build();

    let url = Url::parse(format!("https://{domain}/").as_str())?;
    client.set_cookie(&url, cookie);

    // make a request to the login page
    let response = client
        .get(format!("https://{domain}{LOGIN_PAGE}"))
        .headers(headers.clone())
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch login page: {}", response.status()).into());
    }
    let _headers = response.headers(); // digest the headers to get the cookies

    // detect if the ygg_ cookie is set
    let cookies = response.cookies();
    let mut has_ygg_cookie = false;
    for cookie in cookies {
        if cookie.name() == "ygg_" {
            has_ygg_cookie = true;
            break;
        }
    }

    if !has_ygg_cookie {
        return Err("No ygg_ cookie found".into());
    }

    // multipart/form-data
    let payload = [("id", username), ("pass", password)];

    // post multipart on /auth/process_login
    let response = client
        .post(format!("https://{domain}{LOGIN_PROCESS_PAGE}"))
        .headers(headers.clone())
        .form(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        if response.status() == 401 {
            error!("Invalid username or password");
            return Err("Invalid username or password".into());
        }
        return Err(format!("Failed to login: {}", response.status()).into());
    }

    let _headers = response.headers(); // digest the headers to get the cookies

    // get site root page for final cookies
    let response = client
        .get(format!("https://{domain}/"))
        .headers(headers.clone())
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch site root page: {}", response.status()).into());
    }

    let stop = std::time::Instant::now();
    debug!("Logged in successfully in {:?}", stop.duration_since(start));

    let _headers = response.cookies(); // digest the headers to get the cookies

    if use_sessions {
        save_session(username, &client).await?;
    }

    Ok(YggClient::Direct(client))
}

async fn save_session(username: &str, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    // save the session in a file
    let mut file = File::create(format!("sessions/{}.cookies", username))?;
    let cookies_header = client
        .get_cookies(&Url::parse(
            format!("https://{}/", DOMAIN.lock()?.as_str()).as_str(),
        )?)
        .unwrap();
    let cookies_header_value = cookies_header.to_str()?;
    debug!("Cookies: {}", cookies_header_value);
    file.write_all(cookies_header_value.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn add_bypass_headers(headers: &mut HeaderMap) {
    let own_ip_lock = crate::domain::OWN_IP.get();
    if let Some(own_ip) = own_ip_lock {
        headers.insert("CF-Connecting-IP", own_ip.parse().unwrap());
        headers.insert("X-Forwarded-For", own_ip.parse().unwrap());
    }
}
