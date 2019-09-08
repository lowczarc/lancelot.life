use std::io::{BufRead, Read};

use std::collections::HashMap;
use std::str::FromStr;

use crate::response::HttpStatus;

const MAX_BODY_LENGTH: u64 = 500_000;

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: HttpMethod,
    pub location: String,
    pub query: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn read_request<R: BufRead + Read>(reader: &mut R) -> Result<Request, HttpStatus> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    fn get_tests_requests(request: &str) -> Result<BufReader<File>, std::io::Error> {
        let f = File::open(format!("./tests/requests/{}", request))?;
        Ok(BufReader::new(f))
    }

    #[test]
    fn basic_get() {
        let request = Request::read_request(&mut get_tests_requests("basic_get").unwrap()).unwrap();

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.location, "/".to_string());
        assert_eq!(request.query, String::new());
        assert_eq!(request.version, "HTTP/1.1".to_string());
        assert_eq!(request.body.len(), 0);
        assert_eq!(request.headers.len(), 3);
        assert_eq!(request.headers.get("Accept"), Some(&"*/*".to_string()));
        assert_eq!(
            request.headers.get("User-Agent"),
            Some(&"unit-test".to_string())
        );
        assert_eq!(
            request.headers.get("Host"),
            Some(&"lancelot.life".to_string())
        );
    }

    #[test]
    fn post_with_body() {
        let request =
            Request::read_request(&mut get_tests_requests("post_with_body").unwrap()).unwrap();

        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(
            request.headers.get("Content-Length"),
            Some(&"16".to_string())
        );
        assert_eq!(request.body.len(), 16);
        assert_eq!(request.body, "This is the body".to_string().into_bytes());
    }

    #[test]
    fn bad_header() {
        let request = Request::read_request(&mut get_tests_requests("bad_header").unwrap());

        assert_eq!(request, Err(HttpStatus::BadRequest));
    }

    #[test]
    fn empty_request() {
        let request = Request::read_request(&mut get_tests_requests("empty_request").unwrap());

        assert_eq!(request, Err(HttpStatus::BadRequest));
    }

    #[test]
    fn method_not_supported() {
        let request =
            Request::read_request(&mut get_tests_requests("method_not_supported").unwrap());

        assert_eq!(request, Err(HttpStatus::NotImplemented));
    }
}
