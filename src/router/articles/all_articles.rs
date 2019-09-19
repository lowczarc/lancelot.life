use mysql::{self, Pool};
use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;

use crate::{
    router::common_views::STRUCT,
    template::{read_template, HtmlView},
    views::{render_view, ViewVar},
};

const MAXIMAL_PREVIEW_LENGTH: usize = 75;

lazy_static! {
    pub static ref HTML_STRUCTURE: HtmlView = read_template("views/all_articles.html").unwrap();
}

pub fn render(db_pool: Arc<Pool>, tag: Option<&String>) -> String {
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    let db_request = if let Some(tag) = tag {
        db_pool.prep_exec("SELECT articles.id, articles.titre, articles.content, group_concat( tags.tag SEPARATOR ', ' ) AS 'tags' FROM articles LEFT JOIN tags on tags.article_id = articles.id WHERE articles.id IN (SELECT articles.id FROM articles LEFT JOIN tags on tags.article_id = articles.id WHERE tags.tag = :tag) GROUP BY articles.id ORDER BY articles.date DESC", (tag,))
    } else {
        db_pool.prep_exec("SELECT articles.id, articles.titre, articles.content, group_concat( tags.tag SEPARATOR ', ' ) AS 'tags' FROM articles LEFT JOIN tags on tags.article_id = articles.id GROUP BY articles.id ORDER BY articles.date DESC", ())
    };

    let articles: Vec<ViewVar> = db_request
        .map(|result| {
            result
                .map(std::result::Result::unwrap)
                .map(|row| {
                    let (id, titre, content, tags): (i32, String, String, Option<String>) =
                        mysql::from_row(row);
                    let mut article: HashMap<String, ViewVar> = HashMap::new();
                    let content_cut = content
                        .replace("\n", "<br/>")
                        .chars()
                        .take(MAXIMAL_PREVIEW_LENGTH)
                        .collect::<String>();
                    let url_title = titre
                        .chars()
                        .filter(|elem| elem.is_alphanumeric() || *elem == ' ')
                        .collect::<String>()
                        .replace(" ", "-")
                        .to_lowercase();

                    if let Some(tags) = tags {
                        article.insert(
                            "tags".to_string(),
                            tags.split(", ")
                                .map(ViewVar::from)
                                .collect::<Vec<ViewVar>>()
                                .into(),
                        );
                    }
                    article.insert("link".to_string(), format!("{}-{}", id, url_title).into());
                    article.insert("title".to_string(), titre.into());
                    article.insert(
                        "content".to_string(),
                        format!(
                            "{}{}",
                            content_cut,
                            if content_cut.len() >= MAXIMAL_PREVIEW_LENGTH {
                                "..."
                            } else {
                                ""
                            }
                        )
                        .into(),
                    );
                    article.into()
                })
                .collect()
        })
        .unwrap();

    add_to_view!(vars, articles: articles);
    add_to_view!(vars, section: render_view(&HTML_STRUCTURE, &vars));
    add_to_view!(vars, title: "Articles - Lancelot Owczarczak");
    render_view(&STRUCT, &vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view() {
        lazy_static::initialize(&HTML_STRUCTURE);
    }
}
