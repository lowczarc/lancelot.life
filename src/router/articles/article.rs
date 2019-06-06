use std::collections::HashMap;
use std::sync::Arc;

use mysql::Pool;

use regex;

use crate::{
    response::HttpStatus,
    router::common_views::{ASIDE, STRUCT},
    views::{render_view, HtmlView, ViewVar},
};

const HTML_STRUCTURE: HtmlView = import_view!("views/article.html");

pub fn render(db_pool: Arc<Pool>, params: regex::Captures) -> Result<String, HttpStatus> {
    let mut vars: HashMap<String, ViewVar> = HashMap::new();
    let id: i32 = params.name("id").unwrap().as_str().parse().unwrap();
    let name = params.name("name").unwrap().as_str().to_string();

    for result in db_pool.prep_exec("SELECT articles.id, articles.titre, articles.content, group_concat( tags.tag SEPARATOR ', ' ) AS 'tags' FROM articles LEFT JOIN tags on tags.article_id = articles.id WHERE articles.id = :id GROUP BY articles.id", (id,)) {
        let row = result.map(|x| x.unwrap()).next();
        let (_id, titre, content, tags): (i32, String, String, Option<String>) = if let Some(row) = row {
            mysql::from_row(row)
        } else {
            return Err(HttpStatus::NotFound);
        };
        let url_title = titre.chars().filter(|elem| elem.is_alphanumeric() || *elem == ' ').collect::<String>().replace(" ", "-").to_lowercase();
        if name != url_title {
            return Err(HttpStatus::NotFound);
        }

        if let Some(tags) = tags {
            add_to_view!(vars, tags: tags.split(", ").map(|elem| { ViewVar::from(elem) }).collect::<Vec<ViewVar>>());
        }
        add_to_view!(vars, content: content.replace("\n", "<br/>"));
        add_to_view!(vars, title: titre);
    }

    add_to_view!(vars, section: render_view(HTML_STRUCTURE, &vars));
    add_to_view!(vars, aside: render_view(ASIDE, &HashMap::new()));

    add_to_view!(vars, title: "Article lol - Lancelot Owczarczak");

    Ok(render_view(STRUCT, &vars))
}
