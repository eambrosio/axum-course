use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello-query?name=Emilio").await?.print().await?;
    // hc.do_get("/hello-path/Emilio").await?.print().await?;

    hc.do_post("/api/login", json!({"username": "test","password": "test"}))
        .await?
        .print()
        .await?;

    Ok(())
}
