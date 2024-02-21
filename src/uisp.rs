use crate::api_error::ApiError;
use crate::device::UISPDevice;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref UISP_INSTANCE: UISP = {
        let base_url = env::var("UISP_API").expect("UISP_API environment variable not found.");
        let api_token = env::var("UISP_TOKEN").expect("UISP_TOKEN environment variable not found.");
        UISP::new(base_url, api_token)
    };
}

pub struct UISP {
    base_url: String,
    api_token: String,
}

impl UISP {
    pub fn new(base_url: String, api_token: String) -> Self {
        Self {
            base_url,
            api_token,
        }
    }

    pub fn get_redirect_url(&self, device_id: String) -> String {
        format!(
            "{}/devices/{device_id}/iplink/redirect",
            self.get_base_url()
        )
    }

    pub fn get_base_url(&self) -> &String {
        &self.base_url
    }

    pub fn get_api_token(&self) -> &String {
        &self.api_token
    }

    pub fn get_device_list_url(&self) -> String {
        format!("{}/devices", self.get_base_url())
    }

    pub async fn fetch_device_list(&self) -> Result<Vec<UISPDevice>, ApiError> {
        let client: reqwest::Client = reqwest::Client::new();
        let resp = client
            .get(self.get_device_list_url())
            .header("x-auth-token", self.get_api_token())
            .send()
            .await?;
        let json = resp.json::<Vec<UISPDevice>>().await?;

        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use super::*;

    #[actix_web::test]
    #[ignore]
    async fn test_fetch_device_list() -> Result<(), ApiError> {
        dotenv().ok();
        let list = UISP_INSTANCE.fetch_device_list().await?;

        // List should be non-empty
        assert!(list.len() > 0);
        Ok(())
    }
}
