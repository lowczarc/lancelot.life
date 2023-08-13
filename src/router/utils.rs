use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;

use futures::executor::block_on;

use sqlx::{self, Pool, Postgres};

use crate::template::{read_template, HtmlView};
use crate::views::{render_view, ViewVar};

struct HeartBeat {
    heart_rate: Option<f64>,
}

pub fn heartbeat_delta(db_pool: Arc<Pool<Postgres>>) -> f64 {
    if let Some(Some(x)) = block_on(sqlx::query_as!(HeartBeat, "SELECT heart_rate FROM dblink('dbname=data','SELECT AVG(heart_rate), date::time FROM heart_rate_entry GROUP BY date::time') AS heart_rate_entry(heart_rate float, date time) WHERE date = left(NOW()::time::varchar(255),5)::time;").fetch_all(&*db_pool))
        .unwrap()
        .into_iter()
        .next()
        .map(|e| e.heart_rate) {
            60./x
        } else {
            0.
        }
}

pub fn initial_vars(db_pool: Arc<Pool<Postgres>>) -> HashMap<String, ViewVar> {
    let mut vars: HashMap<String, ViewVar> = HashMap::new();

    add_to_view!(
        vars,
        delta_hb: format!("{}", heartbeat_delta(db_pool.clone()))
    );

    vars
}

lazy_static! {
    pub static ref COMMON_STRUCTURE: HtmlView = read_template("views/common.html").unwrap();
}

pub fn render_in_common_view(view: &HtmlView, vars: &HashMap<String, ViewVar>) -> String {
    let mut common_vars = vars.clone();

    add_to_view!(common_vars, page: render_view(view, vars));

    render_view(&COMMON_STRUCTURE, &common_vars)
}
