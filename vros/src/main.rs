use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    vros_steamvr::start().await.unwrap().await.unwrap().unwrap();

    Ok(())
}
