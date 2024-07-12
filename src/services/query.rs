use crate::GLOBAL_MAP;
use ntex::web;
//use std::time::{Instant, Duration};

pub async fn index(path: web::types::Path<String>) -> Result<String, web::Error> {
    let map = GLOBAL_MAP.read().unwrap();
    //let start = Instant::now();
    let key: &String = &path.into_inner();
    if let Some(value) = map.get(key) {
        return Ok(value.to_string());
    }
    Err(web::error::ErrorBadRequest("requested key not found").into())
    //let duration = start.elapsed();

    //println!("Time elapsed in query() is: {:?}", duration);
}
