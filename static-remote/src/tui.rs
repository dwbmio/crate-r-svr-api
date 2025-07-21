use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

use crate::{EFileSchema, RemoteFileInfo};

pub struct GuiCmd {
    multi_progress: MultiProgress,
    spinner_style: ProgressStyle,
}

impl GuiCmd {
    pub fn new() -> Self {
        let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
        Self {
            spinner_style: spinner_style,
            multi_progress: MultiProgress::new(),
        }
    }

    pub fn append_bar(
        &mut self,
        runtime: &tokio::runtime::Runtime,
        arc_remote_file_info: Arc<RemoteFileInfo>,
        cur: u32,
        max: u32,
        mut rx: Receiver<Vec<u32>>,
    ) -> Arc<ProgressBar> {
        let pb: ProgressBar = self.multi_progress.add(ProgressBar::new(100));
        pb.set_style(self.spinner_style.clone());
        pb.set_prefix(format!("[{}/{}]|", cur, max));
        let arc_pb = Arc::new(pb);
        let log_pb = Arc::clone(&arc_pb);
        let cc = Arc::clone(&arc_remote_file_info);
        runtime.spawn(async move {
            let arc_info = Arc::clone(&cc);
            while let Some(msg) = rx.recv().await {
                let per = ((msg[0] as f32 / msg[1] as f32) * 100.0) as u64;
                if !EFileSchema::is_no_progress(arc_info.schema) {
                    log_pb.set_message(format!("{:?}%({}KB/{}KB)", per, msg[0], msg[1]));
                }
                log_pb.set_position(per);
            }
        });
        arc_pb
    }
}
