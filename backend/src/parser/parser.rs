use std::path::PathBuf;

use crate::model::ModelDataPoint;
use crate::unzipper::Unzipper;

use clausewitz_parser::clausewitz::root;

pub struct Parser {}

impl Parser {
    pub fn from_file(path: &PathBuf) -> ModelDataPoint {
        let (meta, gamestate) = Unzipper::get_zipped_content(&path).unwrap();

        Parser::from_meta(meta.as_str());
        Parser::from_gamestate(gamestate.as_str());

        ModelDataPoint { data: 0 }
    }
    pub fn from_meta(_string: &str) -> ModelDataPoint {
        // let i: usize = string.parse().unwrap();
        ModelDataPoint { data: 0 }
    }
    pub fn from_gamestate(_string: &str) -> ModelDataPoint {
        // let i: usize = string.parse().unwrap();
        ModelDataPoint { data: 0 }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_string() {}
}
