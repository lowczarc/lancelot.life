use std::io::{Error, ErrorKind, Read};
use std::str::Chars;

pub type HtmlView = Vec<HtmlValue>;

#[derive(Debug, PartialEq)]
pub enum HtmlValue {
    Litteral(String),
    Array(String, String, Vec<HtmlValue>),
    Content(String),
}

fn skip_spaces(chars: &mut Chars) -> Option<char> {
    let mut c;

    loop {
        c = chars.next();
        if c != Some(' ') && c != Some('\t') && c != Some('\n') {
            break;
        }
    }
    c
}

fn parse_content(template: &mut Chars) -> Result<HtmlValue, std::io::Error> {
    let mut mode_array = false;
    let mut variable_name = String::new();
    let mut array_iter_name = String::new();

    while let Some(mut c) = template.next() {
        if !mode_array {
            match c {
                '}' => {
                    return Ok(HtmlValue::Content(variable_name));
                }
                ' ' | '\t' | '\n' => {
                    c = skip_spaces(template).unwrap();
                    if c != '|' {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Unexpected '{}', expected: '|'", c),
                        ));
                    }
                    mode_array = true;
                }
                _ => variable_name.push(c),
            }
        } else if c == '|' {
            c = skip_spaces(template).unwrap();
            if c != '[' {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unexpected '{}', expected: '['", c),
                ));
            }

            let sub_litteral = parse_litteral(template)?;
            c = skip_spaces(template).unwrap();
            if c != '}' {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unexpected '{}', expected: '}}'", c),
                ));
            }

            return Ok(HtmlValue::Array(
                variable_name,
                array_iter_name,
                sub_litteral,
            ));
        } else {
            array_iter_name.push(c);
        }
    }
    Err(Error::new(
        ErrorKind::UnexpectedEof,
        "Unexpected End Of File",
    ))
}

fn parse_litteral(template: &mut Chars) -> Result<Vec<HtmlValue>, std::io::Error> {
    let mut ret = Vec::new();
    let mut escaped = false;
    let mut current_string = String::new();

    while let Some(c) = template.next() {
        if c == '\\' && !escaped {
            escaped = true;
        } else {
            if c == '{' && !escaped {
                if !current_string.is_empty() {
                    ret.push(HtmlValue::Litteral(current_string));
                }
                current_string = String::new();
                ret.push(parse_content(template)?);
            } else if c == ']' && !escaped {
                if !current_string.is_empty() {
                    ret.push(HtmlValue::Litteral(current_string));
                }
                return Ok(ret);
            } else {
                current_string.push(c);
            }
            escaped = false;
        }
    }
    if !current_string.is_empty() {
        ret.push(HtmlValue::Litteral(current_string));
    }

    Ok(ret)
}

pub fn read_template(path: &str) -> Result<Vec<HtmlValue>, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let mut template = buffer.chars();

    let ret = parse_litteral(&mut template)?;

    if template.next() != None {
        Err(Error::new(ErrorKind::InvalidData, "Unexpected ']'"))
    } else {
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use HtmlValue::*;

    #[test]
    fn only_litteral_template() {
        let template = read_template("tests/templates/only_litteral_template").unwrap();

        assert_eq!(
            template,
            vec![Litteral("test tata tutu [test]{test}\n".to_string())]
        );
    }

    #[test]
    fn litteral_content_template() {
        let template = read_template("tests/templates/litteral_content_template").unwrap();

        assert_eq!(
            template,
            vec![
                Litteral("test ".to_string()),
                Content("tata".to_string()),
                Litteral(" test ".to_string()),
                Content("tutu".to_string()),
                Litteral("\n".to_string())
            ]
        );
    }

    #[test]
    fn array_template() {
        let template = read_template("tests/templates/array_template").unwrap();

        assert_eq!(
            template,
            vec![
                Content("test".to_string()),
                Litteral(" ".to_string()),
                Array(
                    "array".to_string(),
                    "elem".to_string(),
                    vec![
                        Litteral("\n\t".to_string()),
                        Array(
                            "elem.subarray".to_string(),
                            "machin".to_string(),
                            vec![Litteral("test ".to_string()), Content("tata".to_string())]
                        ),
                        Litteral("\n\t".to_string()),
                        Content("test".to_string()),
                        Litteral("\n".to_string())
                    ]
                ),
                Litteral("\n".to_string()),
            ]
        );
    }
}
