use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use reqwest_tracing::TracingMiddleware;
use crate::{
    error::{self, DownloadResult, FileSyncError, UploadResult},
    ihandler::RemoteFileHandler,
    settings::HttpRegionSetting,
    RemoteFileInfo,
};

pub struct HttpHandler {
    pub setting: HttpRegionSetting,
}

impl RemoteFileHandler<HttpRegionSetting> for HttpHandler {

    async fn upload(&self, info: &RemoteFileInfo, process: tokio::sync::mpsc::Sender<Vec<u32>>) -> UploadResult {
        Err(error::FileSyncError::SyncFailed("Http not support upload file!".to_owned()))
    }

    async fn download(
        &self,
        dinfo: &RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> DownloadResult {
        let f_loc = self.exec_download(dinfo, process).await?;
        Ok(f_loc)
    }

    async fn exec_download(
        &self,
        dinfo: &RemoteFileInfo,
        psender: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> Result<String, FileSyncError> {
        use futures_util::StreamExt;
        use tokio::{fs::File, io::AsyncWriteExt};

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let req_cli = ClientBuilder::new(reqwest::Client::new())
            .with(TracingMiddleware {})
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        let resp = req_cli.get(dinfo.link.to_owned()).send().await?;
        let mut path_save = File::create(dinfo.write_path.to_owned())
            .await
            .map_err(|e| error::FileSyncError::SyncFailed(format!("err = {}", e.to_string())))?;

        let max_content_l = resp.content_length().unwrap_or_default();
        let mut stream = resp.bytes_stream();
        let mut process: u64 = 0;
        while let Some(item) = stream.next().await {
            let chunk = item.map_err(|e| error::FileSyncError::SyncFailed(e.to_string()))?;
            path_save.write_all(&chunk).await.map_err(|e| {
                error::FileSyncError::SyncFailed(format!("err = {}", e.to_string()))
            })?;
            process += u64::try_from(chunk.len()).unwrap_or_default();
            let _ = psender
                .send(vec![
                    process.try_into().unwrap(),
                    max_content_l.try_into().unwrap(),
                ])
                .await;
        }
        path_save
            .sync_all()
            .await
            .map_err(|e| error::FileSyncError::SyncFailed(format!("err = {}", e.to_string())))?;
        log::debug!("write file = {} suc!", dinfo.write_path);
        Ok(dinfo.write_path.to_owned())
    }

    fn set_plat_config(&mut self, conf: HttpRegionSetting) {
        self.setting = conf;
    }
    
    async fn exec_upload(
        &self,
        url: &RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> Result<String, FileSyncError> {
        // do nothing
        Ok("".to_owned())
    }
}
