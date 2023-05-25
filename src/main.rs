#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _app = gwahaedir::rocket().launch().await?;
    Ok(())
}