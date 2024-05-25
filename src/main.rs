use ntex::web;
use ntex_files::NamedFile;
use serde::Deserialize;
use std::path::PathBuf;

mod file;

#[derive(Deserialize)]
struct NewputData {
    key: String,
    value: String,
}

#[web::get("/")]
async fn index() -> Result<String, web::Error> {
    Ok(format!("Thanks for use "))
}

#[web::post("/newput")]
async fn newput(data: web::types::Json<NewputData>) -> Result<String, web::Error> {
    let key = data.key.to_string();
    let value = data.value.to_string();
    file::write(key, value)?;
    Ok(format!("Already write to {}!", data.key))
}

async fn query(req: web::HttpRequest) -> Result<NamedFile, web::Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(index)
            .service(newput)
            .route("/query/{filename}", web::get().to(query))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
