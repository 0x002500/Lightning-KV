use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::utils::write_file::write_data;

#[derive(Deserialize)]
pub struct CreateData {
    key: String,
    value: String,
}

pub async fn create_interface(data: web::Json<CreateData>) -> impl Responder {
    let _ = write_data(data.key.clone(), data.value.clone());
    HttpResponse::Ok().body("Write Success!")
}
