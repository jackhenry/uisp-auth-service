use crate::{api_error::ApiError, device::CacheableDevice, uisp::UISP_INSTANCE};
use lazy_static::lazy_static;
use r2d2;
use redis::ConnectionLike;
use std::env;
extern crate redis;

type Pool = r2d2::Pool<redis::Client>;
pub type CacheConnection = r2d2::PooledConnection<redis::Client>;

lazy_static! {
    static ref POOL: Pool = {
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL environment variable not found.");
        let client = redis::Client::open(redis_url.clone())
            .expect(format!("Failed to open connection to {:?}", redis_url).as_str());
        Pool::new(client).expect("failed")
    };
}

pub fn init() {
    info!("Initializing redis connection");
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get redis connection");
    assert_eq!(
        true,
        conn.check_connection(),
        "Redis connection check failed. Check connection to redis server."
    );
    info!("Finished initializing redis connection");
}

pub fn connection() -> Result<CacheConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting redis connection: {}", e)))
}

pub async fn populate() -> Result<(), ApiError> {
    info!("Populating redis cache with device addresses and client ids");
    let device_list = UISP_INSTANCE.fetch_device_list().await?;
    info!("Finished populating redis cache");
    //println!("Device list length: {}", device_list.len());

    let mut count = 0;

    for device in device_list {
        // Cache the device id if it's cacheable (IP address exists)
        if let Ok(device) = CacheableDevice::try_from(device) {
            device.to_cache()?;
            count += 1;
        }
    }

    //println!("Total: {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use crate::api_error::ApiError;

    use super::populate;

    #[actix_web::test]
    async fn test_populate() -> Result<(), ApiError> {
        dotenv().ok();
        populate().await
    }
}
