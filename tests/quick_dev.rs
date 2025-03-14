use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let sut = httpc_test::new_client("http://localhost:8080")?;

    sut.do_get("/hello?name=Luan").await?.print().await?;

    sut.do_get("/hello2/Luan").await?.print().await?;

    let req_login = sut.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
