use gliner_rs_api::rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket().await.launch().await?;
    Ok(())
}
