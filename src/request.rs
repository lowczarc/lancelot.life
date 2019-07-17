use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

use std::collections::HashMap;
use std::str::FromStr;

use crate::response::HttpStatus;

const MAX_BODY_LENGTH: u64 = 500_000;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub location: String,
    pub query: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn read_request(reader: &mut BufReader<TcpStream>) -> Result<Request, HttpStatus> {
        if let Some(Ok(request)) = reader.lines().next() {
            let http_request_vec: Vec<&str> = request.split(' ').collect();

            if http_request_vec.len() != 3 {
                return Err(HttpStatus::BadRequest);
            }

            // Get method if implemented
            let method = if let Ok(method) = HttpMethod::from_str(http_request_vec[0]) {
                method
            } else {
                return Err(HttpStatus::NotImplemented);
            };

            // Location is on the form of "/location?query"
            let mut location_splitted = http_request_vec[1].split('?');
            let location = if let Some(location) = location_splitted.next() {
                location.into()
            } else {
                String::new()
            };
            let query = location_splitted.collect::<Vec<&str>>().join("?");

            let version = http_request_vec[2].into();

            // Read headers until blank line
            let mut headers: HashMap<String, String> = HashMap::new();
            for line in reader.by_ref().lines() {
                let line_str = if let Ok(line) = line {
                    line
                } else {
                    return Err(HttpStatus::InternalServerError);
                };

                if line_str == "" {
                    break;
                }

                // Headers are on the form of "Key: Value"
                let line: Vec<&str> = line_str.split(':').collect();
                if line.len() >= 2 {
                    headers.insert(
                        line[0].trim().into(),
                        line.into_iter()
                            .skip(1)
                            .collect::<Vec<&str>>()
                            .join(":")
                            .trim()
                            .into(),
                    );
                } else {
                    return Err(HttpStatus::BadRequest);
                }
            }

            // Read a body of Content-Length bytes if Content-Length is defined and method is not GET
            let body = if let Some(content_length) = headers.get("Content-Length") {
                if let Ok(length) = content_length.parse() {
                    if length > MAX_BODY_LENGTH || method == HttpMethod::GET {
                        return Err(HttpStatus::BadRequest);
                    }

                    let mut body: Vec<u8> = Vec::new();

                    if reader.take(length).read_to_end(&mut body).is_err() {
                        return Err(HttpStatus::BadRequest);
                    }
                    body
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            let http_request = Request {
                method,
                location,
                query,
                version,
                headers,
                body,
            };
            Ok(http_request)
        } else {
            Err(HttpStatus::BadRequest)
        }
    }

    pub fn query_parse(&self) -> HashMap<&str, String> {
        self.query
            .split('&')
            .map(|elem| {
                let mut query_splitted = elem.split('=');

                (
                    query_splitted.next().unwrap(),
                    query_splitted.collect::<Vec<&str>>().join("=").to_string(),
                )
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            _ => Err(()),
        }
    }
}
