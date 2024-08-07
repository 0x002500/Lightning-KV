use dashmap::DashMap;
use lazy_static::lazy_static;
use ntex::web;
use std::env;
use std::io::Result;
use std::sync::{Arc, RwLock};

mod services;

lazy_static! {
    pub static ref GLOBAL_MAP: Arc<RwLock<DashMap<String, String>>> =
        Arc::new(RwLock::new(DashMap::new()));
}

#[ntex::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut bind_ip = "127.0.0.1".to_string();
    let mut bind_port = 8080;
    let mut bind_ip_is_present = false;
    let mut bind_port_is_present = false;

    // Parse command line arguments
    let mut i = 0;
    while i < args.len() {
        if args[i] == "-ipaddr" && i + 1 < args.len() {
            bind_ip = args[i + 1].clone();
            bind_ip_is_present = true;
            i += 1;
        } else if args[i] == "-port" && i + 1 < args.len() {
            bind_port = args[i + 1].parse().unwrap_or(8080);
            bind_port_is_present = true;
            i += 1;
        } else if i + 1 < args.len() {
            if let Some((host, port)) = parse_host_port(&args[i]) {
                if !bind_ip_is_present && !bind_port_is_present {
                    bind_ip = host.to_string();
                    bind_port = port;
                }
            } else {
                println!("-ipaddr & -port args have higher priority than a full host name. Host name will NOT be used.");
            }
        }
        i += 1;
    }

    println!("Binding to {}:{}", bind_ip, bind_port);

    // Start HTTP server
    web::HttpServer::new(|| {
        web::App::new()
            .route("/", web::get().to(services::index::index))
            .route("/insert", web::post().to(services::insert::index))
            .route("/query/{key}", web::get().to(services::query::index))
            .route("/status", web::get().to(services::status::index))
    })
    .bind((bind_ip.as_str(), bind_port))?
    .workers(4)
    .run()
    .await
}

fn parse_host_port(host_port: &str) -> Option<(&str, u16)> {
    if let Some(idx) = host_port.rfind(':') {
        let host = &host_port[..idx];
        if let Ok(port) = host_port[idx + 1..].parse::<u16>() {
            return Some((host, port));
        }
    }
    None
}
