use crate::{
    request::Request,
    response::Response,
    router::{Regex, Route},
    views::HtmlView,
};

use lazy_static::lazy_static;

const HTML_STRUCTURE: HtmlView = import_view!("views/article.html");

lazy_static! {
    pub static ref ARTICLES: Route = (Regex::new(r"/blabla/.*").unwrap(), super_route);
}

pub fn super_route(_req: Request) -> Response {
    let mut res = Response::new();

    res.header("Content-Type".into(), "text/plain".into());
    res.body(format!("{:#?}", HTML_STRUCTURE));
    res
}
