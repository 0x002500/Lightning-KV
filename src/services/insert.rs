use crate::GLOBAL_MAP;
use ntex::web;
use serde::Deserialize;
//use std::time::{Duration, Instant};
#[derive(Deserialize)]
pub struct InsertData {
    key: String,
    value: String,
}

pub async fn index(data: web::types::Json<InsertData>) -> Result<String, web::Error> {
    let mut map = GLOBAL_MAP.write().unwrap();
    let key = data.key.to_string();
    let value = data.value.to_string();
    map.insert(key, value);

    Ok(format!("Already write to {}!", data.key))
}
