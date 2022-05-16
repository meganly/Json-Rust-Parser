//! # JSON Parser
//!
//! This crate contains a JSON parser that can be used to
//! deserialize a string of JSON data into a Rust data structure.
//!
//! # Examples
//! ```
//!     // Some JSON input data as a raw &str
//!     let example = r#"
//!         {
//!             "number_int": 1,
//!             "number_float": 2.44,
//!             "null_value": null,
//!             "bool_true": true,
//!             "bool_false": false,
//!             "string_easy": "string_value",
//!             "object" : {
//!                 "key1" : 1
//!             },
//!             "array_mixed": [
//!                 1,
//!                 2,
//!                 "string in array",
//!                 {
//!                     "key": "value"
//!                 }
//!             ]
//!         }"#;
//!
//!     // Parse the string of data into a Json enum
//!     let parsed = parse_json(&mut example.chars().peekable()).unwrap();
//!
//!     // Print the deserialized data
//!     println!("{:?}", parsed);
//! ```

pub mod parser;

use crate::parser::parse_json;

fn main() {
    // Some JSON input data as a raw &str
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

    // Parse the string of data into a Json enum
    let parsed = parse_json(&mut example.chars().peekable()).unwrap();

    // Print the deserialized data
    println!("{:?}", parsed);

}
