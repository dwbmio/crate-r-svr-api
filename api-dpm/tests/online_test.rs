use dotenv;

#[tokio::test]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename(".env")?;
    let svr = api_dpm::DpmSvr::new(std::env::var("RSVR_URI").unwrap_or_default(), "".to_owned());
    // svr.post_addruntime("1", "2").await?;
    // svr.get_pkg_info("1").await?;
    let out = svr.get_runtime_list(0, 20).await?;
    println!("out is :{:#?}", out);
    Ok(())
}
