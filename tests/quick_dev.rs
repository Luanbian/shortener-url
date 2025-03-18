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
            "original_url": "https://www.airbnb.com.br/rooms/866003933252757617?adults=2&category_tag=Tag%3A5348&enable_m3_private_room=true&location=Curitiba%2C%20Estado%20de%20Parana%20Brasil&photo_id=1624509846&search_mode=flex_destinations_search&check_in=2025-06-15&source_impression_id=p3_1738635422_P3gi4dQyiZGuWikj&previous_page_section_name=1001&federated_search_id=3c4c36e5-50d7-4559-93aa-d93629f1674a&guests=1&check_out=2025-06-18#availability-calendar",
        }),
    );
    req_create_ticket.await?.print().await?;

    //sut.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
