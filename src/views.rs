use std::collections::HashMap;

pub type HtmlView = &'static[HtmlValue];

#[macro_export]
macro_rules! import_view {
  ($e:expr) => {{
    use crate::views::{{HtmlValue::*, ViewContent::*}};
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $e, ".in")) // TODO: Remove include! and interprete views file at compile time
  }}
}

macro_rules! internal_add_to_view {
    (@assign $id:ident, { $( $inid:tt: $invalue:tt ),* }) => {
        let mut tmp_object: HashMap<String, &ViewVar> = HashMap::new();
        $(
            let mut tmp_value: ViewVar;
            internal_add_to_view!(@assign tmp_value, $invalue);
            tmp_object.insert(stringify!($inid).into(), &tmp_value);
        )*
        $id = ViewVar::from(&tmp_object);
    };

    (@assign $id:ident, [ $( $value:tt ),* ]) => {
        let mut tmp_vec: Vec<ViewVar> = Vec::new();
        let mut tmp_value: ViewVar;
        $(
            internal_add_to_view!(@assign tmp_value, $value);
            tmp_vec.push(tmp_value);
        )*
        $id = ViewVar::from(tmp_vec);
    };

    (@assign $id:ident, $value:expr) => {
        $id = ViewVar::from($value);
    };
}

macro_rules! add_to_view {
    ($vars:ident, $id:tt: { $( $inid:tt: $value:tt ),* }) => {
        let mut tmp_value: ViewVar;
        internal_add_to_view!(@assign tmp_value, { $( $inid: $value ),* });
        $vars.insert(stringify!($id).into(), &tmp_value);
    };
    ($vars:ident, $id:tt: [ $( $value:tt ),* ]) => {
        let mut tmp_value: ViewVar;
        internal_add_to_view!(@assign tmp_value, [ $( $value ),* ]);
        $vars.insert(stringify!($id).into(), &tmp_value);
    };
    ($vars:ident, $id:tt: $value:expr) => {
        let mut tmp_value: ViewVar;
        internal_add_to_view!(@assign tmp_value, $value);
        $vars.insert(stringify!($id).into(), &tmp_value);
    };
}

#[derive(Debug)]
pub enum ViewVar<'a> {
    Simple(String),
    Object(&'a HashMap<String, &'a ViewVar<'a>>),
    Array(Vec<ViewVar<'a>>)
}

impl<'a> From<String> for ViewVar<'a> {
    fn from(simple: String) -> Self {
        ViewVar::Simple(simple)
    }
}

impl<'a> From<&str> for ViewVar<'a> {
    fn from(simple: &str) -> Self {
        ViewVar::Simple(simple.into())
    }
}

impl<'a> From<&'a HashMap<String, &'a ViewVar<'a>>> for ViewVar<'a> {
    fn from(object: &'a HashMap<String, &'a ViewVar<'a>>) -> Self {
        ViewVar::Object(object)
    }
}

impl<'a> From<Vec<ViewVar<'a>>> for ViewVar<'a> {
    fn from(array: Vec<ViewVar<'a>>) -> Self {
        ViewVar::Array(array)
    }
}

impl<'a> ViewVar<'a> {
    fn is_object(&self) -> bool {
        if let ViewVar::Object(_) = self {
            return true;
        }
        false
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

fn get_var_value<'a>(index: &str, vars: &'a HashMap<String, &ViewVar<'a>>) -> Option<&'a ViewVar<'a>>{
    let mut values = index.split('.').peekable();
    let mut tmp_vars = &ViewVar::Object(vars);

    while let Some(value) = values.next() {
        if let ViewVar::Object(vars) = tmp_vars {
            if let Some(var) = vars.get(value) {
                if values.peek().is_some() {
                    if var.is_object() {
                        tmp_vars = var;
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
    }
    None
}

impl HtmlValue {
    fn render(&self, vars: &HashMap<String, &ViewVar>) -> Option<String> {
        match self {
            HtmlValue::Litteral(value) => Some(value.to_string()),
            HtmlValue::Value(ViewContent::Content(value)) => {
                if let Some(ViewVar::Simple(var)) = get_var_value(value, vars) {
                    Some(var.to_string())
                } else {
                    None
                }
            }
            HtmlValue::Value(ViewContent::Array(array, name, childrens)) => {
                if let Some(ViewVar::Array(array)) = get_var_value(array, vars) {
                    let mut new_hash = vars.clone();

                    return Some(array.iter().map(|elem| {
                        new_hash.insert(name.to_string(), elem);
                        childrens.iter().map(|elem| if let Some(value) = elem.render(&new_hash) {
                            value
                        } else {
                            String::new()
                        }).collect::<String>()
                    }).collect::<String>());
                }
                None
            }
        }
    }
}

pub fn render_view(view: HtmlView, vars: HashMap<String, &ViewVar>) -> String {
    view.iter().map(|elem| if let Some(value) = elem.render(&vars) {
            value
        } else {
            String::new()
        }).collect::<String>()
}