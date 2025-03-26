use dotenv;
use rsvr_core::HttpRpcCore;

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::from_filename(".env")?;
    // 123
    println!(":test here");
    let rpc_core = HttpRpcCore {
        host: std::env::var("RSVR_URI").unwrap_or_default(),
        cookie: None,
    };
    let svr = api_picboo::PicbooSvr::new(rpc_core, None);
    let out = svr.find_isbn("7108018802").await?;
    let out2 = svr.isbn_add("9787572804977").await?;
    let out3 = svr.find_isbn("9787572804977").await?;
    println!("find_isbn =>{:#?}", out);
    println!("isbn_add =>{:#?}", out2);
    println!("find_isbn =>{:#?}", out3);
    Ok(())
}
