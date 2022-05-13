pub mod parser;

use crate::parser::parse_json;

fn main() {

    let example = r#"
        {
            "number_int": 1,
            "number_float": 2.44,
            "null_value": null,
            "bool_true": true,
            "bool_false": false,
            "string_easy": "string_value",
            "object" : {
                "key1" : 1
            },
            "array_mixed": [
                1,
                2,
                "string in array",
                {
                    "key": "value"
                }
            ]
        }"#;

    let parsed = parse_json(&mut example.chars().peekable()).unwrap();

    println!("{:?}", parsed);

}
