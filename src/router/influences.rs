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

const HTML_STRUCTURE: HtmlView = import_view!("views/influences.html");

lazy_static! {
    pub static ref INFLUENCES: Route = (Regex::new(r"^/influences/?$").unwrap(), influences_route);
}

pub fn influences_route(_req: Request, db_pool: Arc<Pool>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(vars, title: "My Influences - Lancelot Owczarczak");

    let influences = db_pool
        .prep_exec("SELECT name, link FROM influences", ())
        .map(|result| {
            result
                .map(std::result::Result::unwrap)
                .map(|row| {
                    let (name, link): (String, Option<String>) = mysql::from_row(row);
                    let mut object: HashMap<String, ViewVar> = HashMap::new();

                    add_to_view!(object, name: name);
                    if let Some(link) = link {
                        add_to_view!(object, link: link);
                    }
                    object.into()
                })
                .collect::<Vec<ViewVar>>()
        })
        .unwrap();

    add_to_view!(vars, influences: influences);
    add_to_view!(vars, section: render_view(HTML_STRUCTURE, &vars));
    add_to_view!(vars, aside: render_view(ASIDE, &HashMap::new()));

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(STRUCT, &vars));
    Ok(res)
}
