use std::path::PathBuf;

use crate::unzipper::Unzipper;

use clausewitz_parser::clausewitz::root::root;
use clausewitz_parser::clausewitz::Val;
use model::ModelDataPoint;

pub struct Parser {}

pub struct ParseResult<'a> {
    meta: Val<'a>,
    gamestate: Val<'a>,
}

impl Parser {
    pub fn from_file(path: &PathBuf) -> ModelDataPoint {
        let (meta, gamestate) = Unzipper::get_zipped_content(&path).unwrap();

        let meta = Parser::from_meta(meta.as_str());
        let gamestate = Parser::from_gamestate(gamestate.as_str());

        println!("{:#?}", meta);
        log::info!("{:#?}", meta);
        let result = ParseResult { meta, gamestate };
        ModelDataPoint::from(result)
    }
    pub fn from_meta<'a>(_string: &'a str) -> Val<'a> {
        // let i: usize = string.parse().unwrap();
        let (_, val) = root(_string).unwrap();

        val
    }
    pub fn from_gamestate<'a>(_string: &'a str) -> Val<'a> {
        // let i: usize = string.parse().unwrap();
        let (_, val) = root(_string).unwrap();

        val
    }
}

impl From<ParseResult<'_>> for ModelDataPoint {
    fn from(_: ParseResult<'_>) -> Self {
        ModelDataPoint { data: 0 }
    }
}
