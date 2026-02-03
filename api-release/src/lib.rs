//! HFrog Release API 客户端
//!
//! 提供与 HFrog 服务 Release API 的交互功能。
//!
//! ## 功能
//! - 软件 (Software) 管理：创建、查询、更新
//! - 平台 (Platform) 管理：创建
//! - 版本 (Version) 管理：创建、设置最新版本
//! - 发布 (Release) 管理：创建/更新、查询
//!
//! ## 示例
//! ```rust,ignore
//! use api_release::ReleaseApi;
//!
//! let api = ReleaseApi::new("https://hfrog.gamesci-lite.com");
//! let software = api.get_software("hfrog").await?;
//! ```

pub mod model;

pub use model::*;
pub use static_remote::{error, RespVO};
use static_remote::{reqwest, url};

/// HFrog Release API 客户端
pub struct ReleaseApi {
    host: String,
}

impl ReleaseApi {
    /// 创建新的 API 客户端
    ///
    /// # 参数
    /// - `host`: HFrog 服务地址，如 `https://hfrog.gamesci-lite.com`
    pub fn new(host: impl Into<String>) -> Self {
        Self { host: host.into() }
    }

    fn client_builder(&self) -> Result<reqwest::Client, error::ApiReqError> {
        let cli = reqwest::Client::builder().build()?;
        Ok(cli)
    }

    // ========================================================================
    // Software APIs
    // ========================================================================

    /// 获取软件详情
    ///
    /// 根据软件名称查询软件信息，包含该软件的所有版本及各版本的平台发布信息。
    ///
    /// # 参数
    /// - `name`: 软件名称
    ///
    /// # 返回
    /// 返回 `RespVO<SoftwareInfo>`，包含软件信息和版本列表
    pub async fn get_software(
        &self,
        name: &str,
    ) -> Result<RespVO<SoftwareInfo>, error::ApiReqError> {
        let path = format!("/api/release/softwares/{}", name);
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            .send()
            .await?
            .json::<RespVO<SoftwareInfo>>()
            .await?;
        Ok(resp)
    }

    /// 创建软件
    ///
    /// # 参数
    /// - `req`: 创建软件请求
    ///
    /// # 返回
    /// 返回新建的软件信息，如果软件已存在返回错误
    pub async fn create_software(
        &self,
        req: &CreateSoftwareReq,
    ) -> Result<RespVO<SoftwareInfo>, error::ApiReqError> {
        let path = "/api/release/softwares";
        let addr = url::Url::parse(&self.host)?.join(path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .post(addr)
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?
            .json::<RespVO<SoftwareInfo>>()
            .await?;
        Ok(resp)
    }

    /// 更新软件信息
    ///
    /// # 参数
    /// - `name`: 软件名称
    /// - `req`: 更新请求（仅更新非空字段）
    pub async fn update_software(
        &self,
        name: &str,
        req: &UpdateSoftwareReq,
    ) -> Result<RespVO<SoftwareInfo>, error::ApiReqError> {
        let path = format!("/api/release/softwares/{}", name);
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .put(addr)
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?
            .json::<RespVO<SoftwareInfo>>()
            .await?;
        Ok(resp)
    }

    // ========================================================================
    // Platform APIs
    // ========================================================================

    /// 创建平台
    ///
    /// # 参数
    /// - `req`: 创建平台请求
    ///
    /// # 返回
    /// 返回新建的平台信息，如果平台已存在返回错误
    pub async fn create_platform(
        &self,
        req: &CreatePlatformReq,
    ) -> Result<RespVO<PlatformInfo>, error::ApiReqError> {
        let path = "/api/release/platforms";
        let addr = url::Url::parse(&self.host)?.join(path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .post(addr)
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?
            .json::<RespVO<PlatformInfo>>()
            .await?;
        Ok(resp)
    }

    // ========================================================================
    // Version APIs
    // ========================================================================

    /// 创建版本
    ///
    /// # 参数
    /// - `req`: 创建版本请求
    ///
    /// # 返回
    /// 返回新建的版本信息
    pub async fn create_version(
        &self,
        req: &CreateVersionReq,
    ) -> Result<RespVO<VersionInfo>, error::ApiReqError> {
        let path = "/api/release/versions";
        let addr = url::Url::parse(&self.host)?.join(path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .post(addr)
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?
            .json::<RespVO<VersionInfo>>()
            .await?;
        Ok(resp)
    }

    /// 设置最新版本
    ///
    /// # 参数
    /// - `req`: 设置最新版本请求
    pub async fn set_latest_version(
        &self,
        req: &SetLatestVersionReq,
    ) -> Result<RespVO<VersionInfo>, error::ApiReqError> {
        let path = "/api/release/versions/latest";
        let addr = url::Url::parse(&self.host)?.join(path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .put(addr)
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?
            .json::<RespVO<VersionInfo>>()
            .await?;
        Ok(resp)
    }

    // ========================================================================
    // Release APIs
    // ========================================================================

    /// 创建或更新发布
    ///
    /// 为指定软件的指定版本创建一个平台发布记录。
    /// 如果该版本在该平台的发布已存在，则更新下载链接等信息。
    ///
    /// # 参数
    /// - `req`: 创建发布请求
    ///
    /// # 返回
    /// 返回发布信息
    pub async fn create_release(
        &self,
        req: &CreateReleaseReq,
    ) -> Result<RespVO<ReleaseInfo>, error::ApiReqError> {
        let path = "/api/release/releases";
        let addr = url::Url::parse(&self.host)?.join(path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .post(addr)
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?
            .json::<RespVO<ReleaseInfo>>()
            .await?;
        Ok(resp)
    }

    /// 获取发布信息
    ///
    /// 查询指定软件指定版本的所有平台发布信息。
    ///
    /// # 参数
    /// - `software_name`: 软件名称
    /// - `version`: 版本号
    ///
    /// # 返回
    /// 返回该版本所有平台的发布信息
    pub async fn get_release(
        &self,
        software_name: &str,
        version: &str,
    ) -> Result<RespVO<VersionInfo>, error::ApiReqError> {
        let path = format!("/api/release/releases/{}/{}", software_name, version);
        let addr = url::Url::parse(&self.host)?.join(&path)?;
        let cli = self.client_builder()?;
        let resp = cli
            .get(addr)
            .send()
            .await?
            .json::<RespVO<VersionInfo>>()
            .await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_software() {
        let api = ReleaseApi::new("https://hfrog.gamesci-lite.com");
        let resp = api.get_software("hfrog").await;
        println!("{:?}", resp);
    }
}
