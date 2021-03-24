#[derive(Debug)]
pub struct Request <'a>  {
    pub method: &'a str,
    pub path: &'a str,
}

const DEFAULT_PATH: &'static str = "/index.html";

impl <'a> Request <'a> {
    pub fn new(buf: &[u8]) -> Option<Request> {
        let request_str = match std::str::from_utf8(buf) {
            Ok(t) => t,
            Err(_) => return None,
        };

        let mut first_line = request_str.split("\r\n").next()?.split(" ");
        let method = first_line.next()?;
        let mut path = first_line.next()?;

        if path == "/" {
            path = DEFAULT_PATH
        }

        Some(Request {
            method,
            path,
        })
    }
}