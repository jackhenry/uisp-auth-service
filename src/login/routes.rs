use actix_web::{
    get,
    web::{self, Redirect},
};
use log::{debug, error};
use redis::Commands;

use crate::{api_error::ApiError, cache, uisp::UISP_INSTANCE};

#[get("/login/{ip_address}")]
async fn login(path: web::Path<String>) -> Result<Redirect, ApiError> {
    let ip_address = path.into_inner();
    debug!("Received: {}", ip_address);
    let mut cache = cache::connection()?;
    // Fetch the client id from the redis cache
    let client_id_bytes: Vec<u8> = cache.get(&ip_address).map_err(|_| {
        error!("Redis error while fetching {}", &ip_address);
        ApiError::internal_error()
    })?;
    let client_id = String::from_utf8(client_id_bytes).map_err(|_| {
        error!("Failed parsing client id from redis.");
        ApiError::internal_error()
    })?;

    // Call the UISP api with the fetched client ID and receiver the ticketid
    let response = UISP_INSTANCE
        .login(client_id)
        .await
        .map_err(|_| ApiError::uisp_device_not_found(ip_address.clone()))?;

    let authenticated_url = format!(
        "https://{}:{}/ticket.cgi?ticketid={}",
        ip_address, response.https_port, response.ticketid
    );

    Ok(Redirect::to(authenticated_url))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}
