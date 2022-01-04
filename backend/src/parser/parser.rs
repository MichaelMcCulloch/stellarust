use crate::unzipper::Unzipper;
use anyhow::Result;
use clausewitz_parser::clausewitz::root::root;
use clausewitz_parser::clausewitz::Val;
use data_model::{EmpireData, ModelDataPoint};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;

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
    fn from(result: ParseResult<'_>) -> Self {
        let meta = result.meta;
        let gamestate = result.gamestate;

        let required_dlcs = get_required_dlcs_from_meta(&meta);

        let empire_list = get_empires_from_gamestate(gamestate);

        todo!()
    }
}

fn get_required_dlcs_from_meta(meta: &Val) -> Vec<String> {
    if let Val::Dict(pairs) = meta {
        let required_dlcs = pairs
            .into_iter()
            .filter_map(|(k, v)| if *k == "required_dlcs" { Some(v) } else { None })
            .next()
            .unwrap();
        if let Val::Set(dlc_list) = required_dlcs {
            return dlc_list
                .into_iter()
                .filter_map(|val| {
                    if let Val::StringLiteral(string_litteral) = val {
                        Some(String::from(*string_litteral))
                    } else {
                        None
                    }
                })
                .collect();
        } else {
            unimplemented!("This should be an array")
        }
    } else {
        unimplemented!("This should be an array")
    }
}

fn get_empires_from_gamestate(gamestate: Val) -> Vec<EmpireData> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use clausewitz_parser::clausewitz::root::root;

    use super::get_required_dlcs_from_meta;

    #[test]
    fn get_required_dlcs_from_meta__meta__returns_dlc_list() {
        let home = std::env::var("HOME").unwrap();
        let ext = "Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/meta";
        let meta_path = PathBuf::from_iter(vec![home.as_str(), ext]);
        let meta_string = fs::read_to_string(meta_path).unwrap();

        let (_, parse) = root(&meta_string.as_str()).unwrap();

        let dlcs = get_required_dlcs_from_meta(&parse);

        assert_eq!(
            dlcs,
            vec![
                "Ancient Relics Story Pack",
                "Anniversary Portraits",
                "Apocalypse",
                "Distant Stars Story Pack",
                "Federations",
                "Horizon Signal",
                "Humanoids Species Pack",
                "Leviathans Story Pack",
                "Lithoids Species Pack",
                "Megacorp",
                "Necroids Species Pack",
                "Nemesis",
                "Plantoids Species Pack",
                "Synthetic Dawn Story Pack",
                "Utopia"
            ]
        );
    }
}
