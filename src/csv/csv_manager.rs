pub mod csv_manager {

    use crate::api::user::user_model::{UserModel};
    use std::error::Error;
    use std::fs::File;
    use std::io::{self};
    use std::path::{Path, PathBuf};
    

    pub fn csv_read<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
        let path = PathBuf::from(filename.as_ref());

        if !path.exists() {
            return Err(Box::from(format!("File not found: {:?}", path)));
        }

        let file = File::open(&path)?;
        println!("File is good: {:?}", path);

        let mut rdr = csv::Reader::from_reader(file);

        let mut records: Vec<Vec<String>> = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let record_values: Vec<String> = record.iter().map(|field| field.to_string()).collect();
            records.push(record_values);
        }
          csv_deserialize();
        Ok(records)
    }

    fn csv_deserialize() -> Result< Vec<UserModel>, Box<dyn Error>> {
        let file = File::open("misc/MOCK_DATA.csv")?;

        let mut rdr = csv::Reader::from_reader(file);

        let mut list_users : Vec<UserModel> = Vec::new();

        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic deserialization.
            let record: UserModel = result?;

            list_users.push(record);
        }

        println!("count of all users: {:?}",list_users.len());

        Ok(list_users)
    }

    pub fn csv_printer() {
        // Create a CSV parser that reads data from stdin.
        // let file = File::open("MOCK_DATA.csv");
        let mut rdr = csv::Reader::from_reader(io::stdin());
        // Loop over each record.
        for result in rdr.records() {
            // An error may occur, so abort the program in an unfriendly way.
            // We will make this more friendly later!
            let record = result.expect("a CSV record");
            // Print a debug version of the record.
            println!("{:?}", record);
        }
    }
}
