use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let sut = httpc_test::new_client("http://localhost:8080")?;
    sut.do_get("/hello2/Luan").await?.print().await?;
    Ok(())
}
