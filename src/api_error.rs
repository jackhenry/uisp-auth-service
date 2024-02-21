use std::fmt;

use actix_web::{http::header::ContentType, HttpResponse};
use redis::RedisError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new<S>(status_code: u16, message: S) -> ApiError
    where
        S: Into<String>,
    {
        ApiError {
            status_code,
            message: message.into(),
        }
    }

    pub fn uisp_device_not_found(ip_address: String) -> ApiError {
        ApiError::new(
            404,
            format!("UISP Didn't find a device with the IP {}", ip_address),
        )
    }

    pub fn internal_error() -> ApiError {
        ApiError::new(500, "Internal Server Error")
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<RedisError> for ApiError {
    fn from(error: RedisError) -> ApiError {
        ApiError::new(500, format!("Redis error: {}", error))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> ApiError {
        ApiError::new(500, format!("Request error: {}", error))
    }
}
