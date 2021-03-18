use std::{net::TcpStream, io::{Read, Write}, path};
use crate::server::request::Request;
use std::sync::Mutex;
use crate::server::file_manager::{get_file, get_mime_type};

const BAD_REQUEST: &'static[u8] = b"HTTP/1.1 400 Bad Request\r\n";
const NOT_FOUND: &'static[u8] = b"HTTP/1.1 404 Not Found\r\n";
const OK_REQUEST: &'static[u8] = b"HTTP/1.1 200 OK\r\n";
const INTERNAL_ERROR: &'static[u8] = b"HTTP/1.1 500 Internal Server Error\r\n";

const CONNECTION: &'static str = "Connection: close";
const SERVER: &'static str = "Server: Pismenniy Daniil";
const RESPONSE_END: &'static[u8] = b"\r\n\r\n";

pub fn handle_request(mut conn: TcpStream) {
    let root_path = path::Path::new("/home/daniil/Desktop/web_server/web_server/static");

    let mut buf = [0; 1024];

    let request_str = match conn.read(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    };

    if request_str.is_err() {
        return respond(&mut conn, BAD_REQUEST).unwrap_or(());
    }

    let request_str = request_str.unwrap();
    let request = Request::new(&request_str);

    if request.is_none() {
        return respond(&mut conn, BAD_REQUEST).unwrap_or(());
    }

    let request = request.unwrap();
    let file = get_file(root_path, request.path);

    if file.is_err() {
        return respond(&mut conn, NOT_FOUND).unwrap_or(());
    }

    let mut file = file.unwrap();
    let mime_type = get_mime_type(request.path);

    if mime_type.is_none() {
        return respond(&mut conn, NOT_FOUND).unwrap_or(());
    }

    let mime_type = format!("Content-Type: {}", mime_type.unwrap());
    let content_length = format!("Content-Length: {}", file.metadata().unwrap().len());
    let date = format!("Date: {}", "");

    let headers = [mime_type, content_length, date, CONNECTION.to_string(), SERVER.to_string()];
    let join = headers.join("\r\n");

    conn.write(OK_REQUEST);
    conn.write(&join.as_bytes());
    conn.write(RESPONSE_END);

    let mut buffer = Vec::new();

    if request.method == "GET" {
        match file.read_to_end(&mut buffer) {
            Err(e) => {
                return respond(&mut conn, INTERNAL_ERROR).unwrap_or(());
            },
            _ => (),
        }
    }

    conn.write(&buffer);
    conn.flush().unwrap()
}

pub fn respond(conn: &mut TcpStream, resp: &'static [u8]) -> std::io::Result<()> {
    conn.write(resp)?;
    conn.flush()?;
    Ok(())
}
