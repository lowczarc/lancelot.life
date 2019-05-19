use lazy_static::lazy_static;
use crate::{
  router::{Regex, Route},
  response::Response,
  request::Request
};

lazy_static! {
    pub static ref ARTICLES: Route = (Regex::new(r"/blable/.*").unwrap(), super_route);
}

pub fn super_route(req: Request) -> Response {
  let mut res = Response::new();

  res.body("test".to_string());
  res
}
