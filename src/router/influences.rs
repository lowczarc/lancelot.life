use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;

use sqlx::{Pool, Postgres};

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

lazy_static! {
    pub static ref INFLUENCES: Route<Pool<Postgres>> =
        (Regex::new(r"^/influences/?$").unwrap(), influences_route);
    pub static ref HTML_STRUCTURE: HtmlView = read_template("views/influences.html").unwrap();
}

struct Influence {
    name: String,
    link: Option<String>,
}

pub fn influences_route(
    _req: &Request,
    db_pool: Arc<Pool<Postgres>>,
) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(vars, title: "My Influences - Lancelot Owczarczak");

    let influences = block_on(
        sqlx::query_as!(Influence, "SELECT name, link FROM influences").fetch_all(&*db_pool),
    )
    .unwrap()
    .into_iter()
    .map(|elem| {
        let mut object: HashMap<String, ViewVar> = HashMap::new();

        add_to_view!(object, name: elem.name);
        if let Some(link) = elem.link {
            add_to_view!(object, link: link);
        }
        object.into()
    })
    .collect::<Vec<ViewVar>>();

    add_to_view!(vars, influences: influences);
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
