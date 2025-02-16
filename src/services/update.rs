use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::{fs::File, io::Write};

#[derive(Deserialize)]
pub struct UpdateData {
    key: String,
    value: String,
}

async fn update(key: String, value: &[u8]) -> std::io::Result<()> {
    let mut buffer: File = File::create(key)?;
    buffer.write_all(value)?;
    buffer.flush()?;

    Ok(())
}

pub async fn update_interface(data: web::Json<UpdateData>) -> impl Responder {
    let _ = update(data.key.clone(), data.value.as_bytes());
    HttpResponse::Ok().body("Write Success!")
}
