use ntex::web;

pub async fn index() -> Result<String, web::Error> {
    Ok("Thanks for use Lightning-KV".to_string())
}
