#![allow(dead_code)]
use std::env;

use serde;

///S3配置
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct S3RegionSetting {
    pub access_key: String,
    pub access_sec: String,
    pub end_point: String,
    pub bucket: String,
    pub region: Option<String>,
    pub path: Option<String>,
}

impl std::fmt::Display for S3RegionSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"[s3] \
        end_point = {end_point} \
        bucket = {bucket} \
        path = {path}",
            end_point = &self.end_point,
            bucket = &self.bucket,
            path = self.path.clone().unwrap_or_default()
        )
    }
}

/// HTTP特有配置：
#[derive(Default)]
pub struct HttpRegionSetting {}

impl S3RegionSetting {
    pub fn new(access_key: String, access_sec: String, end_point: String, bucket: String) -> Self {
        Self {
            access_key,
            access_sec,
            end_point,
            bucket,
            region: None,
            path: None,
        }
    }

    pub fn from_env() -> Self {
        Self {
            access_key: env::var("S3_ACCESS_KEY").unwrap_or("UNKNOWN S3_ACCESS_KEY".to_string()),
            access_sec: env::var("S3_SECRET_KEY").unwrap_or("UNKNOWN S3_SECRET_KEY".to_string()),
            end_point: env::var("S3_ENDPOINT").unwrap_or("UNKNOWN S3_ENDPOINT".to_string()),
            bucket: env::var("S3_BUCKET").unwrap_or("UNKNOWN S3_BUCKET".to_string()),
            region: Some(env::var("S3_REGION").unwrap_or("UNKNOWN S3_BUCKET".to_string())),
            path: Some(env::var("S3_PATH").unwrap_or_default()),
        }
    }

    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }
}

/// 下载配置：
/// * 并发数量
/// * 速度限制
pub struct MultiSetting {
    async_max: u8,
    speed_limit: Option<u32>,
}

impl Default for MultiSetting {
    fn default() -> Self {
        Self {
            async_max: 5,
            speed_limit: None,
        }
    }
}
