use crate::unzipper::Unzipper;
use anyhow::Result;
use clausewitz_parser::{par_root, root, Val};
use data_model::{Budget, EmpireData, ModelDataPoint, Resources};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    path::PathBuf,
};
use stellarust::dto::{BudgetComponent, ResourceClass};
use strum::IntoEnumIterator;

use super::Key;

pub struct Parser {}

pub struct ParseResult<'a> {
    pub meta: Val<'a>,
    pub gamestate: Val<'a>,
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
        let (meta, gamestate) = Unzipper::get_zipped_content(&path)?;

        let meta = Parser::from_meta(meta.as_str())?;

        let gamestate_prepared = gamestate.replace("\n}\n", "\n}\n#");
        let gamestate = Parser::from_gamestate(gamestate_prepared.as_str())?;

        let result = ParseResult { meta, gamestate };

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
        let result = par_root(string);
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

    let _required_dlcs = get_required_dlcs_from_meta(meta);
    let campaign_name = get_name_from_meta(meta);

    let empires = get_empires_from_gamestate(gamestate).expect("Parsing Not OK");

    ModelDataPoint {
        campaign_name,
        empires,
    }
}

fn get_name_from_meta(meta: &Val<'_>) -> String {
    String::from(get_string_contents(
        get_val_from_path(PathBuf::from("name"), meta).unwrap(),
    ))
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

fn get_empires_from_gamestate(gamestate: &Val<'_>) -> Result<Vec<EmpireData>> {
    let country_list = get_array_contents(get_val_from_path(PathBuf::from("country"), gamestate)?);

    Ok(country_list
        .into_iter()
        .filter_map(|val| get_empire_data(val).ok())
        .collect())
}

#[derive(Debug, PartialEq)]
pub struct PathParseError {
    err: String,
}

impl Error for PathParseError {}

impl Display for PathParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

fn get_val_from_path<'a>(pathbuf: PathBuf, root: &'a Val<'a>) -> Result<&'a Val<'a>> {
    let mut curr_val = root;
    for path_component in pathbuf.into_iter() {
        if let Ok(array_index) = path_component.to_str().unwrap().parse::<usize>() {
            if let Val::Array(array) = curr_val {
                if array_index < array.len() {
                    curr_val = array.get(array_index).unwrap();
                } else {
                    return Err(anyhow::Error::from(PathParseError {
                        err: format!("Index {} out of bounds({})", array_index, array.len()),
                    }));
                }
            } else {
                return Err(anyhow::Error::from(PathParseError {
                    err: format!("Expected an array"),
                }));
            }
        } else {
            if let Val::Dict(dict) = curr_val {
                let mut found = false;
                for (k, v) in dict.into_iter() {
                    if k == &path_component.to_str().unwrap() {
                        curr_val = v;
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(anyhow::Error::from(PathParseError {
                        err: format!("Key {} not found in dict", path_component.to_str().unwrap()),
                    }));
                }
            } else {
                return Err(anyhow::Error::from(PathParseError {
                    err: format!("Expected a dict"),
                }));
            }
        }
    }

    Ok(curr_val)
}

fn get_resources(economy_module: &Val<'_>) -> Resources {
    let get_resource = |res: ResourceClass| -> f64 {
        if let Ok(val) = get_val_from_path(
            PathBuf::from(format!("resources/{}", res.key())),
            economy_module,
        ) {
            match val {
                Val::Integer(a) => *a as f64,
                Val::Decimal(a) => *a,
                _ => panic!(),
            }
        } else {
            0.0f64
        }
    };

    Resources {
        energy: get_resource(ResourceClass::Energy),
        minerals: get_resource(ResourceClass::Minerals),
        food: get_resource(ResourceClass::Food),
        physics_research: get_resource(ResourceClass::Physics),
        society_research: get_resource(ResourceClass::Society),
        engineering_research: get_resource(ResourceClass::Engineering),
        influence: get_resource(ResourceClass::Influence),
        unity: get_resource(ResourceClass::Unity),
        consumer_goods: get_resource(ResourceClass::ConsumerGoods),
        alloys: get_resource(ResourceClass::Alloys),
        volatile_motes: get_resource(ResourceClass::Motes),
        exotic_gases: get_resource(ResourceClass::Gasses),
        rare_crystals: get_resource(ResourceClass::Crystals),
        sr_living_metal: get_resource(ResourceClass::LivingMetal),
        sr_zro: get_resource(ResourceClass::Zro),
        sr_dark_matter: get_resource(ResourceClass::DarkMatter),
    }
}

fn get_budget(budget: &Val) -> Budget {
    let current_month_dict = get_val_from_path(PathBuf::from("current_month"), &budget).unwrap();
    let last_month_dict = get_val_from_path(PathBuf::from("last_month"), &budget).unwrap();

    let get_budget_val =
        |key: BudgetComponent, val: &Val| -> HashMap<ResourceClass, Vec<(String, f64)>> {
            get_budget_component_map(get_val_from_path(PathBuf::from(key.key()), val).unwrap())
        };

    Budget {
        income: get_budget_val(BudgetComponent::Income, current_month_dict),
        expense: get_budget_val(BudgetComponent::Expenses, current_month_dict),
        balance: get_budget_val(BudgetComponent::Balance, current_month_dict),
        income_last_month: get_budget_val(BudgetComponent::Income, last_month_dict),
        expense_last_month: get_budget_val(BudgetComponent::Expenses, last_month_dict),
        balance_last_month: get_budget_val(BudgetComponent::Balance, last_month_dict),
    }
}

fn get_budget_component_map(component: &Val<'_>) -> HashMap<ResourceClass, Vec<(String, f64)>> {
    if let Val::Dict(sources) = component {
        let map =
            sources
                .into_iter()
                .fold(HashMap::new(), |mut map, (contributor, contributions)| {
                    let contribitions_per_class = get_contributions_per_class(contributions);

                    for (key, amount) in contribitions_per_class.into_iter() {
                        map.entry(key)
                            .or_insert(vec![])
                            .push((String::from(*contributor), amount));
                    }
                    map
                });
        map
    } else {
        panic!()
    }
}

fn get_contributions_per_class(contributions: &Val<'_>) -> Vec<(ResourceClass, f64)> {
    ResourceClass::iter()
        .filter_map(|class| {
            if let Ok(val) = get_val_from_path(PathBuf::from(class.key()), contributions) {
                match val {
                    Val::Decimal(d) => Some((class, *d)),
                    Val::Integer(i) => Some((class, *i as f64)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect()
}

fn get_empire_data(country: &Val<'_>) -> Result<EmpireData> {
    let economy_module =
        get_val_from_path(PathBuf::from("modules/standard_economy_module"), country)?;
    let name = get_val_from_path(PathBuf::from("name"), country)?;
    let budget = get_val_from_path(PathBuf::from("budget"), country)?;

    Ok(EmpireData {
        name: String::from(get_string_contents(name)),
        resources: get_resources(economy_module),
        budget: get_budget(budget),
    })
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
    use std::{collections::HashMap, fs, path::PathBuf};

    use clausewitz_parser::root;
    use data_model::{Budget, Resources};

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
        let empire_string = r###"
        name="Queptilium Remnant"
        budget={
			current_month={
				income={
					none={
					}
					source_1={
						energy=1000
					}
					source_2={
						energy=1000
						minerals=300
					}
				}
				expenses={
					none={
					}
					sink_1={
						energy=500
					}
					sink_2={
						energy=500
						minerals=150
					}
				}
				balance={
					none={
					}
					source_1={
						energy=1000
					}
					source_2={
						energy=1000
						minerals=300
					}
                    sink_1={
						energy=-500
					}
					sink_2={
						energy=-500
						minerals=-150
					}
				}
				extra_income={
					none={
					}
				}
				extra_expenses={
					none={
					}
				}
				extra_balance={
					none={
					}
				}
			}
			last_month={
				income={
					none={
					}
					source_1={
						energy=1000
					}
					source_2={
						energy=1000
						minerals=300
					}
				}
				expenses={
					none={
					}
					sink_1={
						energy=500
					}
					sink_2={
						energy=500
						minerals=150
					}
				}
				balance={
					none={
					}
					source_1={
						energy=1000
					}
					source_2={
						energy=1000
						minerals=300
					}
                    sink_1={
						energy=-500
					}
					sink_2={
						energy=-500
						minerals=-150
					}
				}
				extra_income={
					none={
					}
				}
				extra_expenses={
					none={
					}
				}
				extra_balance={
					none={
					}
				}
			}
		}
        modules={
			standard_economy_module={
				resources={
					energy=11484.2
					minerals=10302.2
					food=1119
					physics_research=3
					engineering_research=9
					influence=503
					unity=245.972
					consumer_goods=96
					alloys=201.6
					volatile_motes=16
					exotic_gases=17.6
					rare_crystals=16
					sr_living_metal=8
					sr_zro=8
					sr_dark_matter=8
				}
			}
		}
        
        "###;

        let (_, parse) = root(empire_string).unwrap();

        let empire = get_empire_data(&parse).unwrap();
        println!("{:#?}", empire);
        assert_eq!(
            empire,
            EmpireData {
                name: String::from("Queptilium Remnant"),
                resources: Resources {
                    energy: 11484.2,
                    minerals: 10302.2,
                    food: 1119.0,
                    physics_research: 3.0,
                    society_research: 0.0,
                    engineering_research: 9.0,
                    influence: 503.0,
                    unity: 245.972,
                    consumer_goods: 96.0,
                    alloys: 201.6,
                    volatile_motes: 16.0,
                    exotic_gases: 17.6,
                    rare_crystals: 16.0,
                    sr_living_metal: 8.0,
                    sr_zro: 8.0,
                    sr_dark_matter: 8.0
                },
                budget: Budget {
                    income: HashMap::new(),
                    expense: HashMap::new(),
                    balance: HashMap::new(),
                    income_last_month: HashMap::new(),
                    expense_last_month: HashMap::new(),
                    balance_last_month: HashMap::new()
                }
            }
        );
    }

    #[test]
    fn get_resources__given_economy_module__returns_all_resources() {
        let module_entry = r###"standard_economy_module={
            resources={
                energy=11484.2
                minerals=10302.2
                food=1119
                physics_research=3
                engineering_research=9
                influence=503
                unity=245.972
                consumer_goods=96
                alloys=201.6
                volatile_motes=16
                exotic_gases=17.6
                rare_crystals=16
                sr_living_metal=8
                sr_zro=8
                sr_dark_matter=8
            }
        }"###;

        let (_, val) = root(module_entry).unwrap();

        if let Val::Dict(entries) = val {
            let (_, economy_module) = entries.into_iter().next().unwrap();
            let resources = get_resources(&economy_module);
            assert_eq!(
                resources,
                Resources {
                    energy: 11484.2,
                    minerals: 10302.2,
                    food: 1119.0,
                    physics_research: 3.0,
                    society_research: 0.0,
                    engineering_research: 9.0,
                    influence: 503.0,
                    unity: 245.972,
                    consumer_goods: 96.0,
                    alloys: 201.6,
                    volatile_motes: 16.0,
                    exotic_gases: 17.6,
                    rare_crystals: 16.0,
                    sr_living_metal: 8.0,
                    sr_zro: 8.0,
                    sr_dark_matter: 8.0
                }
            );
        } else {
            panic!()
        }
    }

    #[test]
    fn get_budget__given_budget__returns_budget() {
        let home = std::env::var("HOME").unwrap();
        let ext = "Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/empire_budget";
        let empire_budget_path = PathBuf::from_iter(vec![home.as_str(), ext]);
        let empire_budget_string = fs::read_to_string(empire_budget_path).unwrap();

        let (_, parse) = root(&empire_budget_string.as_str()).unwrap();

        if let Val::Dict(entries) = parse {
            let (_, budget_dict) = entries.into_iter().next().unwrap();
            let _budget = get_budget(&budget_dict);
        } else {
            panic!()
        }
    }

    #[test]
    fn get_income__given_income__returns_income() {
        let income_entry_text = r###"income={
            none={
            }
            source_A={
                energy=1
                minerals=3
                food=5
                
            }
            source_B={
                energy=8
                minerals=13
                food=21
                
            }
            
        }"###;

        let num_resources = 3;
        let mut map: HashMap<ResourceClass, Vec<(String, f64)>> =
            HashMap::with_capacity(num_resources);
        map.insert(
            ResourceClass::Energy,
            vec![
                (String::from("source_A"), 1.0),
                (String::from("source_B"), 8.0),
            ],
        );
        map.insert(
            ResourceClass::Minerals,
            vec![
                (String::from("source_A"), 3.0),
                (String::from("source_B"), 13.0),
            ],
        );

        map.insert(
            ResourceClass::Food,
            vec![
                (String::from("source_A"), 5.0),
                (String::from("source_B"), 21.0),
            ],
        );

        let (_, val) = root(income_entry_text).unwrap();
        let income = if let Val::Dict(kv) = val {
            kv.into_iter().next().unwrap().1
        } else {
            panic!()
        };

        let income = get_budget_component_map(&income);
        assert_eq!(income, map);
    }

    #[test]
    fn get_expenses__given_income__returns_income() {
        let income_entry_text = r###"income={
            none={
            }
            source_A={
                energy=1
                minerals=3
                food=5
                
            }
            source_B={
                energy=8
                minerals=13
                food=21
                
            }
            
        }"###;

        let num_resources = 3;
        let mut map: HashMap<ResourceClass, Vec<(String, f64)>> =
            HashMap::with_capacity(num_resources);
        map.insert(
            ResourceClass::Energy,
            vec![
                (String::from("source_A"), 1.0),
                (String::from("source_B"), 8.0),
            ],
        );
        map.insert(
            ResourceClass::Minerals,
            vec![
                (String::from("source_A"), 3.0),
                (String::from("source_B"), 13.0),
            ],
        );

        map.insert(
            ResourceClass::Food,
            vec![
                (String::from("source_A"), 5.0),
                (String::from("source_B"), 21.0),
            ],
        );

        let (_, val) = root(income_entry_text).unwrap();
        let income = if let Val::Dict(kv) = val {
            kv.into_iter().next().unwrap().1
        } else {
            panic!()
        };

        let income = get_budget_component_map(&income);
        assert_eq!(income, map);
    }

    #[test]
    fn get_name_from_meta__contains_keyvalue_name__returns_value() {
        let text = "name=\"Eat My Shorts\"\n";

        let (_, dict) = root(text).unwrap();

        let name = get_name_from_meta(&dict);

        assert_eq!(name, "Eat My Shorts");
    }
}
