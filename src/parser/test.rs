#[test]
fn test_null() {
    let json = r#"null"#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    assert_eq!(parsed, super::Json::Null);
}

#[test]
fn test_true() {
    let json = r#"true"#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    assert_eq!(parsed, super::Json::Bool(true));
}

#[test]
fn test_false() {
    let json = r#"false"#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    assert_eq!(parsed, super::Json::Bool(false));
}

#[test]
fn test_number() {
    let json = r#"-3.14"#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    assert_eq!(parsed, super::Json::Number(-3.14));
}

#[test]
fn test_exponent() {
    let json = r#"3e8"#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    assert_eq!(parsed, super::Json::Number(3e8));
}

#[test]
fn test_string() {
    let json = r#" "hello world" "#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    assert_eq!(parsed, super::Json::String(String::from("hello world")));
}

#[test]
fn test_escaped_string() {
    let json = r#""quotation mark: \", reverse solidus: \\, solidus: \/, backspace: \b, formfeed: \f, linefeed: \n, carriage return: \r, horizontal tab: \t""#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    let expected = "quotation mark: \", reverse solidus: \\, solidus: /, backspace: \x08, formfeed: \x0C, linefeed: \n, carriage return: \r, horizontal tab: \t";
    assert_eq!(parsed, super::Json::String(String::from(expected)));
}

#[test]
fn test_utf8() {
    let json = r#""ラメン""#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    let expected = "ラメン";
    assert_eq!(parsed, super::Json::String(String::from(expected)));
}

#[test]
fn test_unicode() {
    let json = r#""\u0064\u0065\u0063o\u0064\u0065 m\u0065: \ud83d\udca9""#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    let expected = "\u{0064}\u{0065}\u{0063}o\u{0064}\u{0065} m\u{0065}: \u{fffd}\u{fffd}";
    assert_eq!(parsed, super::Json::String(String::from(expected)));
}

#[test]
fn test_object() {
    let json = r#"{"key1": false, "key2": 13}"#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    let keys = vec![String::from("key1"), String::from("key2")];
    let values = vec![super::Json::Bool(false), super::Json::Number(13.0)];
    let expected: super::HashMap<_, _> = keys.into_iter().zip(values.into_iter()).collect();
    assert_eq!(parsed, super::Json::Object(expected));
}

#[test]
fn test_array() {
    let json = r#"[1, "string", true]"#;
    let parsed = super::parse_json(&mut json.chars().peekable()).unwrap();
    let first = super::Json::Number(1.0);
    let second = super::Json::String(String::from("string"));
    let third = super::Json::Bool(true);
    let expected = vec![first, second, third];
    assert_eq!(parsed, super::Json::Array(expected));
}