//! Parse JSON data into a Rust data structure

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;


/// Represents the different data types available in JSON
#[derive(PartialEq, Debug)]
pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl Json {
    /// Destructure a `Json::String` into a Rust `String`
    fn destructure (self) -> Result<String, String> {
        match self {
            Json::String(s) => Ok(s),
            _ => Err(String::from("can only destructure Json::String")),
        }
    }
}

/// Consume the whitespace chars in a peekable iterator
fn parse_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(c) = chars.peek() {
        if c.is_ascii_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

/// Determine if the chars in a peekable iterator match the chars in a vector
fn parse_expected(chars: &mut Peekable<Chars>, expected: Vec<char>) -> Result<(), String> {
    for expect in expected {
        if chars.next() != Some(expect) {
            return Err(String::from("expected valid token"));
        }
    }
    Ok(())
}

/// Parse a peekable iterator of 4 hex digits into a unicode char
fn parse_unicode(chars: &mut Peekable<Chars>) -> char {
    let unicode: String = chars.take(4).collect();

    u32::from_str_radix(&unicode, 16)
        .ok()
        .and_then(std::char::from_u32)
        .unwrap_or('\u{fffd}')
}


/// Parse a peekable iterator over the chars of a string slice into a Json enum
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
                if !matches!(tk, Some(',') | Some('}')) {
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
                if !matches!(tk, Some(',') | Some(']')) {
                    return Err(String::from("expected valid token"));
                }
            }
            return Ok(Json::Array(arg));
        }
        Some('"') => {
            let mut arg = String::new();
            while let Some(c) = chars.next() {
                if c == '\\' {
                    match chars.next() {
                        Some('"') => arg.push('\"'),
                        Some('\\') => arg.push('\\'),
                        Some('/') => arg.push('/'),
                        Some('b') => arg.push('\x08'),
                        Some('f') => arg.push('\x0C'),
                        Some('n') => arg.push('\n'),
                        Some('r') => arg.push('\r'),
                        Some('t') => arg.push('\t'),
                        Some('u') => arg.push(parse_unicode(chars)),
                        _ => return Err(String::from("expected valid token")),
                    }
                } else if c == '"' {
                    break;
                } else {
                    arg.push(c);
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
        Some(x) if matches!(x, '0'..='9' | '-') => {
            let mut arg = x.to_string();
            while let Some(c) = chars.peek() {
                if matches!(c, '0'..='9' | '.' | 'E' | 'e' | '+' | '-') {
                    arg.push(*c);
                    chars.next();
                } else {
                    break;
                }
            }
            let arg = arg.parse().expect("invalid number");
            return Ok(Json::Number(arg));
        }
        _ => return Err(String::from("expected valid token")),
    }
}