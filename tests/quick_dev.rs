use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client.do_get("/hello2/Emilio").await?.print().await?;

    // hc.do_get("/ README.md").await?.print().await?;

    let req_login = client.do_post(
        "/api/login",
        json!({
            "username": "test",
            "pwd": "test"
        }),
    );

    req_login.await?.print().await?;

    Ok(())
}
