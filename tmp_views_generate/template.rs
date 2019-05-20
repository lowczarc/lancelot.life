enum HtmlValue {
    Litteral(&'static str),
    Value(ViewContent),
}

use HtmlValue::*;

enum ViewContent {
    Array(&'static str, &'static str, &'static [HtmlValue]),
    Content(&'static str),
}

use ViewContent::*;

pub const HTMLDOC: &[HtmlValue] = 
