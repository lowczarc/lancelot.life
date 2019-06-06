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
    views::{render_view, HtmlView, ViewVar},
};

const HTML_STRUCTURE: HtmlView = import_view!("views/about.html");

lazy_static! {
    pub static ref ABOUT: Route = (Regex::new(r"^/about/?$").unwrap(), about_route);
}

pub fn about_route(_req: Request, _db_pool: Arc<Pool>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(vars, title: "About - Lancelot Owczarczak");

    add_to_view!(vars, section: render_view(HTML_STRUCTURE, &vars));
    add_to_view!(vars, aside: render_view(ASIDE, &HashMap::new()));

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(STRUCT, &vars));
    Ok(res)
}
