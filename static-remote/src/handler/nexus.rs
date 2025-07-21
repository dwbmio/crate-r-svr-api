use crate::{ihandler::RemoteFileHandler, settings::NexusRegionSetting};
use reqwest::{multipart, Url};
use std::path::Path;
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
        let f_s3 = self.exec_download(rf_info, process).await?;
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
        rf_info: &crate::RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> Result<String, crate::error::FileSyncError> {
        let cli = self.http_client();
        let component_download_url = Url::parse(format!("{}/", self.setting.endpoint).as_str());
        // log::debug!("")
        Ok("()".to_owned())
    }

    async fn exec_upload(
        &self,
        rf_info: &crate::RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> Result<String, crate::error::FileSyncError> {
        let cli = self.http_client();
        // Build multipart form
        let mut file = File::open(rf_info.link.clone()).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        let file_name = Path::new(&rf_info.link.clone())
            .file_name()
            .map(|os_str| os_str.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown-filename".to_owned());
        let part = multipart::Part::bytes(buffer).file_name(file_name);
        let form = multipart::Form::new().part("file", part);
        cli.post(&self.setting.endpoint)
            .basic_auth(&self.setting.user_name, Some(&self.setting.password))
            .multipart(form)
            .send()
            .await?;
        Ok(rf_info.write_path.clone())
    }
}
