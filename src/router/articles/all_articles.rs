use std::collections::HashMap;
use std::sync::Arc;
use mysql::{self, Pool};

use crate::views::{HtmlView, ViewVar, render_view};

const HTML_STRUCTURE: HtmlView = import_view!("views/all_articles.html");

const MAXIMAL_PREVIEW_LENGTH: usize = 75;

pub fn render(db_pool: Arc<Pool>) -> String {
    let mut vars: HashMap<String, &ViewVar> = HashMap::new();

    let articles: Vec<(ViewVar, ViewVar)> =
        db_pool.prep_exec("SELECT titre, content from articles", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (titre, content): (String, Option<String>) = mysql::from_row(row);
                        let content_cut = if let Some(content) = content {
                                content.replace("\n", "<br/>").chars().take(MAXIMAL_PREVIEW_LENGTH).collect::<String>()
                            } else {
                                String::new()
                            };
                        (titre.into(), format!("{}{}", content_cut, if content_cut.len() >= MAXIMAL_PREVIEW_LENGTH { "..." } else { "" }).into())
                    }).collect()
            }).unwrap();

    let articles: Vec<HashMap<String, &ViewVar>> = articles.iter().map(|article| {
        let mut obj: HashMap<String, &ViewVar> = HashMap::new();

        obj.insert("title".into(), &article.0);
        obj.insert("content".into(), &article.1);
        obj
    }).collect();

    let articles: Vec<ViewVar> = articles.iter().map(|elem| { elem.into() }).collect();

    add_to_view!(vars, articles: articles);

    render_view(HTML_STRUCTURE, vars.clone())
}