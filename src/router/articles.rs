use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    request::Request,
    response::Response,
    router::{Regex, Route, common_struct::HTML_COMMON_STRUCT},
    views::{HtmlView, ViewVar, render_view},
};

const HTML_STRUCTURE: HtmlView = import_view!("views/article.html");

lazy_static! {
    pub static ref ARTICLES: Route = (Regex::new(r"/blabla/.*").unwrap(), article_route);
}

pub fn article_route(_req: Request) -> Response {
    let mut res = Response::new();
    let mut vars: HashMap<String, &ViewVar> = HashMap::new();

    add_to_view!(vars, articles: [
        {
            title: "What did you expect",
            content: "There isn't any article here yet"
        },
        {
            title: "Another useless article",
            content: "There isn't any article here yet"
        }
    ]);

    add_to_view!(vars, section: render_view(HTML_STRUCTURE, vars.clone())); 

    add_to_view!(vars, title: "Lancelot Owczarczak");

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(HTML_COMMON_STRUCT, vars));
    res
}
