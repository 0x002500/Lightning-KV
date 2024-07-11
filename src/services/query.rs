use crate::GLOBAL_MAP;
use ntex::web;
//use std::time::{Instant, Duration};

pub async fn index(path: web::types::Path<String>) -> Result<String, web::Error> {
    //let start = Instant::now();
    let key: &String = &path.into_inner();
    let query: Option<dashmap::mapref::one::Ref<String, String>>  = GLOBAL_MAP.get(key);
    let rep: String = query.unwrap().to_string();
    //let duration = start.elapsed();

    //println!("Time elapsed in query() is: {:?}", duration);
    Ok(rep)
}
