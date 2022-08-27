use std::fs;
use std::path::Path;
use std::sync::Arc;

pub use regex::Regex;

use sqlx::{Pool, Postgres};

use crate::{request::Request, response::HttpStatus, response::Response};

pub mod common_views;
mod goals;
mod index;
mod influences;
pub mod utils;

pub type RouteFn<T> = fn(&Request, Arc<T>) -> Result<Response, HttpStatus>;
pub type Route<T> = (Regex, RouteFn<T>);

pub fn create_routes() -> Vec<&'static Route<Pool<Postgres>>> {
    let mut routes: Vec<&Route<Pool<Postgres>>> = Vec::new();
    routes.push(&goals::GOALS);
    routes.push(&index::INDEX);
    routes.push(&influences::INFLUENCES);
    // Add here all special routes
    routes
}

pub fn router(req: Request, db_pool: Arc<Pool<Postgres>>) -> Response {
    let routes = create_routes();

    // Determine if it's a special route or static route
    let mut response = if let Some(key) = routes
        .iter()
        .position(|(regex, _route)| regex.is_match(&req.location))
    {
        match routes[key].1(&req, db_pool) {
            Ok(response) => response,
            Err(status) => common_views::default_http_status(status),
        }
    } else {
        router_static("static", &req)
    };

    // Generate the ETag for caching resources
    let res_etag = response.etag();

    // Check if the client has the same ETag as the server and send a 304
    if let Some(req_etag) = req.headers.get("if-none-match") {
        if req_etag.eq(&res_etag) {
            response.raw_body(Vec::new());
            response.status(HttpStatus::NotModified);
        }
    }

    response.header("ETag".to_string(), res_etag);

    response
}

fn router_static(static_dir: &str, req: &Request) -> Response {
    let mut res = Response::new();
    let mut current_path = Path::new(&format!("./{}/{}", static_dir, req.location)).canonicalize();

    if let Ok(path) = current_path {
        // if path is a dir, get the index.html
        if path.is_dir() {
            current_path =
                Path::new(&format!("{}/{}", path.to_str().unwrap(), "index.html")).canonicalize();
        } else {
            current_path = Ok(path);
        }
    }

    match current_path {
        Ok(path) => {
            // Verify Request is not "GET /../something HTTP/1.1"
            if path.starts_with(Path::new("./static").canonicalize().unwrap()) {
                let content_type = if let Some(extension) = path.extension() {
                    match extension.to_str() {
                        Some("html") | Some("htm") => "text/html; charset=utf-8",
                        Some("css") => "text/css",
                        Some("jpg") | Some("jpeg") => "image/jpeg",
                        Some("png") => "image/png",
                        _ => "",
                    }
                } else {
                    ""
                };

                if content_type != "" {
                    res.header("Content-Type".to_string(), content_type.to_string());
                }

                // Allow static content to be cached for an hour
                res.header(
                    "Cache-Control".to_string(),
                    "public, max-age=3600;".to_string(),
                );

                res.raw_body(fs::read(path).expect("Failed to read static file"));
            } else {
                res = common_views::default_http_status(HttpStatus::Forbidden);
            }
        }
        Err(_) => {
            res = common_views::default_http_status(HttpStatus::NotFound);
        }
    }

    res
}
