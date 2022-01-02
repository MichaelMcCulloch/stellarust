use crate::unzipper::Unzipper;
use anyhow::Result;
use clausewitz_parser::clausewitz::root::root;
use clausewitz_parser::clausewitz::Val;
use data_model::ModelDataPoint;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::{any, fmt};

pub struct Parser {}

pub struct ParseResult<'a> {
    meta: Val<'a>,
    gamestate: Val<'a>,
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    err: String,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Parser {
    pub fn from_file(path: &PathBuf) -> Result<ModelDataPoint> {
        log::info!(
            "file received: {}",
            path.file_name().unwrap().to_str().unwrap()
        );

        let (meta, gamestate) = Unzipper::get_zipped_content(&path)?;

        log::info!("unzipped");
        let meta = Parser::from_meta(meta.as_str())?;
        let gamestate = Parser::from_gamestate(gamestate.as_str())?;

        let result = ParseResult { meta, gamestate };

        log::info!("parsed");
        Ok(ModelDataPoint::from(result))
    }
    pub fn from_meta<'a>(string: &'a str) -> Result<Val<'a>> {
        let result = root(string);
        match result {
            Ok((_, val)) => Ok(val),
            Err(e) => Err(anyhow::Error::from(ParseError {
                err: format!("Error parsing meta \n{}", e),
            })),
        }
    }
    pub fn from_gamestate<'a>(string: &'a str) -> Result<Val<'a>> {
        let result = root(string);
        match result {
            Ok((_, val)) => Ok(val),
            Err(e) => Err(anyhow::Error::from(ParseError {
                err: format!("Error parsing gamestate:\n{}", e),
            })),
        }
    }
}

impl From<ParseResult<'_>> for ModelDataPoint {
    fn from(_: ParseResult<'_>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_() {}
}
