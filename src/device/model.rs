use lazy_static::lazy_static;
use redis::Commands;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{api_error::ApiError, cache};

lazy_static! {
    static ref IP_ADDRESS_RE: Regex = Regex::new(r"((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}")
        .expect("Cannot initiate regex library.");
}

#[derive(Deserialize)]
pub struct UISPDeviceIdentification {
    pub id: String,
    #[serde(flatten)]
    _extra: HashMap<String, Value>,
}

#[derive(Deserialize)]
pub struct UISPDevice {
    #[serde(rename = "ipAddress")]
    #[serde(deserialize_with = "deserialize_ip_address", default)]
    pub ip_address: Option<String>, // ipAddress field is possibly null
    pub identification: UISPDeviceIdentification,
    #[serde(flatten)]
    _extra: HashMap<String, Value>,
}

/**
 * Custom deserializer for ip_address field. Some IP addresses end with the subnet in CIDR formation. Function uses regex to extract only the IPv4 address.
 */
fn deserialize_ip_address<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let str = Option::<String>::deserialize(deserializer)?;

    let result = str.and_then(|ip_str| {
        IP_ADDRESS_RE
            .find(&ip_str)
            .and_then(|ip_address| Some(ip_address.as_str().to_string()))
    });

    Ok(result)
}

#[derive(Serialize)]
pub struct CacheableDevice {
    ip_address: String,
    device_id: String,
}

impl CacheableDevice {
    pub fn new(ip_address: String, device_id: String) -> Self {
        CacheableDevice {
            ip_address,
            device_id,
        }
    }
}

impl TryFrom<UISPDevice> for CacheableDevice {
    type Error = ApiError;

    fn try_from(value: UISPDevice) -> Result<Self, ApiError> {
        match value.ip_address {
            Some(ip_address) => Ok(CacheableDevice::new(ip_address, value.identification.id)),
            None => Err(ApiError::new(
                500,
                "UISP Device does not have an IP assignment".to_string(),
            )),
        }
    }
}

impl CacheableDevice {
    pub fn to_cache(&self) -> Result<(), ApiError> {
        let mut cache = cache::connection()?;
        let key = format!("{}", self.ip_address);
        let value = format!("{}", self.device_id);
        cache
            .set(key, value)
            .map_err(|_err| ApiError::new(500, format!("Unable to cache device in Redis")))?;

        Ok(())
    }

    pub fn find_in_cache(ip_address: String) -> Result<Option<Self>, ApiError> {
        let mut cache = cache::connection()?;
        let cache_key = format!("{}", ip_address);
        let res: Vec<u8> = cache.get(&cache_key)?;
        match String::from_utf8(res) {
            Ok(device_id) => Ok(Some(CacheableDevice::new(ip_address, device_id))),
            Err(_) => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use crate::api_error::ApiError;

    use super::CacheableDevice;

    #[test]
    fn test_to_cache() -> Result<(), ApiError> {
        dotenv().ok();
        let device = CacheableDevice::new("0.0.0.0".to_string(), "123456".to_string());
        device.to_cache()?;
        Ok(())
    }

    #[test]
    fn test_cache_get() -> Result<(), ApiError> {
        dotenv().ok();
        let expected = CacheableDevice::new("0.0.0.0".to_string(), "123456".to_string());
        expected.to_cache()?;
        let fetched_device = CacheableDevice::find_in_cache(expected.ip_address.clone())?;

        assert!(fetched_device.is_some());

        let actual = fetched_device.expect("Failed to fetch device from cache");
        assert_eq!(expected.device_id, actual.device_id);
        assert_eq!(expected.ip_address, actual.ip_address);
        Ok(())
    }
}
