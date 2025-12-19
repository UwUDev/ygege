use crate::categories::CATEGORIES_CACHE;
use actix_web::{HttpResponse, get, web};
use wreq::Client;

#[get("/categories")]
pub async fn categories(data: web::Data<Client>) -> HttpResponse {
    // Try to get from cache first
    if let Some(cached_categories) = CATEGORIES_CACHE.get() {
        return HttpResponse::Ok().json(cached_categories);
    }

    // If cache is empty (shouldn't happen after startup), scrape now
    warn!("Categories cache was empty, scraping now...");
    match crate::categories::scrape_categories(&data).await {
        Ok(categories) => {
            let _ = CATEGORIES_CACHE.set(categories.clone());
            HttpResponse::Ok().json(categories)
        }
        Err(e) => {
            error!("Failed to fetch categories: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch categories")
        }
    }
}
