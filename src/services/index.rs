use ntex::web;

pub async fn index() -> Result<String, web::Error> {
    Ok(format!("Thanks for use Lightning-KV"))
}
