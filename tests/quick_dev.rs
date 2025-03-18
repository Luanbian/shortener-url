use anyhow::Result;
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let sut = httpc_test::new_client("http://localhost:8080")?;

    //sut.do_get("/hello?name=Luan").await?.print().await?;

    //sut.do_get("/hello2/Luan").await?.print().await?;

    let req_login = sut.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    );
    req_login.await?.print().await?;

    let req_create_ticket = sut.do_post(
        "/api/tickets",
        json!({
            "user_id": Uuid::new_v4(),
            "original_url": "https://www.rust-lang.org/",
        }),
    );
    req_create_ticket.await?.print().await?;

    //sut.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
