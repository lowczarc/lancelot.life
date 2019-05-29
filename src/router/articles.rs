mod all_articles;
mod article;

use mysql::{self, Pool};
use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;

use crate::{
    request::Request,
    response::{HttpStatus, Response},
    router::{
        common_views::{ASIDE, STRUCT},
        Regex, Route,
    },
    views::{render_view, ViewVar},
};

lazy_static! {
    pub static ref ARTICLES: Route = (
        Regex::new(r"^/articles/?(?P<article>(?P<id>[0-9]+)-(?P<name>[a-z\-]+)/?)?$").unwrap(),
        article_route
    );
}

pub fn article_route(req: Request, db_pool: Arc<Pool>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();
    let params = ARTICLES.0.captures_iter(&req.location).next().unwrap();

    if params.name("article").is_some() {
        add_to_view!(vars, section: article::render(db_pool, params)?);
    } else {
        add_to_view!(vars, section: all_articles::render(db_pool, req.query_parse().get("tag")));
        add_to_view!(vars, aside: render_view(ASIDE, HashMap::new()));
    }

    add_to_view!(vars, title: "Lancelot Owczarczak");

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(STRUCT, vars));
    Ok(res)
}
