pub use static_remote::error::ApiReqError;
use static_remote::{error, reqwest, url, HttpRpcCore};
//=========================
#[derive(serde::Serialize)]
pub struct PicbooSvr {
    http_rpc: HttpRpcCore,
    authkey: Option<String>,
}
//=========================

fn chk_illegal_isbn(isbn: &str) -> Result<(), ApiReqError> {
    let _l = isbn.len();
    println!("isbn len is :{}", _l);
    if isbn.len() != 13 && isbn.len() != 10 {
        return Err(ApiReqError::CustomError("params:isbn illegal".to_owned()));
    };
    Ok(())
}

impl PicbooSvr {
    pub fn from(host: String, cookie: Option<String>, authkey: Option<String>) -> Self {
        Self {
            http_rpc: HttpRpcCore { host, cookie },
            authkey,
        }
    }

    pub fn new(http_rpc: HttpRpcCore, authkey: Option<String>) -> Self {
        Self { http_rpc, authkey }
    }

    pub fn set_host(&mut self, host: &str) {
        self.http_rpc.host = host.to_owned();
    }

    pub fn set_authkey(&mut self, auth_key: &str) {
        self.http_rpc.cookie = Some(auth_key.to_owned());
    }

    pub fn client_builder(&self) -> Result<reqwest::Client, error::ApiReqError> {
        let cli = reqwest::Client::builder().build()?;
        Ok(cli)
    }

    pub async fn isbn_add(&self, isbn: &str) -> Result<serde_json::Value, ApiReqError> {
        chk_illegal_isbn(isbn)?;
        let addr = url::Url::parse(&self.http_rpc.host)?
            .join("/api/picboo/isbn/add/")?
            .join(isbn)?;
        let cli = self.client_builder()?;
        let resp = cli
            .post(addr)
            .header("Content-Type", "application/json")
            .header("Cookie", self.http_rpc.cookie.clone().unwrap_or_default())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(resp)
    }

    pub async fn find_isbn(&self, isbn: &str) -> Result<serde_json::Value, ApiReqError> {
        chk_illegal_isbn(isbn)?;
        let addr = url::Url::parse(&self.http_rpc.host)?
            .join("/api/picboo/isbn/find/")?
            .join(isbn)?;
        println!("req to url=>{:?}", addr.to_string());
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            .header("Content-Type", "application/json")
            .header("Cookie", self.http_rpc.cookie.clone().unwrap_or_default())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(resp)
    }
}
