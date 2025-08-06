use error::{DownloadResult, FileSyncError};
use handler::http::HttpHandler;
use ihandler::RemoteFileHandler;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{collections::VecDeque, sync::Arc, thread};
use tokio::runtime::Runtime;
mod handler;
mod ihandler;
mod tui;

pub use aws_sdk_s3;
pub use reqwest;
pub use thiserror;
pub use url;

pub use settings::{HttpRegionSetting, MultiSetting, S3RegionSetting};
pub mod error;
pub mod middler;
pub mod settings;
pub use handler::s3::{self, S3Handler};
pub use handler::EFileSchema;

use crate::error::ApiReqError;
use crate::handler::nexus::NexusHandler;
use crate::settings::NexusRegionSetting;
use crate::tui::GuiCmd;

pub type ApiResult = Result<serde_json::Value, ApiReqError>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RespVO<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub code: u16,
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HttpRpcCore {
    pub host: String,
    pub cookie: Option<String>,
}

/// 单个文件的下载关联信息
/// 一个经典的配置如:
/// ```rust
/// let i = DwInfo {
///     link: "plugins/ffmpeg@1.0.0.zip".to_owned(),
///     write_path: "ffmpeg@1.0.0.zip".to_owned(),
///     schema: "s3".into()
/// };
///
/// ```
#[derive(Default, Debug, Clone)]
pub struct RemoteFileInfo {
    pub link: String,
    pub write_path: String,
    pub schema: EFileSchema,
}

impl RemoteFileInfo {
    pub fn new(link: &str, write_path: &str, scheme: &str) -> RemoteFileInfo {
        Self {
            link: link.to_owned(),
            write_path: write_path.to_owned(),
            schema: scheme.into(),
        }
    }
}

/// 下载管理器
/// * 管理队列
/// * 管理处理器
#[allow(dead_code)]
pub struct RFileSyncer {
    // gui
    gui: tui::GuiCmd,
    //队列
    up_list: VecDeque<RemoteFileInfo>,
    dw_list: VecDeque<RemoteFileInfo>,
    //配置
    multi_setting: MultiSetting,
    cur_dw_process: usize,
    //handlers
    s3_handler: Option<Arc<s3::S3Handler>>,
    http_handler: Option<Arc<HttpHandler>>,
    nexus_handler: Option<Arc<NexusHandler>>,
}

impl Default for RFileSyncer {
    fn default() -> Self {
        Self {
            gui: GuiCmd::new(),
            up_list: VecDeque::new(),
            dw_list: VecDeque::new(),
            cur_dw_process: 0,
            multi_setting: MultiSetting::default(),
            s3_handler: None,
            http_handler: None,
            nexus_handler: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExecAction {
    Download,
    Up,
}

impl RFileSyncer {
    ///
    /// 创建tokio的运行时
    fn inc_tokio_runtime() -> Runtime {
        let tokio_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();
        tokio_runtime
    }

    ///
    /// 设置s3关联的配置并创建handler
    pub fn set_s3_config(&mut self, config: &S3RegionSetting) {
        let s3_handler = Arc::new(S3Handler {
            setting: config.to_owned(),
        });
        self.s3_handler = Some(s3_handler);
    }

    /// 设置nexus关联的配置并创建handler
    pub fn set_nexus_config(&mut self, config: &NexusRegionSetting) {
        let nexus_handler = Arc::new(NexusHandler {
            setting: config.to_owned(),
        });
        self.nexus_handler = Some(nexus_handler);
    }

    ///扩容：上传队列
    pub fn append_up(&mut self, info: RemoteFileInfo) -> Result<&Self, FileSyncError> {
        let loc_file = Path::new(&info.link).to_path_buf();
        if loc_file.is_file() {
            self.up_list.push_back(info.clone());
        }
        log::debug!("append_up: {}", info.link);
        Ok(self)
    }

    ///扩容：下载队列
    pub fn append_down(&mut self, info: RemoteFileInfo) -> Result<&Self, FileSyncError> {
        let ret = match &info.schema {
            EFileSchema::S3 => Some((&info.link).to_string()),
            EFileSchema::Nexus => Some((&info.link).to_string()),
            EFileSchema::Http => Some(
                reqwest::Url::parse(&info.link)
                    .map_err(|e| {
                        error::FileSyncError::SyncFailed(format!(
                            "url parse error!url = {}, err = {}",
                            &info.link.to_owned(),
                            e.to_string()
                        ))
                    })?
                    .to_string(),
            ),
            EFileSchema::Unknown => None,
        };
        if let Some(_url) = ret {
            self.dw_list.push_back(info.clone());
        }
        Ok(self)
    }

    pub fn set_download_list(&mut self, l: Vec<RemoteFileInfo>) {
        self.dw_list = VecDeque::from(l);
    }

    pub fn set_upload_list(&mut self, l: Vec<RemoteFileInfo>) {
        self.up_list = VecDeque::from(l);
    }

    #[allow(dead_code)]
    async fn upload_handler<T>() {}

    async fn sync_handler<T, U: RemoteFileHandler<T>>(
        handler: Arc<U>,
        dw: &RemoteFileInfo,
        action: ExecAction,
        process: tokio::sync::mpsc::Sender<Vec<u32>>,
    ) -> DownloadResult {
        let loc = match action {
            ExecAction::Download => handler.download(&dw, process).await?,
            ExecAction::Up => handler.upload(&dw, process).await?,
        };
        return Ok(loc);
    }

    fn exec_once(mut self, action: ExecAction) -> Vec<Result<String, FileSyncError>> {
        let mut tar_list = match action {
            ExecAction::Download => self.dw_list,
            ExecAction::Up => self.up_list,
        };
        let cnt = &tar_list.len();
        let sync_list: Vec<RemoteFileInfo> = tar_list.drain(..*cnt).collect();

        let download_threads = thread::spawn(move || {
            let tokio_runtime = Self::inc_tokio_runtime();

            let ret_list: Vec<Result<String, FileSyncError>> = (0..sync_list.len())
                .map(move |i| {
                    let (tx, rx) = tokio::sync::mpsc::channel::<Vec<u32>>(1);
                    let arc_pb = self.gui.append_bar(
                        &tokio_runtime,
                        Arc::new(sync_list[i].clone()),
                        (i + 1) as u32,
                        sync_list.len() as u32,
                        rx,
                    );
                    let out = tokio_runtime.block_on(async {
                        log::debug!("exec once: {:?}", sync_list[i]);
                        let o = match sync_list[i].schema {
                            EFileSchema::S3 => {
                                let out = Self::sync_handler::<S3RegionSetting, S3Handler>(
                                    self.s3_handler.as_ref().unwrap().clone(),
                                    &sync_list[i],
                                    action,
                                    tx,
                                )
                                .await;
                                out
                            }

                            EFileSchema::Nexus => {
                                log::debug!("nexus here");
                                let out = Self::sync_handler::<NexusRegionSetting, NexusHandler>(
                                    self.nexus_handler.as_ref().unwrap().clone(),
                                    &sync_list[i],
                                    action,
                                    tx,
                                )
                                .await;
                                out
                            }

                            EFileSchema::Http | EFileSchema::Unknown => {
                                let out = Self::sync_handler::<HttpRegionSetting, HttpHandler>(
                                    self.http_handler.as_ref().unwrap().clone(),
                                    &sync_list[i],
                                    action,
                                    tx,
                                )
                                .await;
                                out
                            }
                        };
                        arc_pb.set_message(format!("s3\t|==>>loading{}", sync_list[i].link));
                        o
                    });
                    self.cur_dw_process += 1;
                    match &out {
                        Ok(o) => {
                            let log_out = match action {
                                ExecAction::Download => format!("Saved => {}", o),
                                ExecAction::Up => {
                                    format!("Uploaded[{:?}] => {}", sync_list[i].schema, o)
                                }
                            };
                            arc_pb.finish_with_message(log_out);
                        }
                        Err(e) => {
                            arc_pb.finish_with_message(format!("Error => {}", e.to_string()));
                        }
                    };
                    Ok(out?)
                })
                .collect();
            ret_list
        });
        let loc_list = download_threads.join().expect("Download thread raise out!");
        loc_list
    }

    pub fn exec_upload(self) -> Vec<Result<String, FileSyncError>> {
        self.exec_once(ExecAction::Up)
    }

    pub fn exec_download(self) -> Vec<Result<String, FileSyncError>> {
        self.exec_once(ExecAction::Download)
    }
}
