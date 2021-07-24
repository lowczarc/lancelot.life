use std::collections::HashMap;

use hex;
use md5::{Digest, Md5};

#[derive(Debug)]
pub struct Response {
    status: HttpStatus,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: HttpStatus::OK,
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn etag(&self) -> String {
        let mut hasher = Md5::new();

        hasher.update(&self.body);

        format!("\"{}\"", hex::encode(hasher.finalize()))
    }

    pub fn status(&mut self, status: HttpStatus) {
        self.status = status;
    }

    pub fn header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn body(&mut self, body: String) {
        self.body = body.into_bytes();
    }

    pub fn raw_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        self.header("Content-Length".to_string(), self.body.len().to_string());

        let mut res = format!("{} {}\n{}\n\n", self.version, self.status.send(), {
            let mut header_vec = self
                .headers
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value))
                .collect::<Vec<String>>();
            header_vec.sort();
            header_vec.join("\n")
        },)
        .into_bytes();
        res.append(&mut self.body);
        res
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpStatus {
    OK = 200,
    NotModified = 304,
    BadRequest = 400,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
    NotImplemented = 501,
}

impl HttpStatus {
    pub fn send(&self) -> &str {
        match self {
            HttpStatus::OK => "200 OK",
            HttpStatus::NotModified => "304 Not Modified",
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::Forbidden => "403 Forbidden",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::InternalServerError => "500 Internal Server Error",
            HttpStatus::NotImplemented => "501 Not Implemented",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_response() {
        assert_eq!(
            Response::new().into_bytes(),
            "HTTP/1.1 200 OK\nContent-Length: 0\n\n"
                .to_string()
                .into_bytes()
        );
    }

    #[test]
    fn response_with_body() {
        let mut response = Response::new();

        response.body("This is the body".to_string());

        assert_eq!(
            response.into_bytes(),
            "HTTP/1.1 200 OK\nContent-Length: 16\n\nThis is the body"
                .to_string()
                .into_bytes()
        );
    }

    #[test]
    fn response_with_header() {
        let mut response = Response::new();

        response.header("Content-Type".to_string(), "text/plain".to_string());
        assert_eq!(
            response.into_bytes(),
            "HTTP/1.1 200 OK\nContent-Length: 0\nContent-Type: text/plain\n\n"
                .to_string()
                .into_bytes()
        );
    }
}
