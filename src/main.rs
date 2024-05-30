use ntex::web;

mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .route("/", web::get().to(services::index::index))
            .route("/newput", web::post().to(services::newput::index))
            .route("/query/{filename}", web::get().to(services::query::index))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}
