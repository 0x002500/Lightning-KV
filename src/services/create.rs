use actix_web::{web, HttpResponse, Responder};
use std::{fs::File, io::Write};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateData {
    key: String,
    value: String,
}

async fn write(key: String, value: &[u8]) -> std::io::Result<()> {
    let mut buffer: File = File::create(key)?;
    buffer.write_all(value)?;
    buffer.flush()?;

    Ok(())
}

pub async fn create_interface(data: web::Json<CreateData>) -> impl Responder {
    let _ = write(data.key.clone(), data.value.as_bytes());
    HttpResponse::Ok().body("Write Success!")
}
