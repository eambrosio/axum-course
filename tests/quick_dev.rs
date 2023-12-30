use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8081")?;

    // hc.do_get("/hello-query?name=Emilio").await?.print().await?;
    hc.do_get("/hello-path/Emilio").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;

    // hc.do_post("/api/login", json!({"username": "error","password": "error"}))
    // hc.do_post("/api/login", json!({"username": "no-test","password": "no-test"}))
    hc.do_post("/api/login", json!({"username": "test","password": "test"}))
        .await?
        .print()
        .await?;

    let create_ticket = hc.do_post("/api/tickets", json!({"title": "AAA"}));
    create_ticket.await?.print().await?;

    // hc.do_delete("/api/tickets/1").await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
