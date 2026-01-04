use crate::config::Config;
use actix_web::{get, web};

use serde_json::Value;

use wreq::Client;

#[get("/user")]
pub async fn get_user_info(
    data: web::Data<Client>,
    config: web::Data<Config>,
) -> Result<web::Json<Value>, Box<dyn std::error::Error>> {
    let client = data.get_ref();

    let user = crate::user::get_account(client).await;
    // check if error is session expired
    if let Err(e) = &user {
        if e.to_string().contains("Session expired") {
            info!("Trying to renew session...");
            let new_client =
                crate::auth::login(&config, true)
                    .await?;
            data.get_ref().clone_from(&&new_client);
            info!("Session renewed, retrying to get user info...");
            let user = crate::user::get_account(&new_client).await?;
            let json = serde_json::to_value(&user)?;
            return Ok(web::Json(json));
        }
    }

    let user = user?;
    let json = serde_json::to_value(&user)?;
    Ok(web::Json(json))
}
