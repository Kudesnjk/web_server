use std::{net::TcpStream, io::{Read, Write}, path};
use crate::server::request::Request;
use crate::server::file_manager::{get_file, get_mime_type};
use chrono::Datelike;

const BAD_REQUEST: &'static[u8] = b"HTTP/1.1 400 BAD REQUEST";
const NOT_FOUND: &'static[u8] = b"HTTP/1.1 404 NOT FOUND";
const OK_REQUEST: &'static[u8] = b"HTTP/1.1 200 OK";
const INTERNAL_ERROR: &'static[u8] = b"HTTP/1.1 500 INTERNAL SERVER ERROR";
const METHOD_NOT_ALLOWED: &'static[u8] = b"HTTP/1.1 405 METHOD NOT ALLOWED";

const CONNECTION: &'static str = "Connection: Closed";
const SERVER: &'static str = "Server: Pismenniy Daniil";
const SEPARATOR: &'static[u8] = b"\r\n";

pub fn handle_request(mut conn: TcpStream) {
    let root_path = path::Path::new("/home/daniil/Desktop/web_server/static");

    let mut buf = [0; 1024];

    let request_str = match conn.read(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    };

    if request_str.is_err() {
        return respond_err(&mut conn, BAD_REQUEST).unwrap_or_else(|e| {
            println!("{}", e)
        });
    }

    let request_str = request_str.unwrap();
    let request = Request::new(&request_str);

    if request.is_none() {
        return respond_err(&mut conn, BAD_REQUEST).unwrap_or_else(|e| {
            println!("{}", e)
        });
    }

    let request = request.unwrap();

    if request.method != "GET" && request.method != "HEAD" {
        return respond_err(&mut conn, METHOD_NOT_ALLOWED).unwrap_or_else(|e| {
            println!("{}", e)
        })
    }

    let file = get_file(root_path, request.path);

    if file.is_err() {
        return respond_err(&mut conn, NOT_FOUND).unwrap_or_else(|e| {
            println!("{}", e)
        });
    }

    let mut file = file.unwrap();
    let mime_type = get_mime_type(request.path);

    if mime_type.is_none() {
        return respond_err(&mut conn, NOT_FOUND).unwrap_or_else(|e| {
            println!("{}", e)
        });
    }

    let mime_type = format!("Content-Type: {}", mime_type.unwrap());
    let content_length = format!("Content-Length: {}", file.metadata().unwrap().len());

    let headers = [mime_type, content_length];
    let join = headers.join("\r\n");

    conn.write(OK_REQUEST);
    conn.write(SEPARATOR);
    conn.write(&join.as_bytes());
    conn.write(SEPARATOR);
    add_required_headers(&mut conn);
    conn.write(SEPARATOR);

    if request.method == "GET" {
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Err(e) => {
                return respond_err(&mut conn, INTERNAL_ERROR).unwrap_or(());
            },
            _ => (),
        }

        conn.write(&buffer);
    }

    conn.flush().unwrap()
}

fn respond_err(conn: &mut TcpStream, resp: &'static [u8]) -> std::io::Result<()> {
    conn.write(resp)?;
    conn.write(SEPARATOR)?;
    add_required_headers(conn)?;
    conn.write(SEPARATOR)?;
    conn.flush()?;
    Ok(())
}

fn add_required_headers(conn: &mut TcpStream) -> std::io::Result<()> {
    conn.write(get_date().as_bytes())?;
    conn.write(SEPARATOR)?;
    conn.write(CONNECTION.as_bytes())?;
    conn.write(SEPARATOR)?;
    conn.write(SERVER.as_bytes())?;
    conn.write(SEPARATOR)?;
    Ok(())
}

fn write_with_sep(conn: &mut TcpStream, data: &[u8]) -> std::io::Result<()> {
    conn.write(data)?;
    conn.write(SEPARATOR)?;
    Ok(())
}

fn get_date() -> String {
    let now = chrono::Utc::now();
    format!("Date: {}", now.to_rfc2822().replace("+0000", "GMT"))
}
