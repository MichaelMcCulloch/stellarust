use std::path::PathBuf;

use std::fs;

use super::data::ModelDataPoint;

pub struct Parser {}

impl Parser {
    pub fn from_file(path: &PathBuf) -> ModelDataPoint {
        Parser::from_string(fs::read_to_string(path).unwrap().as_str())
    }
    pub fn from_string(string: &str) -> ModelDataPoint {
        let i: usize = string.parse().unwrap();
        ModelDataPoint { data: i }
    }
}
