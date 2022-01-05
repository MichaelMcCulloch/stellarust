use crate::unzipper::Unzipper;
use anyhow::Result;
use clausewitz_parser::clausewitz::root::root;
use clausewitz_parser::clausewitz::Val;
use data_model::{EmpireData, ModelDataPoint, Resources};
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
        data_point_from_parse_result(&result)
    }
}

fn data_point_from_parse_result(result: &ParseResult<'_>) -> ModelDataPoint {
    let meta = &result.meta;
    let gamestate = &result.gamestate;

    let required_dlcs = get_required_dlcs_from_meta(meta);

    let empire_list = get_empires_from_gamestate(gamestate);

    ModelDataPoint {
        empires: empire_list,
    }
}

fn get_required_dlcs_from_meta(meta: &Val<'_>) -> Vec<String> {
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

fn get_empires_from_gamestate(gamestate: &Val<'_>) -> Vec<EmpireData> {
    let key_value_pairs = get_dict_contents(gamestate);

    let array_countries = dict_get(key_value_pairs, "country").unwrap();

    let country_list = get_array_contents(array_countries);

    country_list
        .into_iter()
        .filter_map(|val| get_empire_data(val))
        .collect()
}

fn get_empire_data(val: &Val<'_>) -> Option<EmpireData> {
    let country_detail = get_dict_contents(val);

    let modules = get_dict_contents(dict_get(country_detail, "modules").unwrap());
    let opt_economy_module = dict_get(modules, "standard_economy_module");
    if let Some(val) = opt_economy_module {
        let economy_module = get_dict_contents(val);
        let resources = get_dict_contents(dict_get(economy_module, "resources").unwrap());

        let energy = get_number_contents(dict_get(resources, "energy").unwrap_or(&Val::Integer(0)));
        let minerals =
            get_number_contents(dict_get(resources, "minerals").unwrap_or(&Val::Integer(0)));
        let food = get_number_contents(dict_get(resources, "food").unwrap_or(&Val::Integer(0)));
        let physics_research = get_number_contents(
            dict_get(resources, "physics_research").unwrap_or(&Val::Integer(0)),
        );
        let society_research = get_number_contents(
            dict_get(resources, "society_research").unwrap_or(&Val::Integer(0)),
        );
        let engineering_research = get_number_contents(
            dict_get(resources, "engineering_research").unwrap_or(&Val::Integer(0)),
        );
        let influence =
            get_number_contents(dict_get(resources, "influence").unwrap_or(&Val::Integer(0)));
        let unity = get_number_contents(dict_get(resources, "unity").unwrap_or(&Val::Integer(0)));
        let consumer_goods =
            get_number_contents(dict_get(resources, "consumer_goods").unwrap_or(&Val::Integer(0)));
        let alloys = get_number_contents(dict_get(resources, "alloys").unwrap_or(&Val::Integer(0)));

        let name = get_string_contents(dict_get(country_detail, "name").unwrap());

        Some(EmpireData {
            name: String::from(name),
            resources: Resources {
                energy,
                minerals,
                food,
                physics_research,
                society_research,
                engineering_research,
                influence,
                unity,
                consumer_goods,
                alloys,
            },
        })
    } else {
        None
    }
}

fn dict_get<'a>(dict: &'a Vec<(&'a str, Val<'a>)>, key: &'a str) -> Option<&'a Val<'a>> {
    dict.into_iter()
        .filter_map(|(k, v)| if *k == key { Some(v) } else { None })
        .next()
}

fn get_dict_contents<'a>(val: &'a Val<'a>) -> &'a Vec<(&'a str, Val<'a>)> {
    if let Val::Dict(kv) = val {
        kv
    } else {
        unimplemented!("This should be a dict");
    }
}

fn get_array_contents<'a>(gamestate: &'a Val<'a>) -> &'a Vec<Val<'a>> {
    if let Val::Array(arr) = gamestate {
        arr
    } else {
        unimplemented!("This should be an array");
    }
}

fn get_string_contents<'a>(gamestate: &'a Val<'a>) -> &'a str {
    if let Val::StringLiteral(str) = gamestate {
        str
    } else {
        unimplemented!("This should be a StringLitteral");
    }
}

fn get_decimal_contents<'a>(gamestate: &'a Val<'a>) -> f64 {
    if let Val::Decimal(dec) = gamestate {
        *dec
    } else {
        unimplemented!("This should be a Decimal");
    }
}

fn get_integer_contents<'a>(gamestate: &'a Val<'a>) -> i64 {
    if let Val::Integer(int) = gamestate {
        *int
    } else {
        unimplemented!("This should be an Integer");
    }
}

fn get_number_contents<'a>(gamestate: &'a Val<'a>) -> f64 {
    match gamestate {
        Val::Decimal(f) => *f,
        Val::Integer(i) => *i as f64,
        _ => unimplemented!("this should be a number"),
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use clausewitz_parser::clausewitz::root::root;
    use data_model::Resources;
    use futures::executor::block_on;

    use super::*;

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

    #[test]
    fn get_empire_data__valid_country___returns_empire_data_with_name() {
        let home = std::env::var("HOME").unwrap();
        let ext = "Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/empire";
        let meta_path = PathBuf::from_iter(vec![home.as_str(), ext]);
        let meta_string = fs::read_to_string(meta_path).unwrap();

        let (_, parse) = root(&meta_string.as_str()).unwrap();

        let empire = get_empire_data(&parse);
        assert_eq!(
            empire,
            Some(EmpireData {
                name: String::from("United Nations of Earth"),
                resources: Resources {
                    energy: 162.17754,
                    minerals: 127.512,
                    food: 224.152,
                    physics_research: 21.056,
                    society_research: 21.056,
                    engineering_research: 24.056,
                    influence: 103.0,
                    unity: 16.594,
                    consumer_goods: 105.659,
                    alloys: 112.8195
                }
            })
        );
    }

    #[test]
    fn get_empires_from_gamestate__gamestate_file__returns_list() {
        let home = std::env::var("HOME").unwrap();
        let ext = "Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/gamestate";
        let gamestate_path = PathBuf::from_iter(vec![home.as_str(), ext]);
        let gamestate_string = fs::read_to_string(gamestate_path).unwrap();

        let expected_empire_list: Vec<_> = vec![
            "United Nations of Earth",
            "Yaanari Imperium",
            "Confederation of Jakaro",
            "Scyldari Confederacy",
            "Maloqui Hierarchy",
            "Desstican Monopoly",
            "Techarus Core",
            "United Panaxala Imperium",
            "Republic of Yapathinor",
            "Cormathani Trading Consortium",
            "Vivisandia Guardians",
            "Queptilium Remnant",
            "Mathin Civilization",
            "Placid Leviathans",
            "Placid Leviathans",
            "Tiyanki Space Whale Ancient",
            "Commonwealth of Man",
            "Andigonj Corsairs",
            "Curator Order",
            "Prism",
            "Artisan Troupe",
            "Caravansary Caravan Coalition",
            "The Numistic Order",
            "Racket Industrial Enterprise",
            "XuraCorp",
            "Space Amoeba Gathering",
            "Enigmatic Fortress",
            "Menjeti Freebooters",
            "Riggan Commerce Exchange",
            "Automated Dreadnought",
            "Spaceborne Organics",
            "Mineral Extraction Operation",
            "Armistice Initiative",
            "Tavurite Civilization",
            "Enigmatic Energy",
            "Xu'Lokako Civilization",
            "Sinrath Civilization",
            "Pelisimus Civilization",
            "H'Runi Civilization",
            "Belmacosa Civilization",
            "global_event_country",
            "The Shroud",
            "Creatures of the Shroud",
            "VLUUR",
        ]
        .into_iter()
        .map(|s| EmpireData {
            name: String::from(s),
            resources: Resources {
                energy: 0.0f64,
                minerals: 0.0f64,
                food: 0.0f64,
                physics_research: 0.0f64,
                society_research: 0.0f64,
                engineering_research: 0.0f64,
                influence: 0f64,
                unity: 0.0f64,
                consumer_goods: 0.0f64,
                alloys: 0.0f64,
            },
        })
        .collect();

        let (_, parse) = root(&gamestate_string.as_str()).unwrap();

        let country_list = get_empires_from_gamestate(&parse);

        println!("{:#?}", country_list);

        // assert_eq!(country_list, expected_empire_list);
    }
}
