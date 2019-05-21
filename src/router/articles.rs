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
    pub static ref ARTICLES: Route = (Regex::new(r"/blabla/.*").unwrap(), super_route);
}

pub fn super_route(req: Request) -> Response {
    let mut res = Response::new();
    let mut vars: HashMap<String, &ViewVar> = HashMap::new();
    let mut article: Vec<ViewVar> = Vec::new();

    add_to_view!(vars, article: {
        content: "test",
        title: "tata",
        test: {
            tutu: "tata",
            tata: [
                "machin"
            ]
        }
    });

    //add_to_view!(vars, article, &test);

    add_to_view!(vars, section: render_view(HTML_STRUCTURE, vars.clone())); 

    add_to_view!(vars, title: "Lancelot Owczarczak");

    println!("{:?}", vars);

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(HTML_COMMON_STRUCT, vars));
    res
}
