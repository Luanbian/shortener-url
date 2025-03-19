use anyhow::Result;
use serde_json::json;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct ShortenerResponse {
    short_url: String,
}

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let sut = httpc_test::new_client("http://localhost:8080")?;

    let req_login = sut.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    );
    req_login.await?.print().await?;

    let req_create_ticket = sut.do_post(
        "/api/shortener",
        json!({
            "user_id": Uuid::new_v4(),
            "original_url": "https://play.rust-lang.org/?version=stable&mode=debug&edition=2024",
        }),
    );
    let response = req_create_ticket.await?;
    response.print().await?;

    let shortener_data = response.json_body_as::<ShortenerResponse>();
    let short_url = match shortener_data {
        Ok(data) => data.short_url.split('/').last().unwrap().to_string(),
        Err(e) => {
            println!("Error: {:?}", e);
            String::from("Error")
        }
    };

    let url = format!("/{}", short_url);

    let req_redirect = sut.do_get(&url);
    req_redirect.await?.print().await?;

    Ok(())
}
