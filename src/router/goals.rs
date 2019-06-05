// TODO: Get goals from airtable
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

const HTML_STRUCTURE: HtmlView = import_view!("views/goals.html");

lazy_static! {
    pub static ref GOALS: Route = (Regex::new(r"^/goals/?$").unwrap(), goals_route);
}

pub fn goals_route(_req: Request, _db_pool: Arc<Pool>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(vars, title: "Lancelot Owczarczak");

    add_to_view!(vars, goals:
        [
            ">= 1 Github <b>commit</b> per day",
            "Beautiful <b>F#2</b> to <b>G4</b> 🎺",
            "Improve my English",
            "Learn Lojban",
            "Write articles"
        ]
    );
    add_to_view!(vars, section: render_view(HTML_STRUCTURE, &vars));
    add_to_view!(vars, aside: render_view(ASIDE, &HashMap::new()));

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(STRUCT, &vars));
    Ok(res)
}
