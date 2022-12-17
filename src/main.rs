extern crate csv;
extern crate serde;
extern crate serde_json;

use csv::Reader;
use serde_json::json;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CSV_FILE_NAME: &str = "main.csv";
const JSON_FILE_NAME: &str = "main.json";

fn get_root_dir() -> String {
    let root_dir: PathBuf = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            println!("Error getting current directory: {}", e);
            std::process::exit(1);
        }
    };

    return root_dir.to_str().unwrap().to_string();
}

fn create_json_file() {
    let json = json!({});
    let s = serde_json::to_string(&json).unwrap();

    fs::write(JSON_FILE_NAME, s).unwrap();
}

fn convert_string_to_json(csv_string: String) -> Result<(), Box<dyn Error>> {
    // Convert the string to a JSON value.
    let json_value = serde_json::from_str(&csv_string)?;
    println!("{}", csv_string);

    // Return the JSON value.
    Ok(json_value)
}

fn parse_csv_file() -> Result<String, Box<dyn Error>> {
    // Read the CSV file.
    let mut csv_reader = Reader::from_path(get_root_dir() + CSV_FILE_NAME)?;
    let mut csv_string = String::new();

    // Iterate over each record (row). The iterator yields `StringRecord`s.
    for result in csv_reader.records() {
        let record = result?;
        csv_string.extend(
            record
                .iter()
                .map(|s| String::from_utf8(s.as_bytes().to_vec()).unwrap()),
        );
        csv_string.push('\n');
    }

    Ok(csv_string)

    // Convert the string to a JSON value
    // let json_value = convert_string_to_json(csv_string)?;
    // Ok(json_value)
}

fn main() {
    create_json_file();
    let csv_into_string = parse_csv_file();
    let json_value = convert_string_to_json(csv_into_string.unwrap());
}
