
// fn webpack(option)
extern crate futures;
extern crate valico;
// extern crate serde_json;

// mod declarations;
mod validateSchema;
pub mod declarations;

use std::fs::File;
use serde_json::{
    Result, 
    Value
};
// use validateSchema::validateObject;

fn generateConfig() -> declarations::RustpackOptions
{
    let my_config = declarations::RustpackOptions {
        bail: false,
        cache: true,
        context: "hyITA".to_string(),
        dependencies: vec![
            "react".to_string(), 
            "redux".to_string()
        ],
        mode: "development".to_string(),
        name: "HYITA".to_string(),
        profile: false,
        recordsPath: "dist/".to_string(),
        watch: true
    };
    my_config
}

pub fn rustpack<'a>(options: &'a declarations::RustpackOptions)
{
    let rustpackOptionsSchema = File::open("schemas/RustpackOptions.json").unwrap();
    let rustpackOptionsValidationErrors = validateSchema::validateObject::<i32>(
        rustpackOptionsSchema,
        options
    );
    println!("{}", rustpackOptionsValidationErrors.len());
}

fn main() {
    rustpack(&generateConfig());
    println!("Hello, world!");
}
