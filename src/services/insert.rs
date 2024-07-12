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
    let map: std::sync::RwLockWriteGuard<dashmap::DashMap<String, String>> =
        GLOBAL_MAP.write().unwrap();
    let key: String = data.key.to_string();
    let value: String = data.value.to_string();

    map.insert(key, value);

    Ok(format!("Already write to {}!", data.key))
}
