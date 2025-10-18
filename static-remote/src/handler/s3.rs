use crate::{
    error::{DownloadResult, FileSyncError, UploadResult},
    ihandler::RemoteFileHandler,
    settings::S3RegionSetting,
    ExecAction, RemoteFileInfo,
};
use aws_config::Region;
use aws_sdk_s3::{
    config::Credentials, presigning::PresigningConfig, primitives::ByteStream, Client, Config,
};
use std::{path::Path, time::Duration};
use tokio::{fs::File, io::AsyncReadExt};

#[allow(unused)]
pub struct S3Handler {
    pub setting: S3RegionSetting,
}

impl S3Handler {
    pub fn new(st: S3RegionSetting) -> Self {
        Self { setting: st }
    }

    fn s3_client(&self) -> Client {
        let region_set = &self.setting;
        let credentials = Credentials::new(
            region_set.access_key.to_owned(),
            region_set.access_sec.to_owned(),
            None,
            None,
            "",
        );
        let aws_cfg = Config::builder()
            .region(Region::new(region_set.region.clone().unwrap_or_default()))
            .endpoint_url(region_set.end_point.to_owned())
            .behavior_version_latest()
            // oss 要求false
            // qiniu + minio是true
            .force_path_style(!region_set.end_point.find("oss").is_some())
            .credentials_provider(credentials)
            .build();
        let cli = aws_sdk_s3::client::Client::from_conf(aws_cfg);
        cli
    }
}

impl RemoteFileHandler<S3RegionSetting> for S3Handler {
    async fn upload(
        &self,
        info: &RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> UploadResult {
        let f_s3 = self.exec_upload(info, process).await?;
        Ok(f_s3)
    }

    async fn download(
        &self,
        info: &RemoteFileInfo,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> DownloadResult {
        let f_s3 = self.exec_download(info, process).await?;
        Ok(f_s3)
    }

    async fn exec_download(
        &self,
        info: &RemoteFileInfo,
        _process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> Result<String, FileSyncError> {
        let cli = self.s3_client();
        let f = cli
            .get_object()
            .bucket(self.setting.bucket.to_owned())
            .key(&info.link)
            .send()
            .await
            .map_err(|e| FileSyncError::AwsS3SDKError(e.to_string()))?;

        // 将字节流写入文件
        let path_to_write = Path::new(&info.write_path);
        let mut file = tokio::fs::File::create(path_to_write).await?;
        let mut stream = f.body.into_async_read();
        tokio::io::copy(&mut stream, &mut file).await?;
        Ok(path_to_write.to_path_buf().display().to_string())
    }

    fn set_plat_config(&mut self, conf: S3RegionSetting) {
        self.setting = conf;
    }

    async fn exec_upload(
        &self,
        info: &RemoteFileInfo,
        _process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> Result<String, FileSyncError> {
        let cli = self.s3_client();
        // 将文件上传到 S3
        let mut loc_file = File::open(&info.link).await?;
        let mut buffer = Vec::new();
        loc_file.read_to_end(&mut buffer).await?;
        let body = ByteStream::from(buffer);
        cli.put_object()
            .bucket(self.setting.bucket.clone())
            .key(info.write_path.clone())
            .body(body)
            .send()
            .await
            .map_err(|e| FileSyncError::AwsS3SDKError(e.to_string()))?;
        Ok(info.write_path.clone())
    }
}

impl S3Handler {
    /// 上传一个文件实体
    pub async fn upload_once(s3_setting: &S3RegionSetting, info: &RemoteFileInfo) -> UploadResult {
        let inc = S3Handler::new(s3_setting.to_owned());
        let (tx, _) = tokio::sync::mpsc::channel::<Vec<u32>>(1);
        let out = inc.exec_upload(info, tx).await?;
        Ok(out)
    }

    /// 上传bytes
    pub async fn upload_bytes(
        s3_setting: &S3RegionSetting,
        write_key: &str,
        bytes: ByteStream,
    ) -> UploadResult {
        let inc = S3Handler::new(s3_setting.to_owned());
        let cli = inc.s3_client();
        cli.put_object()
            .bucket(inc.setting.bucket.clone())
            .key(write_key)
            .body(bytes)
            .send()
            .await
            .map_err(|e| FileSyncError::AwsS3SDKError(e.to_string()))?;
        Ok(write_key.to_owned())
    }

    /// 返回一个私有bucket动作签名临时链接
    pub async fn action_object_url_with_presigned(
        s3_setting: &S3RegionSetting,
        bucket_name: &str,
        object_key: &str,
        action: ExecAction,
        timeout: Option<Duration>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let inc = S3Handler::new(s3_setting.to_owned());
        let def_timeout = Duration::from_secs(5);
        let url = match action {
            ExecAction::Download => {
                let get_object_request = inc // 4. 创建 GetObject 请求
                    .s3_client()
                    .get_object()
                    .bucket(bucket_name)
                    .key(object_key);
                let presigned_config =
                    PresigningConfig::expires_in(timeout.unwrap_or(def_timeout))?; // 5. 配置预签名 URL 的有效期
                let presigned_request = get_object_request.presigned(presigned_config).await?; // 6. 生成预签名 URL
                let url = presigned_request.uri().to_string();
                url
            }
            ExecAction::Up => {
                let put_object_request = inc // 4. 创建 GetObject 请求
                    .s3_client()
                    .put_object()
                    .bucket(bucket_name)
                    .key(object_key);
                let presigned_config =
                    PresigningConfig::expires_in(timeout.unwrap_or(def_timeout))?; // 5. 配置预签名 URL 的有效期
                let presigned_request = put_object_request.presigned(presigned_config).await?; // 6. 生成预签名 URL
                let url = presigned_request.uri().to_string();
                url
            }
        };
        Ok(url)
    }
}
