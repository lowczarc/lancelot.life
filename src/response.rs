use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    status: HttpStatus,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), "0".to_string());

        Self {
            status: HttpStatus::OK,
            version: "HTTP/1.1".to_string(),
            headers,
            body: Vec::new(),
        }
    }

    pub fn status(&mut self, status: HttpStatus) {
        self.status = status;
    }

    pub fn header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn body(&mut self, body: String) {
        self.body = body.into_bytes();
        self.header("Content-Length".to_string(), self.body.len().to_string());
    }

    pub fn raw_body(&mut self, body: Vec<u8>) {
        self.body = body;
        self.header("Content-Length".to_string(), self.body.len().to_string());
    }

    pub fn send(mut self) -> Vec<u8> {
        let mut res = format!(
            "{} {}\n{}\n\n",
            self.version,
            self.status.send(),
            self.headers
                .iter()
                .map(|(key, value)| { format!("{}: {}", key, value) })
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .into_bytes();
        res.append(&mut self.body);
        res
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpStatus {
    OK = 200,
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
            HttpStatus::BadRequest => "400 Bad Request",
            HttpStatus::Forbidden => "403 Forbidden",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::InternalServerError => "500 Internal Server Error",
            HttpStatus::NotImplemented => "501 Not Implemented",
        }
    }
}
