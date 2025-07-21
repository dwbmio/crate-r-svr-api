use static_remote::{RFileSyncer, RemoteFileInfo, S3RegionSetting};
use std::env;

/// Test S3 configuration and sync by rfilesyncer
fn test_s3(cli: &mut RFileSyncer) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(
        std::path::Path::new(&std::env::current_dir().expect("ensure cur-dir"))
            .join(".env.cloudflare"),
    )
    .expect(".env.s3 init failed!");
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
    cli.set_s3_config(&S3RegionSetting {
        access_key: env::var("S3_ACCESS_KEY").unwrap(),
        access_sec: env::var("S3_SECRET_KEY").unwrap(),
        end_point: env::var("S3_ENDPOINT").unwrap(),
        bucket: env::var("S3_BUCKET").unwrap(),
        region: Some(env::var("S3_REGION").unwrap_or_default()),
        path: Some("".to_owned()),
    });
    Ok(())
}

/// Test Nexus configuration and sync by rfilesyncer
fn test_nexus(cli: &mut RFileSyncer) -> Result<(), Box<dyn std::error::Error>> {
    let ret = dotenv::from_filename(".env.nexus");
    println!("nexus path: {:?}", ret);
    println!(
        "required envs:\n\
        NEXUS_BASE_URL:{:?}\n\
        NEXUS_API_ENDPOINT:{:?}\n\
        NEXUS_USERNAME:{:?}\n\
        NEXUS_PASSWORD:{:?}",
        env::var("NEXUS_BASE_URL").unwrap(),
        env::var("NEXUS_API_ENDPOINT").unwrap(),
        env::var("NEXUS_USERNAME").unwrap(),
        env::var("NEXUS_PASSWORD").unwrap()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut d = RFileSyncer::default();
    // test_s3(&mut d)?;
    test_nexus(&mut d)?;

    // let file_1 = RemoteFileInfo {
    //     link: "/Users/admin/data0/private_work/dpm/crates/static-remote/13.png".to_owned(),
    //     write_path: "out.env.json".to_owned(),
    //     schema: "s3".into(),
    // };
    // let file_2 = RemoteFileInfo {
    //     link: "/Users/admin/data0/private_work/dpm/crates/static-remote/13.png".to_owned(),
    //     write_path: "14.png".to_owned(),
    //     schema: "s3".into(),
    // };
    let file_3 = RemoteFileInfo {
        link: "/Users/admin/data0/private_work/dpm/crates/static-remote/13.png".to_owned(),
        write_path: "15.png".to_owned(),
        schema: "nexus".into(),
    };
    // d.append_up(file_1.clone())?;
    // d.append_up(file_2.clone())?;
    d.append_up(file_3.clone())?;
    d.exec_upload();
    Ok(())
}
