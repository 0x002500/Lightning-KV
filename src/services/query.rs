use ntex::web;
use ntex_files::NamedFile;
use std::path::PathBuf;


pub async fn index(req: web::HttpRequest) -> Result<NamedFile, web::Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}
