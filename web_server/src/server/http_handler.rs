use std::{net::TcpStream, io::{Read, Write}, path};
use crate::server::request::Request;
use std::sync::Mutex;

const BAD_REQUEST: &'static[u8] = b"HTTP/1.1 400 Bad Request\r\n\r\n";
const OK_REQUEST: &'static[u8] = b"HTTP/1.1 200 OK\r\n\r\n";

pub fn handle_request(mut conn: TcpStream) {
    let root_path = path::Path::new("static/");

    let mut buf = [0; 1024];

    let request_str = match conn.read(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    };

    if request_str.is_err() {
        match respond_if_incorrect(&mut conn) {
            Err(e ) => println!("{}", e.to_string()),
            _ => (),
        }
    }

    let request_str = request_str.unwrap();
    let request = Request::new(&request_str);

    if request.is_none() {
        return match respond_if_incorrect(&mut conn) {
            Err(e) => println!("{}", e.to_string()),
            _ => (),
        }
    }

    let request = request.unwrap();
    println!("{:?}", request);

    conn.write(OK_REQUEST).unwrap();
    conn.flush().unwrap()
}

pub fn respond_if_incorrect(conn: &mut TcpStream) -> std::io::Result<()> {
    conn.write(BAD_REQUEST)?;
    conn.flush()?;
    Ok(())
}
