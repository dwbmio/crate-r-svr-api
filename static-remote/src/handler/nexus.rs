use crate::{ihandler::RemoteFileHandler, settings::NexusRegionSetting};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};
use strfmt::strfmt;
use tokio::{fs::File, io::AsyncReadExt};

pub struct NexusHandler {
    pub setting: NexusRegionSetting,
}

impl NexusHandler {
    pub fn new(setting: NexusRegionSetting) -> Self {
        NexusHandler { setting }
    }
}

impl NexusHandler {
    fn http_client(&self) -> reqwest::Client {
        let cli = reqwest::Client::new();
        cli
    }
}

#[allow(unused_variables)]
impl RemoteFileHandler<NexusRegionSetting> for NexusHandler {
    fn set_plat_config(&mut self, conf: NexusRegionSetting) {}

    async fn upload(
        &self,
        rf_info: &crate::RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> crate::error::UploadResult {
        let f_s3 = self.exec_upload(rf_info, process).await?;
        Ok(f_s3)
    }

    async fn download(
        &self,
        rf_info: &crate::RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> crate::error::DownloadResult {
        let f_s3 = self.exec_download(rf_info, process).await?;
        Ok(f_s3)
    }

    async fn exec_download(
        &self,
        info: &crate::RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> Result<String, crate::error::FileSyncError> {
        let cli = self.http_client();
        let write_p = PathBuf::from_str(&info.write_path).expect("Path for write is not invalid!");
        let url_down = "{endpoint}/repository/{repo_name}/{file_name}";
        let mut hm = HashMap::new();
        hm.insert("endpoint".to_string(), self.setting.endpoint.clone());
        hm.insert("repo_name".to_string(), self.setting.repository.clone());
        hm.insert("file_name".to_string(), info.link.clone());
        let url_put =
            strfmt(url_down, &hm).map_err(|e| crate::error::FileSyncError::UrlStrFmtError(e))?;
        let mut resp = cli.get(url_put).send().await?;
        let mut file = tokio::fs::File::create(&write_p).await?;
        while let Some(chunk) = resp.chunk().await? {
            tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
        }
        Ok(write_p.display().to_string())
    }

    async fn exec_upload(
        &self,
        info: &crate::RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> Result<String, crate::error::FileSyncError> {
        let cli = self.http_client();
        // Build multipart form
        let mut file = File::open(info.link.clone()).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        let file_name = Path::new(&info.link.clone())
            .file_name()
            .map(|os_str| os_str.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown-filename".to_owned());
        // Url
        let url_base = "{endpoint}/repository/{repo_name}/{file_name}";
        let mut hm = HashMap::new();
        hm.insert("endpoint".to_string(), self.setting.endpoint.clone());
        hm.insert("repo_name".to_string(), self.setting.repository.clone());
        hm.insert("file_name".to_string(), info.write_path.clone());
        let url_put =
            strfmt(url_base, &hm).map_err(|e| crate::error::FileSyncError::UrlStrFmtError(e))?;
        let out = cli
            .put(url_put)
            .basic_auth(
                self.setting.user_name.clone(),
                Some(self.setting.password.clone()),
            )
            .body(buffer)
            .send()
            .await?;
        if !out.status().is_success() {
            log::error!("【nexus】request error:{:#?}", out);
        }
        Ok(info.write_path.clone())
    }
}
