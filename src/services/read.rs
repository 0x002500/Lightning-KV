use actix_web::web;
use serde::Deserialize;
use actix_files::NamedFile;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct QueryData {
    key: String
}

pub async fn read_interface(data: web::Json<QueryData>) -> actix_web::Result<NamedFile> {
    let path: PathBuf = PathBuf::from(data.key.clone());
    Ok(NamedFile::open(path)?)
}
