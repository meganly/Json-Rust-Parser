pub mod parser;

use crate::parser::parse_json;

fn main() {

    let example = r#"{"key1": false, "key2": 13}"#;
    parse_json(&mut example.chars().peekable());

}
