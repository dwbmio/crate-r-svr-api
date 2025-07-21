pub mod model;
pub use rsvr_core::{error, RespVO};
use rsvr_core::{reqwest, url};
use serde_json::json;

//=========================
pub struct NexusApi {
    nexus: N
}
//=========================

impl NexusApi {
    pub fn new(host: String, cookie: String) -> Self {
        Self { host, cookie }
    }

    pub fn client_builder(&self) -> Result<reqwest::Client, error::ApiReqError> {
        let cli = reqwest::Client::builder().build()?;
        Ok(cli)
    }

    /// Get the host of the Nexus API.
    pub fn download_component(&self) -> &str {
        let path = "/api/"
    }

    /// upload a single package
    pub async fn put_component_raw(
        &self,
        dpm: &model::Raw,
    ) -> Result<RespVO<serde_json::Value>, error::ApiReqError> {
        let path = "/api/artifactory/add";
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .put(addr)
            .header("Content-Type", "application/json")
            .header("Cookie", self.cookie.as_str())
            .body(json!(dpm).to_string())
            .send()
            .await?
            .json::<RespVO<serde_json::Value>>()
            .await?;
        Ok(resp)
    }
}
