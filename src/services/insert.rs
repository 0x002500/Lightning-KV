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
    //let start = Instant::now();
    let key = data.key.to_string();
    let value = data.value.to_string();
    GLOBAL_MAP.insert(key, value);
    //let duration = start.elapsed();

    //println!("Time elapsed in newput() is: {:?}", duration);
    Ok(format!("Already write to {}!", data.key))
}
