use actix_web::{web, App, HttpServer};
use services::create::create_interface;
use services::query::query_interface;

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/create", web::post().to(create_interface))
            .route("/query", web::get().to(query_interface))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
