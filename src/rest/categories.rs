use crate::categories::CATEGORIES_CACHE;
use actix_web::{HttpResponse, get};

#[get("/categories")]
pub async fn categories() -> HttpResponse {
    if let Some(cached_categories) = CATEGORIES_CACHE.get() {
        return HttpResponse::Ok().json(cached_categories);
    }

    warn!("Categories cache was empty");
    HttpResponse::InternalServerError().body("Categories not initialized")
}
