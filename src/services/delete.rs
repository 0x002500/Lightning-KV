use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct DeleteData {
    key: String
}

pub async fn delete_interface(data: web::Json<DeleteData>) -> impl Responder {
    let path: PathBuf = PathBuf::from(data.key.clone());
    fs::remove_file(path.clone()).expect("File not found!");
    HttpResponse::Ok().body(format!("Delete Success! {}", path.display()))
}
