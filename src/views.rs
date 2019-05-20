pub type HtmlView = &'static[HtmlValue];

#[macro_export]
macro_rules! import_view {
  ($e:expr) => {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $e, ".in"))
  }
}

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
