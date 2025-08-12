#![allow(dead_code)]
use serde;
use std::env;

const S3_ENV_KEYS: [&str; 5] = [
    "S3_ACCESS_KEY",
    "S3_SECRET_KEY",
    "S3_ENDPOINT",
    "S3_BUCKET",
    "S3_REGION",
];

// #region Nexus
///Nexus配置
#[derive(Debug, Clone, serde::Serialize)]
pub struct NexusRegionSetting {
    pub endpoint: String,
    pub repository: String,
    pub user_name: String,
    pub password: String,
}

impl std::fmt::Display for NexusRegionSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r"[nexus] \
endpoint = {endpoint} \
repository = {repository} \
user_name = {user_name} \
password = {password}",
            endpoint = &self.endpoint,
            repository = &self.repository,
            user_name = &self.user_name,
            password = &self.password
        )
    }
}

impl NexusRegionSetting {
    pub fn from_env() -> Self {
        Self {
            endpoint: env::var("NEXUS_ENDPOINT").unwrap_or_default(),
            repository: env::var("NEXUS_REPO").unwrap_or_default(),
            user_name: env::var("NEXUS_USERNAME").unwrap_or_default(),
            password: env::var("NEXUS_PASSWORD").unwrap_or_default(),
        }
    }
}

//#endregion

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

    pub fn clear_env_vars() {
        // 清理现有的环境变量
        let mut tmp = S3_ENV_KEYS.into_iter();
        while let Some(key) = tmp.next() {
            std::env::remove_var(key);
        }
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
