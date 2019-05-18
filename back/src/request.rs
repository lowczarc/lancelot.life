use std::io::{Read, BufReader, BufRead};
use std::net::TcpStream;

use std::collections::HashMap;
use std::str::FromStr;

use crate::response::HttpStatus;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub location: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn read_request(reader: &mut BufReader<TcpStream>) -> Result<Request, HttpStatus>  {
        if let Some(Ok(request)) = reader.lines().next() {
            let http_request_vec : Vec<&str> = request.split(' ').collect();

            let mut headers = HashMap::new();
            for line in reader.by_ref().lines().skip(1) {
                let line_str = if let Ok(line) = line {
                    line
                } else {
                    return Err(HttpStatus::InternalServerError);
                };

                if line_str == "" {
                    break;
                }

                let line: Vec<&str> = line_str.split(':').collect();
                if line.len() >= 2 {
                    headers.insert(line[0].trim().into(), line.into_iter().skip(1).collect::<String>().trim().into());
                } else {
                    return Err(HttpStatus::BadRequest);
                }
            }
            let http_request = Request {
                method: if let Ok(method) = HttpMethod::from_str(http_request_vec[0]) {
                    method
                } else {
                    return Err(HttpStatus::NotImplemented);
                },
                location: http_request_vec[1].into(),
                version: http_request_vec[2].into(),
                headers,
            };
            Ok(http_request)
        } else {
            Err(HttpStatus::BadRequest)
        }
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    HEAD,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "HEAD" => Ok(HttpMethod::HEAD),
            _ => Err(())
        }
    }
}
