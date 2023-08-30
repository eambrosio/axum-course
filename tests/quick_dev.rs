use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}
//TERMINAL 1: cargo watch --quiet --clear --watch src/ --exec run

//TERMINAL 2: cargo watch --quiet --clear --watch tests/ --exec "test --quiet quick_dev -- --nocapture"
