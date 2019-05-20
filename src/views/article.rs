use crate::views::{HtmlValue::{self, *}, ViewContent::*};

pub const HTML_STRUCTURE: &[HtmlValue] = include!(concat!(env!("CARGO_MANIFEST_DIR"), "/views/article.html.in"));
