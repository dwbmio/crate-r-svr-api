use indicatif::style::TemplateError;
use std::fmt;
use std::io;
use thiserror::Error;

///
/// 下载结果
pub type DownloadResult = Result<String, FileSyncError>;

///
/// 上传结果
pub type UploadResult = Result<String, FileSyncError>;

///zpm运行时的错误返回
#[derive(Debug, Error)]
pub enum FileSyncError {
    #[error("Pkm config in error fmt.")]
    FmtError(#[from] fmt::Error),
    #[error("File read/write error!")]
    IoError(#[from] io::Error),
    #[error("Download pm failed!error={0}")]
    SyncFailed(String),
    #[error("Login Failed")]
    LoginFiled(String),
    #[error("LoggerCtor failed")]
    LoggerError(#[from] TemplateError),
    #[cfg(feature = "http")]
    #[error("error={0}")]
    ReqwestError(#[from] reqwest::Error),
    #[cfg(feature = "http")]
    #[error("reqwest error! error={0}")]
    RequestMiddleError(#[from] reqwest_middleware::Error),
    #[error("aws inner error!err_str: {0}")]
    AwsS3SDKError(String),
}

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
