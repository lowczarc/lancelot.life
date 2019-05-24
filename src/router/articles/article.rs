use std::collections::HashMap;
use std::sync::Arc;

use mysql::Pool;

use regex;

use crate::{
    response::HttpStatus,
    views::{HtmlView, ViewVar, render_view}
};

const HTML_STRUCTURE: HtmlView = import_view!("views/article.html");

pub fn render(db_pool: Arc<Pool>, params: regex::Captures) -> Result<String, HttpStatus> {
    let mut vars: HashMap<String, ViewVar> = HashMap::new();
    let id: i32 = params.name("id").unwrap().as_str().parse().unwrap();
    let name = params.name("name").unwrap().as_str().to_string();

    for result in db_pool.prep_exec("SELECT titre, content from articles WHERE id=:id", (id,)) {
        let row = result.map(|x| x.unwrap()).next();
        let (titre, content): (String, String) = if let Some(row) = row {
            mysql::from_row(row)
        } else {
            return Err(HttpStatus::NotFound);
        };
        let url_title = titre.chars().filter(|elem| elem.is_alphanumeric() || *elem == ' ').collect::<String>().replace(" ", "-").to_lowercase();
        if name != url_title {
            return Err(HttpStatus::NotFound);
        }

        add_to_view!(vars, content: content.replace("\n", "<br/>"));
        add_to_view!(vars, title: titre);
    }

    Ok(render_view(HTML_STRUCTURE, vars))
}
