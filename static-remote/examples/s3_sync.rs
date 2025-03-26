use static_remote::{RFileSyncer, RemoteFileInfo, S3RegionSetting};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut d = RFileSyncer::default();
    dotenv::from_filename(".env.cloudfare").expect(".env init failed!");
    println!(
        r"required envs: 
        S3_BUCKET:{:?}
        S3_ACCESS_KEY:{:?}
        S3_SECRET_KEY :{:?}
        S3_ENDPOINT:{:?}
        S3_REGION: {:?}",
        env::var("S3_BUCKET").unwrap(),
        env::var("S3_ACCESS_KEY").unwrap(),
        env::var("S3_SECRET_KEY").unwrap(),
        env::var("S3_ENDPOINT").unwrap(),
        env::var("S3_REGION").unwrap()
    );
    d.set_s3_config(&S3RegionSetting {
        access_key: env::var("S3_ACCESS_KEY").unwrap(),
        access_sec: env::var("S3_SECRET_KEY").unwrap(),
        end_point: env::var("S3_ENDPOINT").unwrap(),
        bucket: env::var("S3_BUCKET").unwrap(),
        region: Some(env::var("S3_REGION").unwrap_or_default()),
        path: Some("".to_owned()),
    });
    let binding1 = RemoteFileInfo {
        link: "/Users/admin/data0/private_work/dpm/crates/static-remote/13.png".to_owned(),
        write_path: "out.env.json".to_owned(),
        schema: "s3".into(),
    };

    let binding2 = RemoteFileInfo {
        link: "/Users/admin/data0/private_work/dpm/crates/static-remote/13.png".to_owned(),
        write_path: "14.png".to_owned(),
        schema: "s3".into(),
    };
    d.append_up(binding1.clone())?;
    d.append_up(binding2.clone())?;
    d.exec_upload();
    Ok(())
}
