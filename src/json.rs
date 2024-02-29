extern crate serde;
extern crate serde_json;

mod json_manager;

use json_manager::{
    create_and_print_json,
    print_json_file_contents,
    print_person_info,
    read_json_from_str,
    save_json_to_file,
};

fn main() {
    let data = r#"{"name": "Khmayes Bobtan", "age": 9, "vip": true}"#;
    match read_json_from_str(data) {
        Ok(v) => {
            print_person_info(&v);

            let new_json_data = create_and_print_json();
            save_json_to_file(&new_json_data, "output.json").unwrap();
            print_json_file_contents("input.json");
        }
        Err(e) => println!("Error parsing JSON: {}", e),
    }
}
