use crate::GLOBAL_MAP;
use ntex::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewputData {
    key: String,
    value: String,
}

pub async fn index(data: web::types::Json<NewputData>) -> Result<String, web::Error> {
    let key = data.key.to_string();
    let value = data.value.to_string();
    GLOBAL_MAP.insert(key, value);
    Ok(format!("Already write to {}!", data.key))
}
