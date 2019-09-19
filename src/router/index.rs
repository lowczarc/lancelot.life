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
    template::{read_template, HtmlView},
    views::{render_view, ViewVar},
};

lazy_static! {
    pub static ref INDEX: Route = (Regex::new(r"^/*$").unwrap(), index_route);
    pub static ref HTML_STRUCTURE: HtmlView = read_template("views/index.html").unwrap();
}

pub fn index_route(_req: Request, _db_pool: Arc<Pool>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(vars, title: "Lancelot Owczarczak");

    add_to_view!(vars, section: render_view(&HTML_STRUCTURE, &vars));
    add_to_view!(vars, aside: render_view(&ASIDE, &HashMap::new()));

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(&STRUCT, &vars));
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
