mod all_articles;
mod article;

use mysql::{self, Pool};
use std::sync::Arc;

use lazy_static::lazy_static;

use crate::{
    request::Request,
    response::{HttpStatus, Response},
    router::{Regex, Route},
};

lazy_static! {
    pub static ref ARTICLES: Route = (
        Regex::new(r"^/articles/?(?P<article>(?P<id>[0-9]+)-(?P<name>[a-z\-]+)/?)?$").unwrap(),
        article_route
    );
}

pub fn article_route(req: Request, db_pool: Arc<Pool>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let params = ARTICLES.0.captures_iter(&req.location).next().unwrap();

    if params.name("article").is_some() {
        res.body(article::render(db_pool, params)?);
    } else {
        res.body(all_articles::render(db_pool, req.query_parse().get("tag")));
    }

    res.header("Content-Type".into(), "text/html; charset=utf8".into());

    Ok(res)
}
