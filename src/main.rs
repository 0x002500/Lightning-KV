use actix_web::{web, App, HttpServer};
use services::create::create_interface;

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/create", web::post().to(create_interface)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
