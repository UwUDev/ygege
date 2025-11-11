use actix_web::{HttpRequest, HttpResponse, get};

#[get("/")]
pub async fn index(req: HttpRequest) -> HttpResponse {
    let body = match is_french_browser(&req) {
        true => include_str!("index-fr.html"),
        false => include_str!("index.html"),
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

fn is_french_browser(req: &HttpRequest) -> bool {
    if let Some(accept_language) = req.headers().get("Accept-Language") {
        if let Ok(lang_str) = accept_language.to_str() {
            let mut french_q = 0.0;
            let mut english_q = 0.0;

            for lang_part in lang_str.split(',') {
                let parts: Vec<&str> = lang_part.trim().split(';').collect();
                let lang = parts[0].to_lowercase();
                let q_value = if parts.len() > 1 {
                    parts[1]
                        .trim()
                        .strip_prefix("q=")
                        .and_then(|q| q.parse::<f32>().ok())
                        .unwrap_or(1.0)
                } else {
                    1.0
                };

                if lang.starts_with("fr") && french_q == 0.0 {
                    french_q = q_value;
                }
                if lang.starts_with("en") && english_q == 0.0 {
                    english_q = q_value;
                }
            }

            return french_q > english_q;
        }
    }
    false
}
