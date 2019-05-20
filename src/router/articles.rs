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

    let title1 = ViewVar::Simple("<h1>Super titre</h1>".into());
    let title2 = ViewVar::Simple("<h2>Deuxieme titre</h2>".into());
    let title3 = ViewVar::Simple("<h3>Troisième titre</h3>".into());
    let title4 = ViewVar::Simple("<h4>Quatrième titre</h4>".into());
    let titres = ViewVar::Array(vec![title1, title2, title3, title4]);
    vars.insert("titres".into(), &titres);

    let query = ViewVar::Simple(req.query);
    vars.insert("query".into(), &query);

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(HTML_STRUCTURE, vars));
    res
}
