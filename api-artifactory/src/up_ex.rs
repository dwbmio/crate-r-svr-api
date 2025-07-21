use std::path::PathBuf;

use static_remote::error;
use static_remote::error::ApiReqError;
use static_remote::reqwest;

pub async fn post_upload_presigned_url(
    pb: &PathBuf,
    presigned_url: &str,
) -> Result<(), error::ApiReqError> {
    let client = reqwest::Client::new();
    let file = std::fs::read(pb).map_err(|e| {
        ApiReqError::CustomError(format!(
            "read file {} failed! error:{}",
            pb.display(),
            e.to_string()
        ))
    })?;
    client.put(presigned_url).body(file).send().await?;
    Ok(())
}
