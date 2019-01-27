
extern crate valico;
extern crate serde_json;

// pub mod declarations { include!("declarations/mod.rs"); }

use valico::json_schema;
use std::vec::Vec;
use serde_json::Value;
use std::fs::File;

// #![feature(core_intrinsics)]

// trait typeOF<T> {
//     fn(&self) -> T
// }

// pub fn validateSchema(&schema: Some(), &options: Some()) 
// {
    
// } 
//options - js
//schema - json

pub fn validate_object<T>(schema: File, options: &Value) -> Vec<T>
{
    let json_v4_schema: Value = serde_json::from_reader(schema).unwrap();
    let mut scope = json_schema::Scope::new();
    let validate = scope.compile_and_return(json_v4_schema.clone(), false).unwrap();
    let valid = validate.validate(options).is_valid();
    if valid {
        Vec::new()
    } else {
        panic!("invalid schema!")
    }
}

// pub fn filterErrors(&errors: vec!)
// {
//     let mut newErrors = Vec::new();
//     // for i in &errors {
//     //     let mut dataPath = errors[i].dataPath;
//     //     let mut children = Vec::new();
//     //     for 
//     // }
// }