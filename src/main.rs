use dashmap::DashMap;
use lazy_static::lazy_static;
use ntex::web;
use std::sync::Arc;

mod services;

lazy_static! {
    pub static ref GLOBAL_MAP: Arc<DashMap<String, String>> = Arc::new(DashMap::new());
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .route("/", web::get().to(services::index::index))
            .route("/newput", web::post().to(services::newput::index))
            .route("/query/{key}", web::get().to(services::query::index))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}
