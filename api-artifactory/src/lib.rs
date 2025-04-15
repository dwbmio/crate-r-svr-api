pub mod model;
pub mod up_ex;
mod runtime;
pub use rsvr_core::{error, RespVO};
use rsvr_core::{reqwest, url};
use serde_json::json;


//=========================
pub struct ArtifactoryApi {
    host: String,
    cookie: String,
}
//=========================

impl ArtifactoryApi {
    pub fn new(host: String, cookie: String) -> Self {
        Self { host, cookie }
    }

    pub fn client_builder(&self) -> Result<reqwest::Client, error::ApiReqError> {
        let cli = reqwest::Client::builder().build()?;
        Ok(cli)
    }

    ///
    /// 根据name和ver查询pkg的信息
    /// HTTP GET
    pub async fn get_find_pkg(
        &self,
        pkg: &str,
    ) -> Result<RespVO<serde_json::Value>, error::ApiReqError> {
        #[allow(unused_assignments)]
        let mut info: (&str, &str) = ("", "");
        if pkg.find("@").is_some() {
            let list = pkg.split("@").collect::<Vec<&str>>();
            info = (list[0], list[1]);
        } else {
            info = (pkg, "^");
        }

        let path = format!(
            "/api/artifactory/find?name={name}&ver={ver}",
            name = info.0,
            ver = info.1
        );
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            .header("Cookie", self.cookie.as_str())
            .send()
            .await?
            .json::<RespVO<serde_json::Value>>()
            .await?;
        Ok(resp)
    }

    ///
    /// 获取pkg列表
    /// HTTP GET
    pub async fn get_pkg_list(
        &self,
        rt_name: &str,
    ) -> Result<RespVO<serde_json::Value>, error::ApiReqError> {
        #[allow(unused_assignments)]
        let path = format!("/api/artifactory/list?runtime={rt_name}");
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            .header("Cookie", self.cookie.as_str())
            .send()
            .await?
            .json::<RespVO<serde_json::Value>>()
            .await?;
        Ok(resp)
    }

    pub async fn get_artifactory_download_url(
        &self,
        id: i32,
    ) -> Result<RespVO<serde_json::Value>, error::ApiReqError> {
        let path = format!("/api/artifactory/get_object_presigned_url?id={}", id);
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            .header("Cookie", self.cookie.as_str())
            .send()
            .await?
            .json::<RespVO<serde_json::Value>>()
            .await?;
        Ok(resp)
    }

    /// 添加一个package
    pub async fn put_artifactory_add(
        &self,
        dpm: &model::ArtifactoryCellInfo,
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
