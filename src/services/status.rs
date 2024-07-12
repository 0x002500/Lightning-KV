use crate::GLOBAL_MAP;
use ntex::web;
use std::mem::size_of_val;

pub async fn index(_path: web::types::Path<()>) -> Result<String, web::Error> {
    let map: std::sync::RwLockReadGuard<dashmap::DashMap<String, String>> =
        GLOBAL_MAP.read().unwrap();
    let size: usize = map.len();
    let capacity: usize = map.capacity();

    let keys_size: usize = map
        .iter()
        .map(
            |pair: dashmap::mapref::multiple::RefMulti<String, String>| {
                let (key, _value) = pair.pair();
                size_of_val(key) + key.capacity()
            },
        )
        .sum();

    let values_size: usize = map
        .iter()
        .map(
            |pair: dashmap::mapref::multiple::RefMulti<String, String>| {
                let (_key, value) = pair.pair();
                size_of_val(value) + value.capacity()
            },
        )
        .sum();

    let hashmap_overhead: usize = size_of_val(&*map);

    let memory_usage: usize = keys_size + values_size + hashmap_overhead;

    let result: String = format!(
        "MAP size: {}, MAP capacity: {}, Memory Usage: {} (bytes)",
        size, capacity, memory_usage
    );
    Ok(result)
}
