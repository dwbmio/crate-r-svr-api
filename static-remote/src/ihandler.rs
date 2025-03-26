use crate::{
    error::{DownloadResult, FileSyncError, UploadResult},
    RemoteFileInfo,
};
use tokio::sync::mpsc::Sender;

pub trait RemoteFileHandler<T> {
    fn set_plat_config(&mut self, conf: T);

    async fn upload(&self, url_list: &RemoteFileInfo, process: Sender<Vec<u32>>) -> UploadResult;

    async fn download(
        &self,
        url_list: &RemoteFileInfo,
        process: Sender<Vec<u32>>,
    ) -> DownloadResult;

    async fn exec_download(
        &self,
        url_list: &RemoteFileInfo,
        process: Sender<Vec<u32>>,
    ) -> Result<String, FileSyncError>;

    async fn exec_upload(
        &self,
        url: &RemoteFileInfo,
        process: Sender<Vec<u32>>,
    ) -> Result<String, FileSyncError>;
}
