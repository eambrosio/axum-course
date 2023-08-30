use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Emilio").await?.print().await?;

    hc.do_get("/ README.md").await?.print().await?;

    Ok(())
}
