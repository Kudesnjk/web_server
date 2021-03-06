use thread_pool::ThreadPool;
mod thread_pool;

use http_handler::HTTPHandler;
mod http_handler;

use std::{env, net::TcpListener};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run $host $port $threads_num!");
        return
    }
    
    let host = &args[1];
    let port = &args[2];
    let threads_num_str = &args[3];

    let threads_num: u8 = match threads_num_str.parse() {
        Ok(t) => {
            if t == 0 {
                println!("Number of threads should be from 1 to 255");
                return;
            }
            t
        }
        Err(_) => {
            println!("Number of threads should be from 1 to 255");
            return;
        } 
    };

    let listener = match TcpListener::bind(format!("{}:{}", host, port)) {
        Ok(t) => {
            println!("Server listening {}:{}", host, port);
            t
        },
        Err(e) => {
            println!("Error occured while binding tcp listener. \nError: {}", e.to_string());
            return;
        },
    };

    let pool = ThreadPool::new(threads_num);
    let handler = HTTPHandler::new(pool);

    for conn in listener.incoming() {
        let conn = match conn {
            Ok(t) => t,
            Err(e) => {
                println!("Connection lost. \nError: {}", e.to_string());
                return;
            }
        };

        handler.handle_connection(conn);
    }
}
