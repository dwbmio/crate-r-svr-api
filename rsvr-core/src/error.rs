use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiReqError {
    #[error("reqwest call error!error={0}")]
    ReqError(#[from] reqwest::Error),

    #[error("{0}")]
    CustomError(String),

    #[error("{0}")]
    RespError(String, u16),

    #[error("url parse failed!")]
    UrlError(#[from] url::ParseError),

    #[error("req caceh failed!")]
    CacheError(String),

    #[error("download file failed!error={0}")]
    DownloadFailed(String),
}
