extern crate serde_json;
extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::{Value, Result};
use std::fs::File;
use std::io::Write;
use std::io;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    vip: bool,
}

fn read_json_from_str(data: &str) -> Result<Value> {
    serde_json::from_str(data)
}

fn print_person_info(person: &Value) {
    println!("Please call {} at the number {}", person["name"], person["age"]);
}

fn create_and_print_json() -> Value {
    let person = Person {
        name: String::from("John Doe"),
        age: 43,
        vip: true,
    };

    let json_data = serde_json::to_value(&person).unwrap();
    println!("{}", json_data);
    json_data
}

fn save_json_to_file(json_data: &Value, filename: &str) -> std::result::Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(serde_json::to_string_pretty(json_data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.as_bytes())?;
    Ok(())
}
fn main() {
    let data = r#"{"name": "John Doe", "age": 30, "vip": true}"#;
    match read_json_from_str(data) {
        Ok(v) => {
            print_person_info(&v);

            let new_json_data = create_and_print_json();
            save_json_to_file(&new_json_data, "output.json").unwrap();
        }
        Err(e) => println!("Error parsing JSON: {}", e),
    }
}
