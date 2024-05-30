use ntex::web;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize)]
pub struct NewputData {
    key: String,
    value: String,
}

fn write(key: String, value: String) -> Result<(), std::io::Error> {
    let path: &Path = Path::new(key.as_str());
    let mut file = File::create(path)?;
    file.write_all(value.as_bytes())?;

    Ok(())
}

pub async fn index(data: web::types::Json<NewputData>) -> Result<String, web::Error> {
    let key = data.key.to_string();
    let value = data.value.to_string();
    write(key, value)?;
    Ok(format!("Already write to {}!", data.key))
}
