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

const HTML_STRUCTURE: HtmlView = import_view!("views/projects.html");

lazy_static! {
    pub static ref PROJECTS: Route = (Regex::new(r"^/projects/?$").unwrap(), project_route);
}

pub fn render(db_pool: Arc<Pool>, tag: Option<&String>) -> String {
    let mut vars: HashMap<String, ViewVar> = HashMap::new();
    let mut links: HashMap<i32, Vec<ViewVar>> = HashMap::new();

    db_pool.prep_exec("SELECT projects.id, links.type, links.link FROM projects INNER JOIN links ON projects.id = links.project_id", ())
        .map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                let (id, image, link): (i32, String, String) = mysql::from_row(row);

                if let Some(vec) = links.get_mut(&id) {
                    vec.push(create_view_var!({ href: link, image: image }));
                } else {
                    links.insert(id, vec![create_view_var!({ href: link, image: image })]);
                };
            }).last();
        }).unwrap();

    let db_request = if let Some(tag) = tag {
        db_pool.prep_exec("SELECT projects.id, projects.titre, projects.image, group_concat( tags.tag SEPARATOR ', ' ) AS 'tags' FROM projects LEFT JOIN tags on tags.project_id = projects.id WHERE projects.id IN (SELECT projects.id FROM projects LEFT JOIN tags on tags.project_id = projects.id WHERE tags.tag = :tag) GROUP BY projects.id ORDER BY projects.date DESC", (tag,))
    } else {
        db_pool.prep_exec("SELECT projects.id, projects.titre, projects.image, group_concat( tags.tag SEPARATOR ', ' ) AS 'tags' FROM projects LEFT JOIN tags on tags.project_id = projects.id GROUP BY projects.id ORDER BY projects.date DESC", ())
    };

    let projects: Vec<ViewVar> = db_request
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, titre, image, tags): (i32, String, Option<String>, Option<String>) =
                        mysql::from_row(row);
                    let mut project: HashMap<String, ViewVar> = HashMap::new();

                    if let Some(tags) = tags {
                        project.insert(
                            "tags".to_string(),
                            tags.split(", ")
                                .map(|elem| ViewVar::from(elem))
                                .collect::<Vec<ViewVar>>()
                                .into(),
                        );
                    }
                    if let Some(image) = image {
                        project.insert(
                            "image".to_string(),
                            format!("<img width=60 alt='image' src='{}' />", image).into(),
                        );
                    }
                    if let Some(link) = links.remove(&id) {
                        project.insert("links".to_string(), link.into());
                    }
                    project.insert("title".to_string(), titre.into());
                    project.into()
                })
                .collect()
        })
        .unwrap();

    add_to_view!(vars, projects: projects);
    render_view(HTML_STRUCTURE, vars)
}

pub fn project_route(req: Request, db_pool: Arc<Pool>) -> Result<Response, HttpStatus> {
    let mut res = Response::new();
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(vars, section: render(db_pool, req.query_parse().get("tag")));
    add_to_view!(vars, aside: render_view(ASIDE, HashMap::new()));

    add_to_view!(vars, title: "Lancelot Owczarczak");

    res.header("Content-Type".into(), "text/html; charset=utf8".into());
    res.body(render_view(STRUCT, vars));
    Ok(res)
}
