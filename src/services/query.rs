use crate::GLOBAL_MAP;
use ntex::web;

pub async fn index(path: web::types::Path<String>) -> Result<String, web::Error> {
    let key: &String = &path.into_inner();
    let query: Option<dashmap::mapref::one::Ref<String, String>>  = GLOBAL_MAP.get(key);
    let rep: String = query.unwrap().to_string();
    Ok(rep)
}
