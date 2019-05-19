use std::fs;
use std::path::Path;
use lazy_static::lazy_static;

pub use regex::Regex;

use crate::{
  response::Response,
  response::HttpStatus,
  request::Request
};

mod articles;

pub type RouteFn = fn(Request) -> Response;
pub type Route = (Regex, RouteFn);

lazy_static! {
  static ref ROUTES: Vec<&'static Route> = {
    let mut routes: Vec<&Route> = Vec::new();
    routes.push(&articles::ARTICLES);
    routes
  };
}

pub fn router(req: Request) -> Response {
  if let Some(key) = ROUTES.iter().position(|(regex, _route)| { regex.is_match(&req.location) }) {
    ROUTES[key].1(req)
  } else {
    router_static("static", req)
  }
}

fn router_static(static_dir: &str, req: Request) -> Response {
  let mut res = Response::new();
  let mut current_path = Path::new(&format!("./{}/{}", static_dir, req.location)).canonicalize();

  if current_path.is_ok() {
    let path = current_path.unwrap();
    if path.is_dir() {
      current_path = Path::new(&format!("{}/{}", path.to_str().unwrap(), "index.html")).canonicalize();
    } else {
      current_path = Ok(path);
    }
  }

  match current_path {
    Ok(path) => {
      if path.starts_with(Path::new("./static").canonicalize().unwrap()) {
        let content_type = if let Some(extension) = path.extension() {
          match extension.to_str().unwrap() {
            "html" | "htm" => "text/html; charset=utf-8",
            "css" => "text/css",
            "jpg" | "jpeg "=> "image/jpeg",
            "png" => "image/png",
            _ => ""
          }
        } else {
          ""
        };
        if content_type != "" {
          res.header("Content-type".to_string(), content_type.to_string());
        }
        res.raw_body(fs::read(path).expect("Failed to read string"));
      } else {
        res.status(HttpStatus::Forbidden);
        res.header("Content-type".to_string(), "text/html".to_string());
        res.body(format!("<!DOCTYPE html><html><body><h1>Forbidden</h1><p>You don't have permission to access {} on this server.</p></body></html>", req.location));
      }
    },
    Err(_) => {
      res.status(HttpStatus::NotFound);
      res.header("Content-type".to_string(), "text/html".to_string());
      res.body(format!("<!DOCTYPE html><html><body><h1>Not Found</h1><p>The requested URL {} was not found on this server.</p></body></html>", req.location));
    },
  }

  res
}
