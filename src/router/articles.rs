mod all_articles;

use std::collections::HashMap;
use std::sync::Arc;
use mysql::{self, Pool};

use lazy_static::lazy_static;

use crate::{
    request::Request,
    response::Response,
    router::{Regex, Route, common_struct::HTML_COMMON_STRUCT},
    views::{ViewVar, render_view},
};

lazy_static! {
    pub static ref ARTICLES: Route = (Regex::new(r"^/articles/(?P<article>(?P<id>[0-9]+)-(?P<name>[a-z\-]+)){0,1}$").unwrap(), article_route);
}

pub fn article_route(req: Request, db_pool: Arc<Pool>) -> Response {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();
    let params = ARTICLES.0.captures_iter(&req.location).next().unwrap();

    add_to_view!(vars, section: all_articles::render(db_pool)); 


    add_to_view!(vars, title: "Lancelot Owczarczak");

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(HTML_COMMON_STRUCT, vars));
    res
}
