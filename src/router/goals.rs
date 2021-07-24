use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;

use sqlx::{self, Pool, Postgres};

use futures::executor::block_on;

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

struct Goal {
    content: String,
}

lazy_static! {
    pub static ref GOALS: Route<Pool<Postgres>> = (Regex::new(r"^/goals/?$").unwrap(), goals_route);
    pub static ref HTML_STRUCTURE: HtmlView = read_template("views/goals.html").unwrap();
}

pub fn goals_route(_req: &Request, db_pool: Arc<Pool<Postgres>>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(vars, title: "My Goals - Lancelot Owczarczak");

    let goals = block_on(sqlx::query_as!(Goal, "SELECT content FROM goals").fetch_all(&*db_pool))
        .unwrap()
        .into_iter()
        .map(|elem| elem.content.into())
        .collect::<Vec<ViewVar>>();

    add_to_view!(vars, goals: goals);
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
