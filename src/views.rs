pub mod article;

#[derive(Debug)]
pub enum HtmlValue {
    Litteral(&'static str),
    Value(ViewContent),
}

#[derive(Debug)]
pub enum ViewContent {
    Array(&'static str, &'static str, &'static [HtmlValue]),
    Content(&'static str),
}
