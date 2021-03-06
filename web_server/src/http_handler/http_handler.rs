use std::net::TcpStream;

use crate::thread_pool::{self, ThreadPool};

pub struct HTTPHandler {
    thread_pool: ThreadPool,
}

impl HTTPHandler {
    pub fn new(thread_pool: ThreadPool) -> HTTPHandler {
        HTTPHandler {
            thread_pool,
        }
    }

    pub fn handle_connection(&self, conn: TcpStream) {
        self.thread_pool.add_to_queue(|| {
            HTTPHandler::handle_request(conn)
        })
    }

    fn handle_request(conn: TcpStream) {
        
    }
}

