use crate::GLOBAL_MAP;
use ntex::web;
use ntex::web::Error;
use ntex::web::HttpResponse;
//use std::time::{Instant, Duration};

pub async fn index(path: web::types::Path<String>) -> Result<String, web::Error> {
    //let start = Instant::now();
    let key: &String = &path.into_inner();
    if let Some(value) = GLOBAL_MAP.get(key) {
        return Ok(value.value().clone());
    }
    Err(web::error::ErrorBadRequest("requested key not found").into())

    //let duration = start.elapsed();

    //println!("Time elapsed in query() is: {:?}", duration);
}
