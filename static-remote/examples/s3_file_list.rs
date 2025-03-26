use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::{Client, Error};
use std::env;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// 列出存储桶中的所有文件
async fn list_all_files(client: &Client, bucket: &str) -> Result<Vec<(String, i64)>, Error> {
    let mut paginator = client
        .list_objects_v2()
        .bucket(bucket)
        .into_paginator()
        .send();

    let mut file_list = Vec::new();

    while let Some(page) = paginator.next().await {
        let page = page?;
        if let Some(contents) = page.contents {
            for object in contents {
                let key = object.key.unwrap_or_default(); // 文件名
                let size = object.size.unwrap_or(0); // 文件大小
                file_list.push((key, size));
            }
        }
    }

    Ok(file_list)
}

// 将文件列表写入到文件
async fn write_to_file(file_list: &[(String, i64)], file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path).await?;
    for (key, size) in file_list {
        let line = format!("File: {}, Size: {} bytes\n", key, size);
        file.write_all(line.as_bytes()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.s3")?;
    let s3_bucket = env::var("S3_BUCKET").unwrap_or("NOT_SET".to_owned());
    let s3_access_key_id = env::var("S3_ACCESS_KEY_ID").unwrap_or("NOT_SET".to_owned());
    let s3_access_key = env::var("S3_ACCESS_KEY").unwrap_or("NOT_SET".to_owned());
    let s3_endpoint = env::var("S3_ENDPOINT").unwrap_or("NOT_SET".to_owned());

    // 设置 AWS 凭据
    let credentials = Credentials::new(
        s3_access_key_id, // 替换为你的 Access Key ID
        s3_access_key,    // 替换为你的 Secret Access Key
        None,
        None,
        "custom", // 如果使用 Session Token，则填写；否则填 None
    );
    // 设置 AWS 区域
    let region = Region::new(s3_bucket.to_owned()); // 替换为你的区域
    let config = aws_config::from_env()
        .region(region)
        .credentials_provider(credentials)
        .endpoint_url(s3_endpoint)
        .load()
        .await;

    let client = Client::new(&config);
    // 输出文件路径
    let output_file_path = "s3_file_list.txt";
    // 调用函数列出所有文件
    println!("Listing files in bucket: {}", s3_bucket.to_string());
    let file_list = list_all_files(&client, &s3_bucket).await?;

    // 将结果写入文件
    write_to_file(&file_list, output_file_path).await.unwrap();
    println!("File list written to {}", output_file_path);

    Ok(())
}
