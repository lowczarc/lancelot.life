use std::collections::HashMap;

pub type HtmlView = &'static[HtmlValue];

#[macro_export]
macro_rules! import_view {
  ($e:expr) => {{
    #[allow(unused)] use crate::views::{{HtmlValue::*, ViewContent::*}};
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $e, ".in")) // TODO: Remove include! and interprete views file at compile time
  }}
}

macro_rules! add_to_view {
    (@assign $id:ident, { $( $inid:tt: $invalue:tt ),* }) => {
        let mut tmp_object: HashMap<String, ViewVar> = HashMap::new();
        $(
            let mut tmp_value: ViewVar;
            add_to_view!(@assign tmp_value, $invalue);
            tmp_object.insert(stringify!($inid).into(), tmp_value);
        )*
        $id = ViewVar::from(&tmp_object);
    };

    (@assign $id:ident, [ $( $value:tt ),* ]) => {
        let mut tmp_vec: Vec<ViewVar> = Vec::new();
        let mut tmp_value: ViewVar;
        $(
            add_to_view!(@assign tmp_value, $value);
            tmp_vec.push(tmp_value);
        )*
        $id = ViewVar::from(tmp_vec);
    };

    (@assign $id:ident, $value:expr) => {
        $id = ViewVar::from($value);
    };

    ($vars:ident, $id:tt: { $( $inid:tt: $value:tt ),* }) => {
        let mut tmp_value: ViewVar;
        add_to_view!(@assign tmp_value, { $( $inid: $value ),* });
        $vars.insert(stringify!($id).into(), tmp_value);
    };

    ($vars:ident, $id:tt: [ $( $value:tt ),* ]) => {
        let mut tmp_value: ViewVar;
        add_to_view!(@assign tmp_value, [ $( $value ),* ]);
        $vars.insert(stringify!($id).into(), tmp_value);
    };

    ($vars:ident, $id:tt: $value:expr) => {
        let mut tmp_value: ViewVar;
        add_to_view!(@assign tmp_value, $value);
        $vars.insert(stringify!($id).into(), tmp_value);
    };
}

#[derive(Debug, PartialEq)]
pub enum ViewVar {
    Simple(String),
    Object(HashMap<String, ViewVar>),
    Array(Vec<ViewVar>)
}

impl<'a> From<String> for ViewVar {
    fn from(simple: String) -> Self {
        ViewVar::Simple(simple)
    }
}

impl<'a> From<&str> for ViewVar {
    fn from(simple: &str) -> Self {
        ViewVar::Simple(simple.into())
    }
}

impl<'a> From<HashMap<String, ViewVar>> for ViewVar {
    fn from(object: HashMap<String, ViewVar>) -> Self {
        ViewVar::Object(object)
    }
}

impl<'a> From<Vec<ViewVar>> for ViewVar {
    fn from(array: Vec<ViewVar>) -> Self {
        ViewVar::Array(array)
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

fn get_var_value<'a>(index: &str, vars: &HashMap<&String, &'a ViewVar>) -> Option<&'a ViewVar>{
    let mut values = index.split('.').peekable();
    let mut tmp_vars = vars;
    let mut borrow_saver;

    while let Some(value) = values.next() {
        if let Some(var) = tmp_vars.get(&value.to_string()) {
            if values.peek().is_some() {
                if let ViewVar::Object(var) = var {
                    borrow_saver = var.iter().collect();
                    tmp_vars = &borrow_saver;
                } else {
                    return None;
                }
            } else {
                return Some(var);
            }
        } else {
            return None;
        }
    }
    None
}

impl HtmlValue {
    fn render(&self, vars: HashMap<&String, &ViewVar>) -> Option<String> {
        match self {
            HtmlValue::Litteral(value) => Some(value.to_string()),
            HtmlValue::Value(ViewContent::Content(value)) => {
                if let Some(ViewVar::Simple(var)) = get_var_value(value, &vars) {
                    Some(var.to_string())
                } else {
                    None
                }
            }
            HtmlValue::Value(ViewContent::Array(array, name, childrens)) => {
                if let Some(ViewVar::Array(array)) = get_var_value(array, &vars) {

                    return Some(array.iter().map(|elem_var| {
                        childrens.iter().map(|elem_html| {
                            let added_var = name.to_string();
                            let mut tmp_vars = vars.clone();
                            tmp_vars.insert(&added_var, elem_var);

                            if let Some(value) = elem_html.render(tmp_vars) {
                                value
                            } else {
                                String::new()
                            }
                        }).collect::<String>()
                    }).collect::<String>());
                }
                None
            }
        }
    }
}

pub fn render_view(view: HtmlView, vars: HashMap<String, ViewVar>) -> String {
    view.iter().map(|elem| if let Some(value) = elem.render(vars.iter().collect()) {
            value
        } else {
            String::new()
        }).collect::<String>()
}