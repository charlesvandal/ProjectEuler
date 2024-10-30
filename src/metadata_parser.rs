use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Problem {
    pub id: u8,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Problems {
    problems: Vec<Problem>,
}

pub fn read_problems_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Problem>, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let json: Problems = serde_json::from_reader(reader).unwrap();

    Ok(json.problems)
}