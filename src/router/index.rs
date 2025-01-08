use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;

use sqlx::{Pool, Postgres};

use crate::{
    request::Request,
    response::{HttpStatus, Response},
    router::{
        utils::{initial_vars, render_in_common_view},
        Regex, Route,
    },
    template::{read_template, HtmlView},
    views::ViewVar,
};

lazy_static! {
    pub static ref INDEX: Route<Pool<Postgres>> = (Regex::new(r"^/*$").unwrap(), index_route);
    pub static ref HTML_STRUCTURE: HtmlView = read_template("views/index.html").unwrap();
}

pub fn index_route(_req: &Request, db_pool: Arc<Pool<Postgres>>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = initial_vars(db_pool.clone());

    add_to_view!(vars, title: "LO");

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_in_common_view(&HTML_STRUCTURE, &vars));
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view() {
        lazy_static::initialize(&HTML_STRUCTURE);
    }
}
