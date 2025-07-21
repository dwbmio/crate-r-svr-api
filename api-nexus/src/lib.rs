pub mod model;
pub use static_remote::{error, RespVO};
use static_remote::{reqwest, settings::NexusRegionSetting, url};

//=========================
pub struct NexusApi(NexusRegionSetting);
//=========================

impl NexusApi {
    pub fn new(nexus: NexusRegionSetting) -> Self {
        Self(nexus)
    }

    pub fn client_builder(&self) -> Result<reqwest::Client, error::ApiReqError> {
        let cli = reqwest::Client::builder().build()?;
        Ok(cli)
    }

    /// Get the host of the Nexus API.
    pub fn download_component(&self) -> &str {
        let path = "/api/";
        path
    }

    // /// upload a single package
    // pub async fn put_component_raw(
    //     &self,
    //     dpm: &model::Raw,
    // ) -> Result<RespVO<serde_json::Value>, error::ApiReqError> {
    //     let path = "/api/artifactory/add";
    //     let addr = url::Url::parse(&self.host)?.join(&path)?;
    //     let cli = self.client_builder()?;
    //     let resp = cli
    //         .put(addr)
    //         .header("Content-Type", "application/json")
    //         .header("Cookie", self.cookie.as_str())
    //         .body(json!(dpm).to_string())
    //         .send()
    //         .await?
    //         .json::<RespVO<serde_json::Value>>()
    //         .await?;
    //     Ok(resp)
    // }
}
