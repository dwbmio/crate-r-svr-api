//! HFrog Release API 数据模型

use serde::{Deserialize, Serialize};

// ============================================================================
// 响应数据结构
// ============================================================================

/// 软件信息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SoftwareInfo {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub install_script_url: Option<String>,
    pub install_command: Option<String>,
    #[serde(default)]
    pub versions: Vec<VersionInfo>,
}

/// 版本信息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VersionInfo {
    pub id: i32,
    pub version: String,
    pub is_latest: bool,
    pub release_notes: Option<String>,
    pub created_at: Option<String>,
    pub created_by: Option<String>,
    #[serde(default)]
    pub platforms: Vec<ReleaseInfo>,
}

/// 发布信息（版本 + 平台）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReleaseInfo {
    pub platform_code: String,
    pub platform_display_name: Option<String>,
    pub download_url: String,
    pub file_size: Option<i64>,
    pub checksum_sha256: Option<String>,
}

/// 平台信息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlatformInfo {
    pub id: i32,
    pub code: String,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub display_name: Option<String>,
}

// ============================================================================
// 请求数据结构
// ============================================================================

/// 创建软件请求
#[derive(Debug, Serialize, Clone)]
pub struct CreateSoftwareReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_script_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_command: Option<String>,
}

/// 更新软件请求
#[derive(Debug, Serialize, Clone)]
pub struct UpdateSoftwareReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_script_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_command: Option<String>,
}

/// 创建平台请求
#[derive(Debug, Serialize, Clone)]
pub struct CreatePlatformReq {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// 创建版本请求
#[derive(Debug, Serialize, Clone)]
pub struct CreateVersionReq {
    pub software_name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

/// 设置最新版本请求
#[derive(Debug, Serialize, Clone)]
pub struct SetLatestVersionReq {
    pub software_name: String,
    pub version: String,
}

/// 创建发布请求
#[derive(Debug, Serialize, Clone)]
pub struct CreateReleaseReq {
    pub software_name: String,
    pub version: String,
    pub platform_code: String,
    pub download_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_sha256: Option<String>,
}
