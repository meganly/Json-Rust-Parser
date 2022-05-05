use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;

pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Json {
    fn destructure (self) -> Result<String, String> {
        match self {
            Json::String(s) => Ok(s),
            _ => Err(String::from("can only destructure Json::String")),
        }
    }
}

fn parse_whitespace(chars: &mut Peekable<Chars>) {
    while chars.peek() == Some(&' ') {
        chars.next();
    }
}

fn parse_expected(chars: &mut Peekable<Chars>, expected: Vec<char>) -> Result<(), String> {
    for expect in expected {
        if chars.next() != Some(expect) {
            return Err(String::from("expected valid token"));
        }
    }
    Ok(())
}

pub fn parse_json(chars: &mut Peekable<Chars>) -> Result<Json, String> {
    parse_whitespace(chars);
    let mut tk = chars.next();
    match tk {
        Some('{') => {
            let mut arg = HashMap::new();
            while tk != Some('}') {
                let key = parse_json(chars)?.destructure()?;
                parse_whitespace(chars);
                parse_expected(chars, vec![':'])?;
                let value = parse_json(chars)?;
                arg.insert(key, value);
                parse_whitespace(chars);
                tk = chars.next();
                if tk != Some(',') && tk != Some('}') {
                    return Err(String::from("expected valid token"));
                }
            }
            return Ok(Json::Object(arg));
        }
        Some('[') => {
            let mut arg = Vec::new();
            while tk != Some(']') {
                arg.push(parse_json(chars)?);
                parse_whitespace(chars);
                tk = chars.next();
                if tk != Some(',') && tk != Some(']') {
                    return Err(String::from("expected valid token"));
                }
            }
            return Ok(Json::Array(arg));
        }
        Some('"') => {
            let mut arg = String::new();
            while let Some(c) = chars.next() {
                if c == '"' {
                    break;
                } else {
                    arg.push_str(&c.to_string());
                }
            }
            return Ok(Json::String(arg));
        }
        Some('n') => {
            parse_expected(chars, vec!['u', 'l', 'l'])?;
            return Ok(Json::Null);
        }
        Some('t') => {
            parse_expected(chars, vec!['r', 'u', 'e'])?;
            return Ok(Json::Bool(true));
        }
        Some('f') => {
            parse_expected(chars, vec!['a', 'l', 's', 'e'])?;
            return Ok(Json::Bool(false));
        }
        Some(x) if x.is_digit(10) || x == '-' => {
            let mut arg = x.to_string();
            while let Some(c) = chars.peek() {
                if c.is_digit(10) || c == &'.' {
                    arg.push_str(&c.to_string());
                    chars.next();
                } else {
                    break;
                }
            }
            let arg = match arg.parse::<f64>() {
                Ok(x) => x,
                Err(_) => panic!("invalid number"),
            };
            return Ok(Json::Number(arg));
        }
        _ => return Err(String::from("expected valid token")),
    }
}