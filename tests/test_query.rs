use anyhow::Result;

#[tokio::test]
async fn test_query() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9341")?;
    hc.do_get("/").await?.print().await?;
    Ok(())
}
