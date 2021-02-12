use anyhow::Result;
use bytes::Bytes;
use std::time::Duration;
#[tokio::main]
async fn main() -> Result<()> {
    loop {
        poll_job().await?;
        std::thread::sleep(Duration::new(3, 0))
    }
    // Ok(())
}

async fn poll_job() -> Result<()> {
    println!("poll start ...");
    let url = "https://httpbin.org/post";
    let client = reqwest::Client::new();
    let raw = [1u8; 10];
    let buf = Bytes::copy_from_slice(&raw);
    let res = client.post(url).body(buf).send().await?;
    let res_bytes = res.bytes().await?;
    dbg!(res_bytes);
    println!("poll over ...");
    Ok(())
}
