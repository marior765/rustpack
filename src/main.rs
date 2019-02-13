
// fn webpack(option)

#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate valico;
extern crate serde_json;

mod declarations;
mod validateSchema;
mod MultiCompiler;

use std::fs::File;
use validateSchema::validate_object;
use serde_json::{
    Result, 
    Value,
    from_str
};


pub fn rustpack<'a>(options: &Value)
{
    let rustpack_options_schema = File::open("schemas/RustpackOptions.json").unwrap();
    let rustpack_options_validation_errors = validate_object::<i32>(
        rustpack_options_schema,
        options
    );
    let compiler = MultiCompiler::new();
    println!("{}", rustpack_options_validation_errors.len());
}

fn main() {
    let data = r#"
    {
        "bail": false,
        "cache": true
    }"#;
    let j: Value = from_str(data).unwrap();
    rustpack(&j);
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
    println!("Hello, world!");
}
