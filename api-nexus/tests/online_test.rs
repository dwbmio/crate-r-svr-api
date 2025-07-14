use api_artifactory::model::ArtifactoryCellInfo;
use dotenv;
use serde_json::json;
use std::io::Write;

#[derive(serde::Deserialize, Debug)]
pub struct PresignedUrl {
    url: String,
}

#[tokio::test]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename(".env")?;
    let svr = api_artifactory::ArtifactoryApi::new(
        std::env::var("HOST").unwrap_or_default(),
        "".to_owned(),
    );

    // api/artifactory/runtime
    let resp_vo = svr.post_add_runtime("test_runtime", "0.0.1").await?;
    assert_ne!(
        resp_vo.code == 0,
        resp_vo.code == 1001,
        "--->> add runtime failed!"
    );
    println!("[done!]add runtime done!");

    // api/artifactory/add
    let timestamp = std::time::SystemTime::now() // 获取当前时间戳（单位为毫秒）
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let random_number = (timestamp % 1_000_000) as u32; // // 简单的伪随机数生成器 限制随机数为 6 位数字
    let pkg_name = format!("file_{}_{}.txt", timestamp, random_number);
    let new_add = ArtifactoryCellInfo {
        pid: 0,
        name: pkg_name.clone(),
        ver: "1.21.0".to_string(),
        descript: "A fundamental package for scientific computing with Python".to_string(),
        md5: Some("d41d8cd98f00b204e9800998ecf8427e".to_owned()),
        cont_size: Some(12345678),
        url: Some("s3_key_super/test".to_string()),
        min_runtime_ver: Some(1),
        max_runtime_ver: Some(2),
        tag: Some(json!(vec!["math".to_string(), "science".to_string()])),
        is_private: Some(false),
        runtime: "test_runtime".to_string(),
    };
    let resp = svr.put_artifactory_add(&new_add).await?;
    println!("[done!]add pkg: {:#?} done!", resp);
    assert_ne!(
        resp.code == 0,
        resp.code == 1001,
        "--->> add artifactory failed!"
    );
    let o: PresignedUrl = serde_json::from_value(resp.data.expect("data is none")).unwrap();
    let client = reqwest::Client::new();
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join("tests/assets/done_up.txt"); // 要上传的文件路径
    let file = std::fs::read(file_path)?;
    let upload_resp = client.put(o.url).body(file).send().await?;
    assert!(upload_resp.status().is_success(), "/add 添加模块失败");
    println!("[done!]artifactory add done!");

    // api/artifactory/find
    let resp = svr.get_find_pkg(&pkg_name).await?;
    println!("[done!]find pkg: {:#?} done!", resp);

    // api/artifactory/get_object_presigned_url
    let pkg: ArtifactoryCellInfo =
        serde_json::from_value(resp.data.expect("ENSURE")).expect("parse failed！");
    let resp = svr.get_artifactory_download_url(pkg.pid).await?;
    println!("[done!]get presigned url: {:#?} done!", resp);

    // try download
    let download_url: PresignedUrl =
        serde_json::from_value(resp.data.expect("ENSURE")).expect("parse failed！");
    let download_resp = client.get(&download_url.url).send().await?;
    assert!(download_resp.status().is_success(), "Download failed");
    let downloaded_file_path = current_dir.join("tests/assets/downloaded_file.txt");
    let mut file = std::fs::File::create(downloaded_file_path)?;
    let content = download_resp.bytes().await?;
    file.write_all(&content)?;
    println!("[done!]File downloaded and saved successfully!");
    Ok(())
}
