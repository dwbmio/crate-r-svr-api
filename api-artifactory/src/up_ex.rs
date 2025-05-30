use std::path::PathBuf;

use rsvr_core::error;
use rsvr_core::error::ApiReqError;
use rsvr_core::reqwest;

pub async fn post_upload_presigned_url(pb: &PathBuf, presigned_url: &str) -> Result<(), error::ApiReqError> {
    let client = reqwest::Client::new();
    let file = std::fs::read(pb).map_err(|e|{
        ApiReqError::CustomError(format!("read file {} failed", pb.display()) )
    })?;
    let upload_resp = client
        .put(presigned_url)
        .body(file)
        .send()
        .await?;
    Ok(())
}
