#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn rest_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;
    hc.do_get("/hello").await?.print().await?;
    Ok(())
}