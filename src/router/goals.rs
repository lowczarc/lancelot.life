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
    pub static ref GOALS: Route = (Regex::new(r"^/goals/?$").unwrap(), goals_route);
    pub static ref HTML_STRUCTURE: HtmlView = read_template("views/goals.html").unwrap();
}

pub fn goals_route(_req: Request, db_pool: Arc<Pool>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(vars, title: "My Goals - Lancelot Owczarczak");

    let goals = db_pool
        .prep_exec("SELECT content FROM goals", ())
        .map(|result| {
            result
                .map(std::result::Result::unwrap)
                .map(|row| {
                    let (content,): (String,) = mysql::from_row(row);
                    content.into()
                })
                .collect::<Vec<ViewVar>>()
        })
        .unwrap();

    add_to_view!(vars, goals: goals);
    add_to_view!(vars, section: render_view(&HTML_STRUCTURE, &vars));
    add_to_view!(vars, aside: render_view(&ASIDE, &HashMap::new()));

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(&STRUCT, &vars));
    Ok(res)
}
