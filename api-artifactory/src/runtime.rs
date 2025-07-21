use crate::{model, ArtifactoryApi};
use serde_json::json;
use static_remote::{error, url, RespVO};

impl ArtifactoryApi {
    ///
    /// 新增一个runtime类型
    /// HTTP POST
    pub async fn post_add_runtime(
        &self,
        runtime_name: &str,
        ver_str: &str,
    ) -> Result<RespVO<serde_json::Value>, error::ApiReqError> {
        let addr = url::Url::parse(&self.host)?.join("/api/artifactory/runtime/add")?;
        //cli
        let cli = self.client_builder()?;
        let n_r = model::Runtime {
            runtime: runtime_name,
            ver_str: ver_str,
        };

        let resp = cli
            .put(addr)
            .header("Content-Type", "application/json")
            .header("Cookie", self.cookie.as_str())
            .body(json!(n_r).to_string())
            .send()
            .await?
            .json::<RespVO<serde_json::Value>>()
            .await?;

        Ok(resp)
    }

    ///
    /// 获取runtime的列表(不长，默认全返回
    /// HTTP GET
    pub async fn get_runtime_list(
        &self,
        page_idx: u64,
        page_cnt: u64,
    ) -> Result<RespVO<serde_json::Value>, error::ApiReqError> {
        let mut addr = url::Url::parse(&self.host)?.join("/api/artifactory/runtime/list")?;
        addr.set_query(Some(&format!("index={}&cnt={}", page_idx, page_cnt)));
        //cli
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            .header("Content-Type", "application/json")
            .header("Cookie", self.cookie.as_str())
            .send()
            .await?
            .json::<RespVO<serde_json::Value>>()
            .await?;
        Ok(resp)
    }

    ///
    /// 获取指定runtime的所有版本
    /// HTTP GET
    pub async fn get_runtime_one(
        &self,
        name: &str,
    ) -> Result<RespVO<serde_json::Value>, error::ApiReqError> {
        //cli
        let cli = self.client_builder()?;
        let path = format!("/api/artifactory/runtime/one?name={name}", name = name,);
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let resp = cli
            .get(addr)
            .send()
            .await?
            .json::<RespVO<serde_json::Value>>()
            .await?;
        Ok(resp)
    }
}
