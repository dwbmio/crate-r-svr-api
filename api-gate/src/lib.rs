pub use rsvr_core::error::ApiReqError;
use rsvr_core::{
    error::{self},
    reqwest, url,
};
//=========================
#[derive(serde::Serialize)]
pub struct GateSvr {
    host: String,
    cookie: Option<String>,
}
//=========================
impl GateSvr {
    pub fn new(host: String, cookie: Option<String>) -> Self {
        Self { host, cookie }
    }

    pub fn set_host(&mut self, host: &str) {
        self.host = host.to_owned();
    }

    pub fn set_authkey(&mut self, auth_key: &str) {
        self.cookie = Some(auth_key.to_owned());
    }

    pub fn client_builder(&self) -> Result<reqwest::Client, error::ApiReqError> {
        let cli = reqwest::Client::builder().build()?;
        Ok(cli)
    }

    /// 检查版本更新
    pub async fn get_checkupdate(
        &self,
        appid: &str,
        vnum: u32,
    ) -> Result<serde_json::Value, ApiReqError> {
        let addr = url::Url::parse(&self.host)?.join(
            format!(
                "/api/app_ver/{appid}/checkupdate/{vnum}",
                appid = appid,
                vnum = vnum.to_string()
            )
            .as_str(),
        )?;
        println!("url value is :{}", addr);
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            // .header("Content-Type", "application/json")
            // .header("Cookie", self.cookie.clone().unwrap_or_default())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(resp)
    }

    pub async fn get_appevent(
        &self,
        appid: &str,
        event: &str,
    ) -> Result<serde_json::Value, ApiReqError> {
        let addr = url::Url::parse(&self.host)?.join(
            format!(
                "/api/app_event/{appid}/{event}/all",
                appid = appid,
                event = event
            )
            .as_str(),
        )?;
        println!("url value is :{}", addr);
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            // .header("Content-Type", "application/json")
            // .header("Cookie", self.cookie.clone().unwrap_or_default())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(resp)
    }

    ///获取app对应啊的最新的版本号
    pub async fn get_latest_appvers(
        &self,
        appid: &str,
        ver: &str,
    ) -> Result<serde_json::Value, ApiReqError> {
        let addr = url::Url::parse(&self.host)?.join(
            format!(
                "/api/app_events/{appid}/{ver}/all",
                appid = appid,
                ver = ver
            )
            .as_str(),
        )?;
        println!("url value is :{}", addr);
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            // .header("Content-Type", "application/json")
            // .header("Cookie", self.cookie.clone().unwrap_or_default())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(resp)
    }
}
