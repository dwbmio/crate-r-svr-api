pub mod model;
mod runtime;
pub use rsvr_core::error;
use rsvr_core::{middler, reqwest, url, RespVO};
use serde_json::json;

//=========================
pub struct DpmSvr {
    host: String,
    cookie: String,
}
//=========================

impl DpmSvr {
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
    pub async fn get_pkg_info(&self, pkg: &str) -> Result<serde_json::Value, error::ApiReqError> {
        #[allow(unused_assignments)]
        let mut info: (&str, &str) = ("", "");
        if pkg.find("@").is_some() {
            let list = pkg.split("@").collect::<Vec<&str>>();
            info = (list[0], list[1]);
        } else {
            info = (pkg, "^");
        }

        let path = format!(
            "/api/dpm/info?name={name}&ver={ver}",
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
        let r = middler::map_respvo_data(resp);
        r
    }


    ///
    /// 获取pkg列表
    /// HTTP GET
    pub async fn get_pkg_list(&self, rt_name: &str) -> Result<serde_json::Value, error::ApiReqError> {
        #[allow(unused_assignments)]
        let path = format!(
            "/api/dpm/list?runtime={rt_name}"
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
        let r = middler::map_respvo_data(resp);
        r
    }


    /// 添加一个package
    pub async fn put_pkg_add(
        &self,
        dpm: &model::ModelDpm,
    ) -> Result<serde_json::Value, error::ApiReqError> {
        let path = "/api/dpm/add";
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .put(addr)
            .header("Content-Type", "application/json")
            .header("Cookie", self.cookie.as_str())
            .body(json!(dpm).to_string())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(resp)
    }
}
