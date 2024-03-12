
    use crate::api::user::user_model::UserModel;
    use std::error::Error;
    use std::fs::{remove_file, File};
    use std::io::{self, Write};
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
        };
        
        Ok(records)
    }

    pub fn csv_deserialize<P: AsRef<Path>>(filename: P) -> Result<Vec<UserModel>, Box<dyn Error>> {
        let file = File::open(filename)?;

        let mut rdr = csv::Reader::from_reader(file);

        let mut list_users: Vec<UserModel> = Vec::new();

        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic deserialization.
            let record: UserModel = result?;

            list_users.push(record);
        }

        println!("count of all users: {:?}", list_users.len());

        Ok(list_users)
    }

    pub fn csv_write<P: AsRef<Path>>(filename: P, data: String) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(filename.as_ref());
        let mut file = File::create(&path)?;

        file.write_all(data.as_bytes())?;

        Ok(())
    }

    pub fn delete_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(filename.as_ref());

        if path.exists() {
            remove_file(&path)?;
        }

        Ok(())
}

