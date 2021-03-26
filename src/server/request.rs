use std::ops::Index;

#[derive(Debug)]
pub struct Request <'a>  {
    pub method: &'a str,
    pub path: String,
}

impl <'a> Request <'a> {
    pub fn new(buf: &[u8]) -> Option<Request> {
        let request_str = std::str::from_utf8(buf).ok()?;

        let mut first_line = request_str.split("\r\n").next()?.split(" ");
        let method = first_line.next()?;
        let mut buf_path = first_line.next()?;

        let mut path = String::from(buf_path);
        path = percent_encoding::percent_decode_str(path.as_str()).decode_utf8_lossy().to_string();

        if path.contains("?") {
            path.truncate(path.find("?")?);
        }

        Some(Request {
            method,
            path,
        })
    }
}