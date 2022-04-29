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

pub fn parse_json(chars: &mut Peekable<Chars>) -> Json {
    let mut tk = chars.next().unwrap();
    if tk == '{' {
        let mut arg = HashMap::new();
        while tk != '}' {
            let mut key = String::new();
            chars.next();
            tk = chars.next().unwrap();
            while tk != '"' {
                key.push_str(&tk.to_string());
                tk = chars.next().unwrap();
            }
            chars.next();
            let value = parse_json(chars);
            arg.insert(key, value);
            tk = chars.next().unwrap();
        }
        return Json::Object(arg);
    } else if tk == '[' {
        let mut arg = Vec::new();
        while tk != ']' {
            arg.push(parse_json(chars));
            tk = chars.next().unwrap();
        }
        return Json::Array(arg);
    } else if tk == '"' {
        let mut arg = String::new();
        tk = chars.next().unwrap();
        while tk != '"' {
            arg.push_str(&tk.to_string());
            tk = chars.next().unwrap();
        }
        return Json::String(arg);
    } else if tk == 'n' {
        chars.next();
        chars.next();
        chars.next();
        return Json::Null;
    } else if tk == 't' {
        chars.next();
        chars.next();
        chars.next();
        return Json::Bool(true);
    } else if tk == 'f' {
        chars.next();
        chars.next();
        chars.next();
        chars.next();
        return Json::Bool(false);
    } else {
        let mut arg = String::new();
        while chars.peek().unwrap().is_numeric() || chars.peek().unwrap() == &'.' {
            arg.push_str(&tk.to_string());
            tk = chars.next().unwrap();
        }
        let arg = arg.parse().unwrap();
        return Json::Number(arg);
    }
}