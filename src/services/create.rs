use actix_web::{web, HttpResponse, Responder};
use async_fs::File;
use futures_lite::io::AsyncWriteExt;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateData {
    key: String,
    value: String,
}

async fn write(key: String, value: &[u8]) -> std::io::Result<()> {
    let mut file: File = File::create(key).await?;

    file.write_all(value).await?;
    file.flush().await?;

    Ok(())
}

pub async fn create_interface(data: web::Json<CreateData>) -> impl Responder {
    write(data.key.clone(), data.value.as_bytes());
    HttpResponse::Ok().body("Write Success!")
}
