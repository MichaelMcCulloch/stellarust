use nom::{error::VerboseError, IResult};
use std::collections::HashMap;
use time::Date;

type Res<T, S> = IResult<T, S, VerboseError<T>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Val<'a> {
    Dict(Option<HashMap<&'a str, Vec<Val<'a>>>>),
    Array(Option<Vec<Val<'a>>>),
    Set(Option<Vec<Val<'a>>>),
    String(&'a str),
    Date(Date),
    Decimal(f64),
    Integer(i64),
    Identifier(&'a str),
}

pub(self) mod space {
    use super::Res;
    use nom::{bytes::complete::take_while, combinator::verify};

    pub fn opt_space<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        take_while(move |character| " \t\r\n".contains(character))(input)
    }

    pub fn req_space<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        verify(opt_space, |spaces: &str| !spaces.is_empty())(input)
    }
}

pub(self) mod dict_array_set {
    use super::{
        decimal_integer_identifier::{identifier, integer},
        helper::{fold_into_array, fold_into_hashmap, trace},
        space::{opt_space, req_space},
        value::value,
        Res, Val,
    };
    use nom::{
        branch::alt,
        bytes::complete::{take, take_while},
        character::complete::{char, digit1},
        combinator::{cut, map, map_res, recognize, verify},
        error::{context, VerboseError},
        multi::separated_list0,
        sequence::{delimited, preceded, separated_pair},
    };

    fn dict_array_set_inner<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        println!("BEFORE: {}", input);
        let (remainder, maybe_key_number_idenentifier): (&'a str, &'a str) =
            take_while(move |character| !"=}".contains(character))(input)?;

        let (_remainder, next_token) = take(1 as usize)(remainder)?;
        println!(
            "remainder, maybe_key_number_idenentifier: {},{}",
            remainder, maybe_key_number_idenentifier
        );
        println!("_remainder, next_token: {},{}", _remainder, next_token);

        if next_token == "}" {
            println!("Found }}, parsing a set");
            return cut(map(set, |vec| {
                if !vec.is_empty() {
                    Val::Set(Some(vec))
                } else {
                    Val::Set(None)
                }
            }))(input);
        } else if next_token == "=" {
            print!("Found =, ");
            return if integer(maybe_key_number_idenentifier).is_ok() {
                println!("parsing an array");
                cut(map(array, |pairs| {
                    if !pairs.is_empty() {
                        Val::Array(Some(fold_into_array(pairs)))
                    } else {
                        Val::Array(None)
                    }
                }))(input)
            } else {
                println!("parsing a dict");
                cut(map(dict, |pairs| {
                    if !pairs.is_empty() {
                        Val::Dict(Some(fold_into_hashmap(pairs)))
                    } else {
                        Val::Dict(None)
                    }
                }))(input)
            };
        } else {
            println!("AFTER: {}", input);

            println!("{}", next_token);
            panic!()
        };
    }
    pub fn dict_array_set<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        delimited(
            char('{'),
            cut(delimited(opt_space, dict_array_set_inner, opt_space)),
            char('}'),
        )(input)
    }
    pub fn key_value<'a>(input: &'a str) -> Res<&'a str, (&'a str, Val<'a>)> {
        separated_pair(
            preceded(opt_space, identifier),
            cut(preceded(opt_space, char('='))),
            preceded(opt_space, value),
        )(input)
    }

    pub fn number_value<'a>(input: &'a str) -> Res<&'a str, (usize, Val<'a>)> {
        separated_pair(
            preceded(
                opt_space,
                map_res(
                    verify(recognize(digit1), |s: &str| !s.is_empty()),
                    str::parse,
                ),
            ),
            cut(preceded(opt_space, char('='))),
            preceded(opt_space, value),
        )(input)
    }
    pub fn array<'a>(input: &'a str) -> Res<&'a str, Vec<(usize, Val<'a>)>> {
        separated_list0(req_space, number_value)(input)
    }

    pub fn set<'a>(input: &'a str) -> Res<&'a str, Vec<Val<'a>>> {
        separated_list0(req_space, value)(input)
    }

    pub fn dict<'a>(input: &'a str) -> Res<&'a str, Vec<(&str, Val<'a>)>> {
        separated_list0(req_space, key_value)(input)
    }
}

pub(self) mod date_string {
    use super::{helper::map_to_date, Res, Val};
    use nom::{
        branch::alt,
        bytes::complete::take_while,
        character::complete::{char, digit1},
        combinator::{cut, map, map_res, recognize},
        sequence::{delimited, tuple},
    };
    use time::Date;

    pub fn date_string<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        delimited(
            char('\"'),
            cut(alt((map(date, Val::Date), map(string, Val::String)))),
            char('\"'),
        )(input)
    }
    pub fn date<'a>(input: &'a str) -> Res<&'a str, Date> {
        map_res(
            recognize(tuple((digit1, char('.'), digit1, char('.'), digit1))),
            map_to_date,
        )(input)
    }
    pub fn string<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        let reserved = "\"={}";
        take_while(move |character: char| {
            !reserved.contains(character)
                && (character.is_alphanumeric()
                    || character.is_whitespace()
                    || character.is_ascii_punctuation())
        })(input)
    }
}

pub(self) mod value {
    use nom::branch::alt;

    use super::{
        date_string::date_string, decimal_integer_identifier::decimal_integer_identifier,
        dict_array_set::dict_array_set, Res, Val,
    };

    pub fn value<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        alt((dict_array_set, date_string, decimal_integer_identifier))(input)
    }
}

pub(self) mod decimal_integer_identifier {
    use super::{Res, Val};
    use nom::{
        branch::alt,
        bytes::complete::take_while,
        character::complete::{char, digit1},
        combinator::{map, map_res, opt, recognize, verify},
        sequence::tuple,
    };

    pub fn decimal_integer_identifier<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        alt((
            map(decimal, Val::Decimal),
            map(integer, Val::Integer),
            map(identifier, Val::Identifier),
        ))(input)
    }
    pub fn decimal<'a>(input: &'a str) -> Res<&'a str, f64> {
        map_res(
            recognize(tuple((opt(char('-')), digit1, char('.'), digit1))),
            str::parse,
        )(input)
    }
    pub fn integer<'a>(input: &'a str) -> Res<&'a str, i64> {
        map_res(recognize(tuple((opt(char('-')), digit1))), str::parse)(input)
    }
    pub fn identifier<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        verify(
            take_while(move |c: char| c.is_alphabetic() || c == '_'),
            |s: &str| !s.is_empty(),
        )(input)
    }
}

pub(self) mod helper {
    use std::{
        collections::HashMap,
        error::Error,
        fmt::{self, Debug, Display, Formatter},
    };

    use time::{Date, Month};

    use super::{Res, Val};

    #[derive(Debug, Clone, PartialEq)]
    pub struct DateParseError {
        err: String,
    }

    impl Error for DateParseError {}

    impl Display for DateParseError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            Debug::fmt(&self.err, f)
        }
    }

    pub fn trace<'a>(name: &'a str, res: bool, debug: &'a str) {
        println!(
            "{} {} with {}",
            name,
            if res { "SUCCEEDED" } else { "FAILED" },
            debug
        );
    }

    pub fn map_to_date<'a>(s: &'a str) -> anyhow::Result<Date> {
        let parts: Vec<&'a str> = s.split(".").collect();

        let year: i32 = parts
            .get(0)
            .ok_or(DateParseError {
                err: String::from("Too Short"),
            })?
            .parse()?;
        let month = match *parts.get(1).ok_or(DateParseError {
            err: String::from("Too Short"),
        })? {
            "01" => Ok(Month::January),
            "02" => Ok(Month::February),
            "03" => Ok(Month::March),
            "04" => Ok(Month::April),
            "05" => Ok(Month::May),
            "06" => Ok(Month::June),
            "07" => Ok(Month::July),
            "08" => Ok(Month::August),
            "09" => Ok(Month::September),
            "10" => Ok(Month::October),
            "11" => Ok(Month::November),
            "12" => Ok(Month::December),
            _ => Err(DateParseError {
                err: String::from("Months beyond December are not supported, dummy!"),
            }),
        };
        let day: u8 = parts
            .get(2)
            .ok_or(DateParseError {
                err: String::from("Too Short"),
            })?
            .parse()?;
        Ok(Date::from_calendar_date(year, month?, day)?)
    }
    pub fn fold_into_hashmap<'a>(
        tuple_vec: Vec<(&'a str, Val<'a>)>,
    ) -> HashMap<&'a str, Vec<Val<'a>>> {
        tuple_vec
            .into_iter()
            .fold(HashMap::new(), |mut acc, (key, value)| {
                {
                    let entry = acc.entry(key).or_insert(vec![]);
                    entry.push(value)
                }
                acc
            })
    }
    pub fn fold_into_array<'a>(tuple_vec: Vec<(usize, Val<'a>)>) -> Vec<Val<'a>> {
        tuple_vec
            .into_iter()
            .fold(Vec::new(), |mut acc, (index, value)| {
                acc.push(value);
                acc
            })
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(test)]
    mod dict_array_set {
        use crate::parser::save_file::{dict_array_set::dict_array_set, Val};

        #[test]
        fn dict_array_set__array__returns_val_array() {
            let text = r###"{
                0="first"
                1="second"
                2="twird"
            }"###;

            let (remainder, actual) = dict_array_set(text).unwrap();
            assert_eq!(
                actual,
                Val::Array(Some(vec!(
                    Val::String("first"),
                    Val::String("second"),
                    Val::String("twird")
                )))
            );
            assert!(remainder.is_empty())
        }
        #[test]
        fn dict_array_set__dict__returns_val_dict() {
            let text = r###"{
                zero="zeroth"
                one="first"
                two="second"
            }"###;

            let (remainder, actual) = dict_array_set(text).unwrap();
            if let Val::Dict(Some(map)) = actual {
                let zero = map.get("zero").unwrap();
                let one = map.get("one").unwrap();
                let two = map.get("two").unwrap();
                assert_eq!(zero, &vec![Val::String("zeroth")]);
                assert_eq!(one, &vec![Val::String("first")]);
                assert_eq!(two, &vec![Val::String("second")]);
                assert!(remainder.is_empty())
            } else {
                panic!("Expected a vec")
            }
        }
        #[test]
        fn dict_array_set__set__returns_val_set() {
            let text = r###"{
                "first"
                "second"
                "twird"
            }"###;

            let (remainder, actual) = dict_array_set(text).unwrap();
            assert_eq!(
                actual,
                Val::Set(Some(vec!(
                    Val::String("first"),
                    Val::String("second"),
                    Val::String("twird")
                )))
            );
            assert!(remainder.is_empty())
        }

        #[test]
        fn dict_array_set__set_of_numbers__returns_val_set() {
            let text = r###"{
                0
                1
                2
            }"###;

            let (remainder, actual) = dict_array_set(text).unwrap();
            assert_eq!(
                actual,
                Val::Set(Some(vec!(
                    Val::Integer(0),
                    Val::Integer(1),
                    Val::Integer(2)
                )))
            );
            assert!(remainder.is_empty())
        }
    }
    #[cfg(test)]
    mod array {
        use crate::parser::save_file::{dict_array_set::array, Val};

        #[test]
        fn array__array_of_arrays__returns_map_of_key_array() {}
        #[test]
        fn array__array_of_dates__returns_map_of_key_date() {
            let keys = super::helper::get_vec_indexes();
            let dates = super::helper::get_vec_dates();
            let val_dates = super::helper::get_vec_val_dates();

            let input: String = keys
                .iter()
                .zip(dates.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(format!("{}", a).as_str());
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(usize, Val)> = keys
                .iter()
                .zip(val_dates.iter())
                .map(|(a, b)| (*a, b.clone()))
                .collect();

            let (remainder, actual) = array(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn array__array_of_decimals__returns_map_of_key_date() {
            let keys = super::helper::get_vec_indexes();
            let decimals = super::helper::get_vec_decimals();
            let val_decimals = super::helper::get_vec_val_decimals();

            let input: String = keys
                .iter()
                .zip(decimals.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(format!("{}", a).as_str());
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(usize, Val)> = keys
                .iter()
                .zip(val_decimals.iter())
                .map(|(a, b)| (*a, b.clone()))
                .collect();

            let (remainder, actual) = array(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn array__array_of_dicts__returns_map_of_key_dict() {}
        #[test]
        fn array__array_of_identifiers__returns_map_of_key_identifier() {
            let keys = super::helper::get_vec_indexes();
            let identifiers = super::helper::get_vec_identifiers();
            let val_identifiers = super::helper::get_vec_val_identifiers();

            let input: String = keys
                .iter()
                .zip(identifiers.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(format!("{}", a).as_str());
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(usize, Val)> = keys
                .iter()
                .zip(val_identifiers.iter())
                .map(|(a, b)| (*a, b.clone()))
                .collect();

            let (remainder, actual) = array(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn array__array_of_integers__returns_map_of_key_integer() {
            let keys = super::helper::get_vec_indexes();
            let integers = super::helper::get_vec_integers();
            let val_integers = super::helper::get_vec_val_integers();

            let input: String = keys
                .iter()
                .zip(integers.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(format!("{}", a).as_str());
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(usize, Val)> = keys
                .iter()
                .zip(val_integers.iter())
                .map(|(a, b)| (*a, b.clone()))
                .collect();

            let (remainder, actual) = array(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn array__array_of_sets__returns_map_of_key_set() {}
        #[test]
        fn array__array_of_strings__returns_map_of_key_string() {
            let keys = super::helper::get_vec_indexes();
            let strings = super::helper::get_vec_strings();
            let val_strings = super::helper::get_vec_val_strings();

            let input: String = keys
                .iter()
                .zip(strings.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(format!("{}", a).as_str());
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(usize, Val)> = keys
                .iter()
                .zip(val_strings.iter())
                .map(|(a, b)| (*a, b.clone()))
                .collect();

            let (remainder, actual) = array(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn dict__empty_dict__returns_empty_map() {
            let vec_text = "";
            let (remainder, actual) = array(vec_text).unwrap();
            assert!(actual.is_empty());
            assert!(remainder.is_empty());
        }
    }

    #[cfg(test)]
    mod helper {
        use time::{Date, Month};

        use crate::parser::save_file::Val;

        pub fn get_vec_indexes() -> Vec<usize> {
            vec![0, 1, 2, 2, 3]
        }

        pub fn get_vec_keys() -> Vec<String> {
            vec!["key_zero", "key_one", "key_two", "key_three", "key_four"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        }

        pub fn get_vec_integers() -> Vec<String> {
            vec!["0", "1", "2", "3", "4"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        }
        pub fn get_vec_decimals() -> Vec<String> {
            vec!["0.5", "1.4", "2.3", "3.2", "4.1"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        }
        pub fn get_vec_identifiers() -> Vec<String> {
            vec!["one", "two", "thr_ee", "four", "five"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        }
        pub fn get_vec_strings() -> Vec<String> {
            vec!["\"one\"", "\"two\"", "\"thr_ee\"", "\"four\"", "\"five\""]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        }
        pub fn get_vec_dates() -> Vec<String> {
            vec![
                "\"2200.01.01\"",
                "\"2200.01.02\"",
                "\"2200.01.03\"",
                "\"2200.01.04\"",
                "\"2200.01.05\"",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
        }

        pub fn get_vec_val_integers() -> Vec<Val<'static>> {
            vec![
                Val::Integer(0),
                Val::Integer(1),
                Val::Integer(2),
                Val::Integer(3),
                Val::Integer(4),
            ]
        }
        pub fn get_vec_val_decimals() -> Vec<Val<'static>> {
            vec![
                Val::Decimal(0.5),
                Val::Decimal(1.4),
                Val::Decimal(2.3),
                Val::Decimal(3.2),
                Val::Decimal(4.1),
            ]
        }
        pub fn get_vec_val_identifiers() -> Vec<Val<'static>> {
            vec![
                Val::Identifier("one"),
                Val::Identifier("two"),
                Val::Identifier("thr_ee"),
                Val::Identifier("four"),
                Val::Identifier("five"),
            ]
        }
        pub fn get_vec_val_strings() -> Vec<Val<'static>> {
            vec![
                Val::String("one"),
                Val::String("two"),
                Val::String("thr_ee"),
                Val::String("four"),
                Val::String("five"),
            ]
        }
        pub fn get_vec_val_dates() -> Vec<Val<'static>> {
            vec![
                Val::Date(Date::from_calendar_date(2200, Month::January, 1).unwrap()),
                Val::Date(Date::from_calendar_date(2200, Month::January, 2).unwrap()),
                Val::Date(Date::from_calendar_date(2200, Month::January, 3).unwrap()),
                Val::Date(Date::from_calendar_date(2200, Month::January, 4).unwrap()),
                Val::Date(Date::from_calendar_date(2200, Month::January, 5).unwrap()),
            ]
        }
    }

    #[cfg(test)]
    mod set {
        use crate::parser::save_file::dict_array_set::set;

        #[test]
        fn set__set_of_arrays__returns_vec_of_arrays() {}
        #[test]
        fn set__set_of_dicts__returns_vec_of_dicts() {}
        #[test]
        fn set__set_of_sets__returns_vec_of_sets() {}
        #[test]
        fn set__set_of_dates__returns_vec_of_dates() {
            let vec_text = super::helper::get_vec_dates();
            let text = vec_text.join("\n");
            let (remainder, actual) = set(text.as_str()).unwrap();
            assert_eq!(actual, super::helper::get_vec_val_dates());
        }
        #[test]
        fn set__set_of_decimals__returns_vec_of_decimals() {
            let vec_text = super::helper::get_vec_decimals();
            let text = vec_text.join("\n");
            let (remainder, actual) = set(text.as_str()).unwrap();
            assert_eq!(actual, super::helper::get_vec_val_decimals());
        }

        #[test]
        fn set__set_of_identifiers__returns_vec_of_identifiers() {
            let vec_text = super::helper::get_vec_identifiers();
            let text = vec_text.join("\n");
            let (remainder, actual) = set(text.as_str()).unwrap();
            assert_eq!(actual, super::helper::get_vec_val_identifiers());
        }
        #[test]
        fn set__set_of_integers__returns_vec_of_integers() {
            let vec_text = super::helper::get_vec_integers();
            let text = vec_text.join("\n");
            let (remainder, actual) = set(text.as_str()).unwrap();
            assert_eq!(actual, super::helper::get_vec_val_integers());
        }

        #[test]
        fn set__set_of_strings__returns_vec_of_strings() {
            let vec_text = super::helper::get_vec_strings();
            let text = vec_text.join("\n");
            let (remainder, actual) = set(text.as_str()).unwrap();
            assert_eq!(actual, super::helper::get_vec_val_strings());
        }
        #[test]
        fn set__empty_set__returns_empty_vec() {
            let vec_text = "";
            let (remainder, actual) = set(vec_text).unwrap();
            assert!(actual.is_empty());
        }
    }

    #[cfg(test)]
    mod dicts {
        use crate::parser::save_file::{dict_array_set::dict, Val};

        #[test]
        fn dict__dict_of_arrays__returns_map_of_key_array() {}
        #[test]
        fn dict__dict_of_dates__returns_map_of_key_date() {
            let keys = super::helper::get_vec_keys();
            let dates = super::helper::get_vec_dates();
            let val_dates = super::helper::get_vec_val_dates();

            let input: String = keys
                .iter()
                .zip(dates.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(a);
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(&str, Val)> = keys
                .iter()
                .zip(val_dates.iter())
                .map(|(a, b)| (a.as_str(), b.clone()))
                .collect();

            let (remainder, actual) = dict(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn dict__dict_of_decimals__returns_map_of_key_date() {
            let keys = super::helper::get_vec_keys();
            let decimals = super::helper::get_vec_decimals();
            let val_decimals = super::helper::get_vec_val_decimals();

            let input: String = keys
                .iter()
                .zip(decimals.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(a);
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(&str, Val)> = keys
                .iter()
                .zip(val_decimals.iter())
                .map(|(a, b)| (a.as_str(), b.clone()))
                .collect();

            let (remainder, actual) = dict(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn dict__dict_of_dicts__returns_map_of_key_dict() {}
        #[test]
        fn dict__dict_of_identifiers__returns_map_of_key_identifier() {
            let keys = super::helper::get_vec_keys();
            let identifiers = super::helper::get_vec_identifiers();
            let val_identifiers = super::helper::get_vec_val_identifiers();

            let input: String = keys
                .iter()
                .zip(identifiers.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(a);
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(&str, Val)> = keys
                .iter()
                .zip(val_identifiers.iter())
                .map(|(a, b)| (a.as_str(), b.clone()))
                .collect();

            let (remainder, actual) = dict(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn dict__dict_of_integers__returns_map_of_key_integer() {
            let keys = super::helper::get_vec_keys();
            let integers = super::helper::get_vec_integers();
            let val_integers = super::helper::get_vec_val_integers();

            let input: String = keys
                .iter()
                .zip(integers.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(a);
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(&str, Val)> = keys
                .iter()
                .zip(val_integers.iter())
                .map(|(a, b)| (a.as_str(), b.clone()))
                .collect();

            let (remainder, actual) = dict(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn dict__dict_of_sets__returns_map_of_key_set() {}
        #[test]
        fn dict__dict_of_strings__returns_map_of_key_string() {
            let keys = super::helper::get_vec_keys();
            let strings = super::helper::get_vec_strings();
            let val_strings = super::helper::get_vec_val_strings();

            let input: String = keys
                .iter()
                .zip(strings.iter())
                .map(|(a, b)| {
                    let mut s = String::new();
                    s.push_str(a);
                    s.push('=');
                    s.push_str(b);
                    s
                })
                .collect::<Vec<String>>()
                .join("\n");

            let expected: Vec<(&str, Val)> = keys
                .iter()
                .zip(val_strings.iter())
                .map(|(a, b)| (a.as_str(), b.clone()))
                .collect();

            let (remainder, actual) = dict(input.as_str()).unwrap();

            assert_eq!(actual, expected);
            assert!(remainder.is_empty());
        }
        #[test]
        fn dict__empty_dict__returns_empty_map() {
            let vec_text = "";
            let (remainder, actual) = dict(vec_text).unwrap();
            assert!(actual.is_empty());
            assert!(remainder.is_empty());
        }
    }

    mod date_string {
        use time::{Date, Month};

        use crate::parser::save_file::{
            date_string::{date, date_string, string},
            Val,
        };

        #[test]
        fn date_string__escaped_date__returns_val_date() {
            let text = "\"2200.01.01\"";
            let (remainder, actual) = date_string(text).unwrap();
            assert_eq!(
                actual,
                Val::Date(Date::from_calendar_date(2200, Month::January, 1).unwrap())
            );
            assert!(remainder.is_empty());
        }
        #[test]
        fn date_string__escaped_string__returns_val_string() {
            let text = "\"This is a string! It has a few \'special\' characters\"";
            let (remainder, actual) = date_string(text).unwrap();
            assert_eq!(
                actual,
                Val::String("This is a string! It has a few \'special\' characters")
            );
            assert!(remainder.is_empty());
        }

        #[test]
        fn date__unquoted_four_dot_two_dot_two__returns_date() {
            let text = "2200.01.01";
            let (remainder, actual) = date(text).unwrap();
            assert_eq!(
                actual,
                Date::from_calendar_date(2200, Month::January, 1).unwrap()
            );
            assert!(remainder.is_empty());
        }

        #[test]
        fn date__unquoted_string__rejected() {
            let text = "a string";
            let actual = date(text);
            assert!(actual.is_err())
        }
        #[test]
        fn string__unquoted_string_with_allowed_special_characters__returns_str() {
            let text = "This is a string! It has a few \'special\' characters";
            let (remainder, actual) = string(text).unwrap();
            assert_eq!(actual, text);
            assert!(remainder.is_empty());
        }
    }

    #[cfg(test)]
    mod decimal_integer_identifier {
        use crate::parser::save_file::{
            decimal_integer_identifier::decimal_integer_identifier, Val,
        };

        #[test]
        fn decimal_integer_identifier__decimal__returns_val_decimal() {
            let text = "0.5";
            let (remainder, actual) = decimal_integer_identifier(text).unwrap();
            assert_eq!(actual, Val::Decimal(0.5));
            assert!(remainder.is_empty());
        }
        #[test]
        fn decimal_integer_identifier__identifier__returns_val_identifier() {
            let text = "wor_ds";
            let (remainder, actual) = decimal_integer_identifier(text).unwrap();
            assert_eq!(actual, Val::Identifier("wor_ds"));
            assert!(remainder.is_empty());
        }
        #[test]
        fn decimal_integer_identifier__integer__returns_val_integer() {
            let text = "5";
            let (remainder, actual) = decimal_integer_identifier(text).unwrap();
            assert_eq!(actual, Val::Integer(5));
            assert!(remainder.is_empty());
        }
    }

    mod identifiers {
        use crate::parser::save_file::decimal_integer_identifier::identifier;

        #[test]
        fn identifier__string_with_underscores__returns_str() {
            let text = "raw_string";
            let (remainder, actual) = identifier(text).unwrap();
            assert_eq!(actual, "raw_string");
            assert!(remainder.is_empty());
        }
        #[test]
        fn identifier__begins_with_number__rejected() {
            let text = "0raw_s0tring";
            let result = identifier(text);
            assert!(result.is_err());
        }
        #[test]
        fn identifier__begins_with_quote__rejected() {
            let text = "\"0raw_s0tring\"";
            let result = identifier(text);
            assert!(result.is_err());
        }
    }

    #[cfg(test)]
    mod integers {
        use crate::parser::save_file::decimal_integer_identifier::integer;

        #[test]
        fn integer__negative__returns_i64() {
            let text = "-7098234322345";
            let (remainder, actual) = integer(text).unwrap();
            assert_eq!(actual, -7098234322345);
            assert!(remainder.is_empty())
        }
        #[test]
        fn integer__positive__returns_i64() {
            let text = "456768978";
            let (remainder, actual) = integer(text).unwrap();
            assert_eq!(actual, 456768978);
            assert!(remainder.is_empty())
        }
        #[test]
        fn integer__zero__returns_i64() {
            let text = "0";
            let (remainder, actual) = integer(text).unwrap();
            assert_eq!(actual, 0);
            assert!(remainder.is_empty())
        }
    }

    #[cfg(test)]
    mod decimals {
        use crate::parser::save_file::decimal_integer_identifier::decimal;

        #[test]
        fn decimal__lt_one__returns_f64() {
            let text = "0.00000000001";
            let (remainder, actual) = decimal(text).unwrap();
            assert_eq!(actual, 0.00000000001);
            assert!(remainder.is_empty())
        }
        #[test]
        fn decimal__negative__returns_f64() {
            let text = "-9098908709.987";
            let (remainder, actual) = decimal(text).unwrap();
            assert_eq!(actual, -9098908709.987);
            assert!(remainder.is_empty())
        }
        #[test]
        fn decimal__positive__returns_f64() {
            let text = "7.0";
            let (remainder, actual) = decimal(text).unwrap();
            assert_eq!(actual, 7.0);
            assert!(remainder.is_empty())
        }
    }

    #[cfg(test)]
    mod key_value {
        use time::{Date, Month};

        use crate::parser::save_file::{dict_array_set::key_value, Val};

        #[test]
        fn key_value__array__returns_key_val_array() {}
        #[test]
        fn key_value__date__returns_key_val_date() {
            let text = "date=\"2200.01.01\"";
            let (remainder, actual) = key_value(text).unwrap();
            assert_eq!(
                actual,
                (
                    "date",
                    Val::Date(Date::from_calendar_date(2200, Month::January, 01).unwrap())
                )
            );
            assert!(remainder.is_empty());
        }
        #[test]
        fn key_value__decimal__returns_key_val_decimal() {
            let text = "key=-0.5";
            let (remainder, actual) = key_value(text).unwrap();
            assert_eq!(actual, ("key", Val::Decimal(-0.5)));
            assert!(remainder.is_empty());
        }
        #[test]
        fn key_value__dict__returns_key_val_dict() {}

        #[test]
        fn key_value__identifier__returns_key_val_identifier() {
            let text = "key=identif_fire";
            let (remainder, actual) = key_value(text).unwrap();
            assert_eq!(actual, ("key", Val::Identifier("identif_fire")));
            assert!(remainder.is_empty());
        }
        #[test]
        fn key_value__integer__returns_key_val_integer() {
            let text = "key=0";
            let (remainder, actual) = key_value(text).unwrap();
            assert_eq!(actual, ("key", Val::Integer(0)));
            assert!(remainder.is_empty());
        }
        #[test]
        fn key_value__set__returns_key_val_set() {}
        #[test]
        fn key_value__string__returns_key_val_string() {
            let text = "key=\"value\"";
            let (remainder, actual) = key_value(text).unwrap();
            assert_eq!(actual, ("key", Val::String("value")));
            assert!(remainder.is_empty());
        }
    }

    #[cfg(test)]
    mod number_value {
        use time::{Date, Month};

        use crate::parser::save_file::{dict_array_set::number_value, Val};

        #[test]
        fn number_value__number__array__returns_ordered_vec_arrays() {}
        #[test]
        fn number_value__number__dict__returns_ordered_vec_dicts() {}
        #[test]
        fn number_value__number__set__returns_ordered_vec_sets() {}
        #[test]
        fn number_value__number__date__returns_ordered_vec_dates() {
            let text = "0=\"2200.01.01\"";
            let (remainder, actual) = number_value(text).unwrap();
            assert_eq!(
                actual,
                (
                    0,
                    Val::Date(Date::from_calendar_date(2200, Month::January, 1).unwrap())
                )
            );
            assert!(remainder.is_empty());
        }
        #[test]
        fn number_value__number__decimal__returns_ordered_vec_decimals() {
            let text = "1=0.7";
            let (remainder, actual) = number_value(text).unwrap();
            assert_eq!(actual, (1, Val::Decimal(0.7)));
            assert!(remainder.is_empty());
        }
        #[test]
        fn number_value__number__identifier__returns_ordered_vec_identifiers() {
            let text = "2=identifire";
            let (remainder, actual) = number_value(text).unwrap();
            assert_eq!(actual, (2, Val::Identifier("identifire")));
            assert!(remainder.is_empty());
        }
        #[test]
        fn number_value__number__integer__returns_ordered_vec_integers() {
            let text = "3=1";
            let (remainder, actual) = number_value(text).unwrap();
            assert_eq!(actual, (3, Val::Integer(1)));
            assert!(remainder.is_empty());
        }
        #[test]
        fn number_value__number__string__returns_ordered_vec_strings() {
            let text = "4=\"value\"";
            let (remainder, actual) = number_value(text).unwrap();
            assert_eq!(actual, (4, Val::String("value")));
            assert!(remainder.is_empty());
        }
    }

    #[cfg(test)]
    mod space {

        use crate::parser::save_file::space::{opt_space, req_space};

        #[test]
        fn opt_space__all_the_spaces__returns_them() {
            let spaces = " \t\r\n";
            let (str, actual) = opt_space(spaces).unwrap();
            assert_eq!(actual, spaces);
            assert!(str.is_empty())
        }
        #[test]
        fn opt_space__empty_string__returns_empty() {
            let spaces = "";
            let (str, actual) = opt_space(spaces).unwrap();
            assert!(spaces.is_empty());
            assert!(str.is_empty());
        }
        #[test]
        fn req_space__all_the_spaces__returns_them() {
            let spaces = " \t\r\n";
            let (str, actual) = req_space(spaces).unwrap();
            assert_eq!(actual, spaces);
            assert!(str.is_empty())
        }
        #[test]
        fn req_space__empty_string__fails() {
            let spaces = "";
            let result = req_space(spaces);
            assert!(result.is_err())
        }
    }

    #[cfg(test)]
    mod value {
        #[test]
        fn value() {}
    }

    #[cfg(test)]
    mod root {
        #[test]
        fn root() {}
    }
}
