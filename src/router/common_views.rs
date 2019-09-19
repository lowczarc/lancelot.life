use lazy_static::lazy_static;

use crate::{
    response::{HttpStatus, Response},
    template::{read_template, HtmlView},
};

lazy_static! {
    pub static ref STRUCT: HtmlView = read_template("views/common/struct.html").unwrap();
    pub static ref ASIDE: HtmlView = read_template("views/common/aside.html").unwrap();
}

pub fn default_http_status(status: HttpStatus) -> Response {
    let mut res = Response::new();

    match status {
        HttpStatus::NotFound => {
            res.status(HttpStatus::NotFound);
            res.header("Content-type".to_string(), "text/html".to_string());
            res.body("<!DOCTYPE html><html><body><h1>Not Found</h1><p>The requested URL was not found on this server.</p></body></html>".to_string());
        }
        HttpStatus::Forbidden => {
            res.status(HttpStatus::Forbidden);
            res.header("Content-type".to_string(), "text/html".to_string());
            res.body("<!DOCTYPE html><html><body><h1>Forbidden</h1><p>You don't have permission to access on this server.</p></body></html>".to_string());
        }
        x => {
            res.status(x);
            res.header("Content-type".to_string(), "text/html".to_string());
            res.body(
                "<!DOCTYPE html><html><body><h1>Internal Server Error</h1></body></html>"
                    .to_string(),
            );
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_struct() {
        lazy_static::initialize(&STRUCT);
    }

    #[test]
    fn test_aside() {
        lazy_static::initialize(&ASIDE);
    }
}
