use actix_web::{web, App, HttpServer};
use services::create::create_interface;
use services::read::read_interface;
use services::update::update_interface;
use services::delete::delete_interface;

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/create", web::post().to(create_interface))
            .route("/read", web::get().to(read_interface))
            .route("/update", web::post().to(update_interface))
            .route("/delete", web::get().to(delete_interface))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
