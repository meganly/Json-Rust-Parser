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

fn destructure (json: Json) -> String {
    match json {
        Json::String(s) => s,
        _ => panic!("can only destructure Json::String"),
    }
 }

fn parse_whitespace(chars: &mut Peekable<Chars>) {
    while chars.peek().unwrap() == &' ' {
        chars.next().unwrap();
    }
}

pub fn parse_json(chars: &mut Peekable<Chars>) -> Json {
    parse_whitespace(chars);
    let mut tk = chars.next().unwrap();
    match tk {
        '{' => {
            let mut arg = HashMap::new();
            while tk != '}' {
                let key = destructure(parse_json(chars));
                parse_whitespace(chars);
                chars.next();
                let value = parse_json(chars);
                arg.insert(key, value);
                parse_whitespace(chars);
                tk = chars.next().unwrap();
            }
            return Json::Object(arg);
        }
        '[' => {
            let mut arg = Vec::new();
            while tk != ']' {
                arg.push(parse_json(chars));
                parse_whitespace(chars);
                tk = chars.next().unwrap();
            }
            return Json::Array(arg);
        }
        '"' => {
            let mut arg = String::new();
            tk = chars.next().unwrap();
            while tk != '"' {
                arg.push_str(&tk.to_string());
                tk = chars.next().unwrap();
            }
            return Json::String(arg);
        }
        'n' => {
            chars.next();
            chars.next();
            chars.next();
            return Json::Null;
        }
        't' => {
            chars.next();
            chars.next();
            chars.next();
            return Json::Bool(true);
        }
        'f' => {
            chars.next();
            chars.next();
            chars.next();
            chars.next();
            return Json::Bool(false);
        }
        _ => {
            let mut arg = String::new();
            while chars.peek().unwrap().is_numeric() || chars.peek().unwrap() == &'.' {
                arg.push_str(&tk.to_string());
                tk = chars.next().unwrap();
            }
            let arg = arg.parse().unwrap();
            return Json::Number(arg);
        }
    }
}