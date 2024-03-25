use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

use std::fs::File;

use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u128,
    exp: u128,
}

pub fn jwt_sign_verify_main(num_iterations:u32) -> Duration {
    let _args: Vec<String> = env::args().collect();



    // let file_contents =
    //     fs::read_to_string("/Users/mayankc/Work/source/perfComparisons/testdata/emails.json")
    //         .unwrap();


    // let emails: Vec<String> = serde_json::from_str(&file_contents).unwrap();
    let emails: Vec<String> = extract_emails_from_file("misc/Rust_User_3.csv").unwrap();

    let mut i = 1;
    let mut idx = 0;
    let jwt_secret = "qwerty";
    // let jwt_secret = env::var("JWT_SECRET").expect("$JWT_SECRET is not set");
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let mut start_ts = 0;
    let validation = Validation::new(Algorithm::HS256);

    loop {
        if i ==1 {
            start_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
        }
        let email = &emails[idx];
        idx += 1;
        let curr_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
        let my_claims = Claims {
            sub: email.to_string(),
            iat: curr_ts,
            exp: curr_ts + 2 * 60 * 60 * 1000,
        };
        let token = match encode(&Header::default(), &my_claims, &jwt_encoding_key) {
            Ok(t) => t,
            Err(_) => panic!(),
        };
        let token_data = match decode::<Claims>(&token, &jwt_decoding_key, &validation) {
            Ok(c) => c,
            Err(err) => panic!("{}", err.to_string()),
        };
        if token_data.claims.sub != email.to_string() {
            panic!("email didn't match");
        }
        if idx >= emails.len() {
            idx = 0;
        }
        i += 1;
        if i > num_iterations {
            break;
        }
    }

    let end_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let diff = end_ts - start_ts;
    let ten_millis = Duration::from_millis(diff.try_into().unwrap());
    println!("diff in ms {:?}", ten_millis);
    return ten_millis;
}

pub fn extract_emails_from_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Open the file
    let file = File::open(filename)?;

    // Create a vector to store the emails
    let mut emails = Vec::new();

    let mut rdr: csv::Reader<File> = csv::Reader::from_reader(file);

    for line in rdr.records() {
        let line = line?;
        let record_values: Vec<String> = line.iter().map(|field| field.to_string()).collect();

        // Extract emails from the line and add them to the vector
        let email = record_values[4].clone();

        emails.push(email);
    }

    println!("email count =  {:?}",emails.len());
    Ok(emails)
}
