use lazy_static::lazy_static;
use crate::{
  router::{Regex, Route},
  response::Response,
  request::Request
};

lazy_static! {
    pub static ref ARTICLES: Route = (Regex::new(r"/blabla/.*").unwrap(), super_route);
}

pub fn super_route(_req: Request) -> Response {
  let mut res = Response::new();

  res.header("Content-Type".into(), "text/plain".into());
  res.body(":)".into());
  res
}
