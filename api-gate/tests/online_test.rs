use dotenv;

#[tokio::test]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename(".env")?;
    let svr = api_gate::GateSvr::new(std::env::var("RSVR_URI").unwrap_or_default(), None);

    // check update
    let chk_v = svr.get_checkupdate("picboo", 1).await;
    println!("api-checkupdate  resp: {:#?}:", chk_v);

    // get event
    let r = svr.get_appevent("picboo", "checkupdate").await;
    println!("api-appevent  resp: {:#?}:", r);
    Ok(())
}
