use crate::{
    response::{HttpStatus, Response},
    views::HtmlView,
};

pub const STRUCT: HtmlView = import_view!("views/common/struct.html");
pub const ASIDE: HtmlView = import_view!("views/common/aside.html");

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
