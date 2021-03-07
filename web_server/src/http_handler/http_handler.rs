use std::{io::{Read, Write}, net::TcpStream, str::Utf8Error};

use crate::thread_pool::ThreadPool;

pub struct HTTPHandler {
    thread_pool: ThreadPool,
}

impl HTTPHandler {
    const BAD_REQUEST: &'static[u8] = b"HTTP/1.1 400 Bad Request\r\n\r\n";
    const OK_REQUEST: &'static[u8] = b"HTTP/1.1 200 OK\r\n\r\n";

    pub fn new(thread_pool: ThreadPool) -> HTTPHandler {
        HTTPHandler {
            thread_pool,
        }
    }

    pub fn handle_connection(&self, conn: TcpStream) {
        match self.thread_pool.add_to_queue(|| {
            HTTPHandler::handle_request(conn)
        }) {
            Ok(_) => (),
            Err(e) => println!("{}", e.to_string()),
        }
    }

    fn handle_request(mut conn: TcpStream) {
        let mut buf = [0; 1024];

        let request_str = match conn.read(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        };
        
        if request_str.is_err() {
            match HTTPHandler::respond_if_incorrect(&mut conn) {
                Err(e ) => println!("{}", e.to_string()),
                _ => (),
            }
        }
        //println!("{}", std::str::from_utf8(&request_str).unwrap());
        let request_str = request_str.unwrap();
        let request = Request::new(&request_str);

        if request.is_none() {
            match HTTPHandler::respond_if_incorrect(&mut conn) {
                Err(e ) => println!("{}", e.to_string()),
                _ => (),
            }
        }

        println!("{:?}", request);
        conn.write(HTTPHandler::OK_REQUEST).unwrap();
        conn.flush().unwrap()
    }

    fn respond_if_incorrect(conn: &mut TcpStream) -> std::io::Result<()> {
        conn.write(HTTPHandler::BAD_REQUEST)?;
        conn.flush()?;
        Ok(())
    }
}

#[derive(Debug)]
struct Request <'a>  {
    method: &'a str,
    path: &'a str,
}

impl <'a> Request <'a> {
    fn new(buf: &[u8]) -> Option<Request> {
        let request_str = match std::str::from_utf8(buf) {
            Ok(t) => t,
            Err(_) => return None,
        };

        let mut first_line = request_str.split("\r\n").next()?.split(" ");
        let method = first_line.next()?;
        let path = first_line.next()?;

        Some(Request {
            method,
            path,
        })
    }
}
