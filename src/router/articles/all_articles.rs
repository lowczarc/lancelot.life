use std::collections::HashMap;
use std::sync::Arc;
use mysql::{self, Pool};

use crate::views::{HtmlView, ViewVar, render_view};

const HTML_STRUCTURE: HtmlView = import_view!("views/all_articles.html");

const MAXIMAL_PREVIEW_LENGTH: usize = 75;

pub fn render(db_pool: Arc<Pool>) -> String {
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    let articles: Vec<ViewVar> =
        db_pool.prep_exec("SELECT articles.id, articles.titre, articles.content, group_concat( tags.tag SEPARATOR ', ' ) AS 'tags' FROM articles LEFT JOIN tags on tags.article_id = articles.id GROUP BY articles.id", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, titre, content, tags): (i32, String, String, Option<String>) = mysql::from_row(row);
                        let mut article: HashMap<String, ViewVar> = HashMap::new();
                        let content_cut = content.replace("\n", "<br/>").chars().take(MAXIMAL_PREVIEW_LENGTH).collect::<String>();
                        let url_title = titre.chars().filter(|elem| elem.is_alphanumeric() || *elem == ' ').collect::<String>().replace(" ", "-").to_lowercase();

                        if let Some(tags) = tags {
                            article.insert("tags".to_string(), tags.split(", ").map(|elem| { ViewVar::from(elem) }).collect::<Vec<ViewVar>>().into());
                        }
                        article.insert("link".to_string(), format!("{}-{}", id, url_title).into());
                        article.insert("title".to_string(), titre.into());
                        article.insert("content".to_string(), format!("{}{}", content_cut, if content_cut.len() >= MAXIMAL_PREVIEW_LENGTH { "..." } else { "" }).into());
                        article.into()
                    }).collect()
            }).unwrap();

    add_to_view!(vars, articles: articles);
    render_view(HTML_STRUCTURE, vars)
}