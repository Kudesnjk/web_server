mod server;
mod config;

use std::{env, net::TcpListener};
use crate::server::thread_pool::ThreadPool;
use crate::server::http_handler;
use crate::config::config::Config;
use std::sync::Arc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run $host $port!");
        return
    }
    
    let host = &args[1];
    let port = &args[2];

    let config = match Config::new() {
        Some(c) => c,
        None => {
            println!("Config file is incorrect!");
            return;
        }
    };

    let listener = match TcpListener::bind(format!("{}:{}", host, port)) {
        Ok(t) => {
            println!("Server listening {}:{}", host, port);
            t
        },
        Err(e) => {
            println!("Error occurred while binding tcp listener. \nError: {}", e.to_string());
            return;
        },
    };

    let pool = ThreadPool::new(config.thread_limit);
    println!("Server is running with {} threads", config.thread_limit);

    let guarded_root = Arc::new(config.document_root);

    for conn in listener.incoming() {
        let conn = match conn {
            Ok(t) => t,
            Err(e) => {
                println!("Connection lost. \nError: {}", e.to_string());
                continue;
            }
        };

        let clone = guarded_root.clone();

        pool.add_to_queue(|| {
            http_handler::handle_request(conn, clone)
        });
    }
}
