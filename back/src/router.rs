use crate::{
  response::Response,
  response::HttpStatus,
  request::Request
};

pub fn router(req: Request) -> Response {
  let mut res = Response::new();

  match req.location {
    _ => {
      res.status(HttpStatus::NotFound);
      res.header("Content-type".to_string(), "text/html".to_string());
      res.body(format!("<!DOCTYPE html><html><body><h1>Not Found</h1><p>The requested URL {} was not found on this server.</p></body></html>", req.location));
    }
  }
  res
}