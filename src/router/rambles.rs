use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;

use sqlx::{types::time::PrimitiveDateTime, Pool, Postgres};

use futures::executor::block_on;

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
    pub static ref INDEX: Route<Pool<Postgres>> =
        (Regex::new(r"^/misc/*$").unwrap(), index_route);
    pub static ref HTML_STRUCTURE: HtmlView = read_template("views/misc.html").unwrap();
}

struct Ramble {
    title: String,
    link: String,
    published_date: PrimitiveDateTime,
}

pub fn index_route(_req: &Request, db_pool: Arc<Pool<Postgres>>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = initial_vars(db_pool.clone());

    add_to_view!(vars, title: "Misc");

    let rambles = block_on(
        sqlx::query_as!(Ramble, "SELECT title, link, published_date FROM rambles ORDER BY published_date DESC")
            .fetch_all(&*db_pool),
    )
    .unwrap()
    .into_iter()
    .map(|elem| {
        let mut object: HashMap<String, ViewVar> = HashMap::new();

        add_to_view!(object, title: elem.title);
        add_to_view!(object, link: elem.link);
        let published_date: String = elem.published_date.to_string().replace(" ", "T");
        add_to_view!(object, published_date: published_date);
        object.into()
    })
    .collect::<Vec<ViewVar>>();

    add_to_view!(vars, rambles: rambles);

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
