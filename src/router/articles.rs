use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    request::Request,
    response::Response,
    router::{Regex, Route},
    views::{HtmlView, ViewVar, render_view},
};

const HTML_STRUCTURE: HtmlView = import_view!("views/article.html");

lazy_static! {
    pub static ref ARTICLES: Route = (Regex::new(r"/blabla/.*").unwrap(), super_route);
}

pub fn super_route(req: Request) -> Response {
    let mut res = Response::new();
    let mut vars: HashMap<String, &ViewVar> = HashMap::new();
    
    let title = ViewVar::Simple("Super titre".into());
    vars.insert("title".into(), &title);

    let name = ViewVar::Simple(req.query);
    vars.insert("query".into(), &name);

    res.header("Content-Type".into(), "text/html".into());
    res.body(render_view(HTML_STRUCTURE, vars));
    res
}
