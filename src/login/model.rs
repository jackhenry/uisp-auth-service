use serde::Deserialize;

use crate::{api_error::ApiError, uisp};

impl uisp::UISP {
    pub async fn login(&self, device_id: String) -> Result<UISPRedirectResult, ApiError> {
        let client = reqwest::Client::new();
        let resp = client
            .post(self.get_redirect_url(device_id))
            .header("x-auth-token", self.get_api_token())
            .send()
            .await?
            .json::<UISPRedirectResult>()
            .await?;

        Ok(resp)
    }
}

#[derive(Deserialize)]
pub struct UISPRedirectResult {
    #[serde(rename = "token")]
    pub ticketid: String,
    #[serde(rename = "httpsPort")]
    pub https_port: u16,
}
