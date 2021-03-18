#[derive(Debug)]
pub struct Request <'a>  {
    method: &'a str,
    path: &'a str,
}

impl <'a> Request <'a> {
    pub fn new(buf: &[u8]) -> Option<Request> {
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