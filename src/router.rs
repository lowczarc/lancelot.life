use lazy_static::lazy_static;
use mysql::Pool;
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub use regex::Regex;

use crate::{request::Request, response::HttpStatus, response::Response};

mod about;
mod articles;
pub mod common_views;
mod goals;
mod projects;

pub type RouteFn = fn(Request, Arc<Pool>) -> Result<Response, HttpStatus>;
pub type Route = (Regex, RouteFn);

lazy_static! {
  static ref ROUTES: Vec<&'static Route> = {
    let mut routes: Vec<&Route> = Vec::new();
    routes.push(&articles::ARTICLES);
    routes.push(&projects::PROJECTS);
    routes.push(&goals::GOALS);
    routes.push(&about::ABOUT);
    // Add here all special routes
    routes
  };
}

pub fn router(req: Request, db_pool: Arc<Pool>) -> Response {
    // Determine if it's a special route or static route
    if let Some(key) = ROUTES
        .iter()
        .position(|(regex, _route)| regex.is_match(&req.location))
    {
        match ROUTES[key].1(req, db_pool) {
            Ok(response) => response,
            Err(status) => common_views::default_http_status(status),
        }
    } else {
        router_static("static", req)
    }
}

fn router_static(static_dir: &str, req: Request) -> Response {
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
                    res.header("Content-type".to_string(), content_type.to_string());
                }

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
