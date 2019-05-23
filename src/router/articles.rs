use std::collections::HashMap;
use std::sync::Arc;
use mysql::{self, Pool};

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

pub fn article_route(_req: Request, db_pool: Arc<Pool>) -> Response {
    let mut res = Response::new();
    let mut vars: HashMap<String, &ViewVar> = HashMap::new();

    let articles: Vec<(ViewVar, ViewVar)> =
        db_pool.prep_exec("SELECT titre, content from articles", ())
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (titre, content): (String, Option<String>) = mysql::from_row(row);

                    (titre.into(), (if let Some(content) = content { content } else { String::new() }).into())
                }).collect()
        }).unwrap();

    let articles: Vec<HashMap<String, &ViewVar>> = articles.iter().map(|article| {
        let mut obj: HashMap<String, &ViewVar> = HashMap::new();

        obj.insert("title".into(), &article.0);
        obj.insert("content".into(), &article.1);
        obj
    }).collect();

    let articles: Vec<ViewVar> = articles.iter().map(|elem| { elem.into() }).collect();

    add_to_view!(vars, articles: articles);

    add_to_view!(vars, section: render_view(HTML_STRUCTURE, vars.clone())); 

    add_to_view!(vars, title: "Lancelot Owczarczak");

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(HTML_COMMON_STRUCT, vars));
    res
}
