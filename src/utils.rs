use crate::LOGIN_PAGE;

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
