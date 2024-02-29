
    use serde::{Deserialize, Serialize};
    use serde_json::{Result, Value};
    use std::fs::File;
    use std::io;
    use std::io::Write;


    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u32,
        vip: bool,
    }

    pub fn read_json_from_str(data: &str) -> Result<Value> {
        serde_json::from_str(data)
    }

    pub fn print_person_info(person: &Value) {
        println!(
            "Please call {} at the number {}",
            person["name"], person["age"]
        );
    }

    //AI
    pub fn read_json_from_file(
        filename: &str,
    ) -> std::result::Result<Value, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        serde_json::from_reader(reader).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
    //AI
    pub fn print_json_file_contents(filename: &str) {
        match read_json_from_file(filename) {
            Ok(v) => {
                println!("Contents of {}:", filename);
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
            }
            Err(e) => println!("Error reading JSON file {}: {}", filename, e),
        }
    }
    pub fn create_and_print_json() -> Value {
        let person = Person {
            name: String::from("Khmayes Bobtan"),
            age: 69,
            vip: true,
        };

        let json_data = serde_json::to_value(&person).unwrap();
        println!("{}", json_data);
        json_data
    }

    pub fn save_json_to_file(json_data: &Value, filename: &str) -> std::result::Result<(), io::Error> {
        let mut file = File::create(filename)?;
        file.write_all(
            serde_json::to_string_pretty(json_data)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                .as_bytes(),
        )?;
        Ok(())
    }

