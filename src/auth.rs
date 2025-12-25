// use crate::domain::get_leaked_ip; // Unused: leaked IP is outdated
use crate::resolver::AsyncCloudflareResolverAdapter;
use crate::{DOMAIN, LOGIN_PAGE, LOGIN_PROCESS_PAGE};
use std::fs::File;
use std::io::Write;
// use std::net::{IpAddr, SocketAddr}; // Unused: no longer forcing DNS resolve
// use std::str::FromStr; // Unused: no longer parsing leaked IP
use std::sync::{Arc, OnceLock};
use wreq::header::HeaderMap;
use wreq::{Client, Url};
use wreq_util::{Emulation, EmulationOS, EmulationOption};

pub static KEY: OnceLock<String> = OnceLock::new();

pub async fn login(
    username: &str,
    password: &str,
    use_sessions: bool,
) -> Result<Client, Box<dyn std::error::Error>> {
    debug!("Logging in with username: {}", username);

    let emu = EmulationOption::builder()
        .emulation(Emulation::Chrome132) // no H3 check on CF before 133
        .emulation_os(EmulationOS::Windows)
        .build();

    // les fameux DNS menteurs
    let cloudflare_dns = Arc::new(AsyncCloudflareResolverAdapter::new()?);

    let domain_lock = DOMAIN.lock()?;
    let cloned_guard = domain_lock.clone();
    let domain = cloned_guard.as_str();
    drop(domain_lock);

    // FIXME: The leaked IP from Pastebin is outdated (89.42.231.91)
    // Current YGG IPs are: 104.26.5.166, 104.26.4.166, 172.67.70.199
    // Commented out to allow normal DNS resolution
    // let leaked_ip = get_leaked_ip().await?;

    let client = Client::builder()
        .emulation(emu)
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .dns_resolver(cloudflare_dns)
        .cert_verification(false)
        .verify_hostname(false)
        // .resolve(
        //     &domain,
        //     SocketAddr::new(IpAddr::from_str(leaked_ip.as_str())?, 443),
        // )
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
            return Ok(client);
        } else {
            debug!(
                "Session is not valid, deleting session file (code {})",
                response.status()
            );
            // session is not valid, delete the file
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
        //.get(format!("https://rp.lila.ws:8749/api/all"))
        .get(format!("https://{domain}{LOGIN_PAGE}"))
        .headers(headers.clone())
        .send()
        .await?;

    /*println!("Body: {}", response.text().await?);
    panic!();*/

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

    Ok(client)
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
