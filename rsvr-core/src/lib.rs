use error::ApiReqError;
pub use reqwest;
use serde::{Deserialize, Serialize};
pub use thiserror;
pub use url;
pub mod error;
pub mod middler;
pub mod sdl;

pub type ApiResult = Result<serde_json::Value, ApiReqError>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RespVO<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub code: u16,
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HttpRpcCore {
    pub host: String,
    pub cookie: Option<String>,
}
