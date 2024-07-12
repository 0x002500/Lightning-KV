use std::mem::size_of_val;
use ntex::web;
use crate::GLOBAL_MAP;

pub async fn index(_path: web::types::Path<()>) -> Result<String, web::Error> {
    let map = GLOBAL_MAP.read().unwrap();
    let size = map.len();
    let capacity = map.capacity();

    let keys_size: usize = map.keys().map(|k| size_of_val(k) + k.capacity()).sum();
    let values_size: usize = map.values().map(|v| size_of_val(v) + v.capacity()).sum();
    let hashmap_overhead = size_of_val(&*map);

    let memory_usage = keys_size + values_size + hashmap_overhead;

    let result = format!("MAP size: {}, MAP capacity: {}, Memory Usage: {} (bytes)", size, capacity, memory_usage);
    Ok(result)
}
