pub mod csv_manager {

    use std::io::{self, Read};
    use std::error::Error;
    use std::fs::File;
    use std::path::{Path, PathBuf};
    use std::ptr::null;
    
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
    
        Ok(records)
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
