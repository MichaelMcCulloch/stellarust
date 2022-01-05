use crate::unzipper::Unzipper;
use anyhow::Result;
use clausewitz_parser::clausewitz::root::root;
use clausewitz_parser::clausewitz::Val;
use data_model::{EmpireData, ModelDataPoint};
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
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
        block_on(data_point_from_parse_result(&result))
    }
}

async fn data_point_from_parse_result(result: &ParseResult<'_>) -> ModelDataPoint {
    let meta = &result.meta;
    let gamestate = &result.gamestate;

    let required_dlcs = get_required_dlcs_from_meta(meta);

    let empire_list = get_empires_from_gamestate(gamestate);

    ModelDataPoint {
        empires: block_on(empire_list),
    }
}

async fn get_required_dlcs_from_meta(meta: &Val<'_>) -> Vec<String> {
    let required_dlcs = if let Val::Dict(pairs) = meta {
        pairs
            .into_iter()
            .filter_map(|(k, v)| if *k == "required_dlcs" { Some(v) } else { None })
            .next()
            .unwrap()
    } else {
        unimplemented!("This should be a dict");
    };
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
        unimplemented!("This should be a set")
    }
}

async fn get_empires_from_gamestate(gamestate: &Val<'_>) -> Vec<EmpireData> {
    let key_value_pairs = if let Val::Dict(kv) = gamestate {
        kv
    } else {
        unimplemented!("This should be a dict");
    };

    let array_countries = dict_get(key_value_pairs, "country");

    let country_list = if let Val::Array(country_list) = array_countries {
        country_list
    } else {
        unimplemented!("This should be a set")
    };

    let x: Vec<_> = country_list
        .into_iter()
        .map(|val| get_empire_data(val))
        .collect();

    join_all(x).await
}

async fn get_empire_data(val: &Val<'_>) -> EmpireData {
    let country_details = if let Val::Dict(pairs) = val {
        pairs
    } else {
        unimplemented!("This should be a dict");
    };

    let string_name = dict_get(country_details, "name");

    let name = if let Val::StringLiteral(name) = string_name {
        name
    } else {
        unimplemented!("This should be a string literal")
    };

    println!("{}", name);

    EmpireData {
        name: String::from(*name),
    }
}

#[inline]
fn dict_get<'a>(dict: &'a Vec<(&'a str, Val<'a>)>, key: &'a str) -> &'a Val<'a> {
    dict.into_iter()
        .filter_map(|(k, v)| if *k == key { Some(v) } else { None })
        .next()
        .expect("This should contain 1 element")
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use clausewitz_parser::clausewitz::root::root;
    use futures::executor::block_on;

    use super::*;

    #[test]
    fn get_required_dlcs_from_meta__meta__returns_dlc_list() {
        let home = std::env::var("HOME").unwrap();
        let ext = "Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/meta";
        let meta_path = PathBuf::from_iter(vec![home.as_str(), ext]);
        let meta_string = fs::read_to_string(meta_path).unwrap();

        let (_, parse) = root(&meta_string.as_str()).unwrap();

        let dlcs = block_on(get_required_dlcs_from_meta(&parse));

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

    #[test]
    fn get_empire_data__valid_country___returns_empire_data_with_name() {
        let home = std::env::var("HOME").unwrap();
        let ext = "Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/empire";
        let meta_path = PathBuf::from_iter(vec![home.as_str(), ext]);
        let meta_string = fs::read_to_string(meta_path).unwrap();

        let (_, parse) = root(&meta_string.as_str()).unwrap();

        let empire = block_on(get_empire_data(&parse));
        assert_eq!(
            empire,
            EmpireData {
                name: String::from("United Nations of Earth")
            }
        );
    }

    #[test]
    fn get_empires_from_gamestate__gamestate_file__returns_list() {
        let home = std::env::var("HOME").unwrap();
        let ext = "Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/gamestate";
        let gamestate_path = PathBuf::from_iter(vec![home.as_str(), ext]);
        let gamestate_string = fs::read_to_string(gamestate_path).unwrap();

        let (_, parse) = root(&gamestate_string.as_str()).unwrap();

        let country_list = block_on(get_empires_from_gamestate(&parse));

        assert_eq!(
            country_list,
            vec![
                EmpireData {
                    name: String::from("United Nations of Earth")
                },
                EmpireData {
                    name: String::from("Yaanari Imperium"),
                },
                EmpireData {
                    name: String::from("Confederation of Jakaro"),
                },
                EmpireData {
                    name: String::from("Scyldari Confederacy"),
                },
                EmpireData {
                    name: String::from("Maloqui Hierarchy"),
                },
                EmpireData {
                    name: String::from("Desstican Monopoly"),
                },
                EmpireData {
                    name: String::from("Techarus Core"),
                },
                EmpireData {
                    name: String::from("United Panaxala Imperium"),
                },
                EmpireData {
                    name: String::from("Republic of Yapathinor"),
                },
                EmpireData {
                    name: String::from("Cormathani Trading Consortium"),
                },
                EmpireData {
                    name: String::from("Vivisandia Guardians"),
                },
                EmpireData {
                    name: String::from("Queptilium Remnant"),
                },
                EmpireData {
                    name: String::from("Mathin Civilization"),
                },
                EmpireData {
                    name: String::from("Placid Leviathans"),
                },
                EmpireData {
                    name: String::from("Placid Leviathans"),
                },
                EmpireData {
                    name: String::from("Tiyanki Space Whale Ancient"),
                },
                EmpireData {
                    name: String::from("Commonwealth of Man"),
                },
                EmpireData {
                    name: String::from("Andigonj Corsairs"),
                },
                EmpireData {
                    name: String::from("Curator Order"),
                },
                EmpireData {
                    name: String::from("Prism"),
                },
                EmpireData {
                    name: String::from("Artisan Troupe"),
                },
                EmpireData {
                    name: String::from("Caravansary Caravan Coalition"),
                },
                EmpireData {
                    name: String::from("The Numistic Order"),
                },
                EmpireData {
                    name: String::from("Racket Industrial Enterprise"),
                },
                EmpireData {
                    name: String::from("XuraCorp"),
                },
                EmpireData {
                    name: String::from("Space Amoeba Gathering"),
                },
                EmpireData {
                    name: String::from("Enigmatic Fortress"),
                },
                EmpireData {
                    name: String::from("Menjeti Freebooters"),
                },
                EmpireData {
                    name: String::from("Riggan Commerce Exchange"),
                },
                EmpireData {
                    name: String::from("Automated Dreadnought"),
                },
                EmpireData {
                    name: String::from("Spaceborne Organics"),
                },
                EmpireData {
                    name: String::from("Mineral Extraction Operation"),
                },
                EmpireData {
                    name: String::from("Armistice Initiative"),
                },
                EmpireData {
                    name: String::from("Tavurite Civilization"),
                },
                EmpireData {
                    name: String::from("Enigmatic Energy"),
                },
                EmpireData {
                    name: String::from("Xu'Lokako Civilization"),
                },
                EmpireData {
                    name: String::from("Sinrath Civilization"),
                },
                EmpireData {
                    name: String::from("Pelisimus Civilization"),
                },
                EmpireData {
                    name: String::from("H'Runi Civilization"),
                },
                EmpireData {
                    name: String::from("Belmacosa Civilization"),
                },
                EmpireData {
                    name: String::from("global_event_country"),
                },
                EmpireData {
                    name: String::from("The Shroud"),
                },
                EmpireData {
                    name: String::from("Creatures of the Shroud"),
                },
                EmpireData {
                    name: String::from("VLUUR"),
                },
            ]
        );
    }
}
