use std::path::PathBuf;

use std::fs;

use crate::unzipper::Unzipper;

use super::data::ModelDataPoint;

pub struct Parser {}

impl Parser {
    pub fn from_file(path: &PathBuf) -> ModelDataPoint {
        let (meta, gamestate) = Unzipper::get_zipped_content(&path).unwrap();

        Parser::from_meta(meta.as_str());
        Parser::from_gamestate(gamestate.as_str());

        ModelDataPoint { data: 0 }
    }
    pub fn from_meta(string: &str) -> ModelDataPoint {
        // let i: usize = string.parse().unwrap();
        ModelDataPoint { data: 0 }
    }
    pub fn from_gamestate(string: &str) -> ModelDataPoint {
        // let i: usize = string.parse().unwrap();
        ModelDataPoint { data: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {}
}
