use crate::template::{HtmlValue, HtmlView};
use std::collections::HashMap;

#[macro_export]
macro_rules! add_to_view {
    (@assign $id:ident, { $( $inid:tt: $invalue:tt ),* }) => {
        let mut tmp_object: HashMap<String, ViewVar> = HashMap::new();
        $(
            let tmp_value: ViewVar;
            add_to_view!(@assign tmp_value, $invalue);
            tmp_object.insert(stringify!($inid).into(), tmp_value);
        )*
        $id = ViewVar::from(tmp_object);
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
        let tmp_value: ViewVar;
        add_to_view!(@assign tmp_value, { $( $inid: $value ),* });
        $vars.insert(stringify!($id).into(), tmp_value);
    };

    ($vars:ident, $id:tt: [ $( $value:tt ),* ]) => {
        let tmp_value: ViewVar;
        add_to_view!(@assign tmp_value, [ $( $value ),* ]);
        $vars.insert(stringify!($id).into(), tmp_value);
    };

    ($vars:ident, $id:tt: $value:expr) => {
        let tmp_value: ViewVar;
        add_to_view!(@assign tmp_value, $value);
        $vars.insert(stringify!($id).into(), tmp_value);
    };
}

macro_rules! create_view_var {
    ({ $( $inid:tt: $value:tt ),* }) => {{
        let tmp_value: ViewVar;
        add_to_view!(@assign tmp_value, { $( $inid: $value ),* });
        tmp_value
    }}
}

#[derive(Debug, PartialEq)]
pub enum ViewVar {
    Simple(String),
    Object(HashMap<String, ViewVar>),
    Array(Vec<ViewVar>),
}

impl From<String> for ViewVar {
    fn from(simple: String) -> Self {
        ViewVar::Simple(simple)
    }
}

impl From<&str> for ViewVar {
    fn from(simple: &str) -> Self {
        ViewVar::Simple(simple.into())
    }
}

impl From<HashMap<String, ViewVar>> for ViewVar {
    fn from(object: HashMap<String, ViewVar>) -> Self {
        ViewVar::Object(object)
    }
}

impl From<Vec<ViewVar>> for ViewVar {
    fn from(array: Vec<ViewVar>) -> Self {
        ViewVar::Array(array)
    }
}

fn get_var_value<'a>(index: &str, vars: &HashMap<&String, &'a ViewVar>) -> Option<&'a ViewVar> {
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
            HtmlValue::Content(value) => {
                if let Some(ViewVar::Simple(var)) = get_var_value(value, &vars) {
                    Some(var.to_string())
                } else {
                    None
                }
            }
            HtmlValue::Array(array, name, childrens) => match get_var_value(array, &vars) {
                Some(ViewVar::Array(array)) => Some(
                    array
                        .iter()
                        .map(|elem_var| {
                            childrens
                                .iter()
                                .map(|elem_html| {
                                    let added_var = name.to_string();
                                    let mut tmp_vars = vars.clone();
                                    tmp_vars.insert(&added_var, elem_var);

                                    if let Some(value) = elem_html.render(tmp_vars) {
                                        value
                                    } else {
                                        String::new()
                                    }
                                })
                                .collect::<String>()
                        })
                        .collect::<String>(),
                ),
                Some(_) => Some(
                    childrens
                        .iter()
                        .map(|elem_html| {
                            if let Some(value) = elem_html.render(vars.clone()) {
                                value
                            } else {
                                String::new()
                            }
                        })
                        .collect::<String>(),
                ),
                None => None,
            },
        }
    }
}

pub fn render_view(view: &HtmlView, vars: &HashMap<String, ViewVar>) -> String {
    view.iter()
        .map(|elem| {
            if let Some(value) = elem.render(vars.iter().collect()) {
                value
            } else {
                String::new()
            }
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_var_value() {
        let mut vars: HashMap<String, ViewVar> = HashMap::new();
        add_to_view!(vars, var1: "test1");
        add_to_view!(vars, var2: {
            var1: "test2",
            var2: [
                "test3",
                {
                    var1: "test4"
                }
            ]
        });

        assert_eq!(
            Some(&ViewVar::Simple("test1".to_string())),
            get_var_value("var1", &vars.iter().collect())
        );

        assert_eq!(
            Some(&ViewVar::Simple("test2".to_string())),
            get_var_value("var2.var1", &vars.iter().collect())
        );

        // TODO: Add support for get_var_value for array
        /*
            assert_eq!(
                Some(&ViewVar::Simple("test3".to_string())),
                get_var_value("var2.var2[0]", &vars.iter().collect())
            );

            assert_eq!(
                Some(&ViewVar::Simple("test4".to_string())),
                get_var_value("var2.var2[1].var1", &vars.iter().collect())
            );
        */
    }

    #[test]
    fn litteral_render() {
        use HtmlValue::*;

        let html: HtmlView = vec![Litteral("test".to_string())];
        let vars: HashMap<String, ViewVar> = HashMap::new();

        assert_eq!(render_view(&html, &vars), "test".to_string());
    }

    #[test]
    fn content_render() {
        use crate::views::HtmlValue::*;

        let html: HtmlView = vec![
            Content("content".to_string()),
            Litteral("|".to_string()),
            Content("object.key1".to_string()),
            Litteral("|".to_string()),
            Content("object.key2".to_string()),
        ];
        let mut vars: HashMap<String, ViewVar> = HashMap::new();

        add_to_view!(vars, content: "test1");
        add_to_view!(vars, object: { key1: "test2", key2: "test3" });

        assert_eq!(render_view(&html, &vars), "test1|test2|test3".to_string());
    }

    #[test]
    fn array_render() {
        use crate::views::HtmlValue::*;

        let html: HtmlView = vec![Array(
            "array".to_string(),
            "elem".to_string(),
            vec![
                Litteral("|".to_string()),
                Content("elem".to_string()),
                Litteral("|".to_string()),
            ],
        )];
        let mut vars: HashMap<String, ViewVar> = HashMap::new();

        add_to_view!(vars, array: ["one", "two", "three", "four"]);

        assert_eq!(
            render_view(&html, &vars),
            "|one||two||three||four|".to_string()
        );
    }
}
