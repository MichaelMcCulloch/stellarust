use nom::{
    branch::alt,
    bytes::complete::{escaped, tag_no_case, take_while},
    character::complete::{alphanumeric0, alphanumeric1, char, digit1, one_of},
    combinator::{cut, map, map_res, opt, recognize, value},
    error::{context, ErrorKind, ParseError, VerboseError},
    multi::separated_list0,
    number::{self, complete::double as parse_double, complete::i64 as parse_i64},
    sequence::{delimited, preceded, separated_pair, terminated},
    AsChar, IResult, InputTakeAtPosition,
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type Res<NEXT_STR, RESULT> = IResult<NEXT_STR, RESULT, VerboseError<NEXT_STR>>;

#[derive(Debug, PartialEq)]
pub enum AmplValue<'a> {
    Boolean(bool),
    String(&'a str),
    Number(f64),
    Dict(HashMap<&'a str, Vec<AmplValue<'a>>>),
    Array(Vec<AmplValue<'a>>),
    Set(Vec<AmplValue<'a>>),
}

fn sp<'a>(i: &'a str) -> Res<&'a str, &'a str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}
fn parse_str<'a>(i: &'a str) -> Res<&'a str, &'a str> {
    escaped(printing_characters, '\\', one_of("\"n\\"))(i)
}
fn printing_characters<T, E>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    E: ParseError<T>,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !char_item.is_alphanumeric()
                && !char_item.is_ascii_punctuation()
                && !(char_item == ' ')
                && !(char_item == '\'')
        },
        ErrorKind::AlphaNumeric,
    )
}

fn boolean<'a>(input: &'a str) -> Res<&'a str, bool> {
    let parse_true = value(true, tag_no_case("yes"));
    let parse_false = value(false, tag_no_case("no"));
    alt((parse_true, parse_false))(input)
}
fn string<'a>(input: &'a str) -> Res<&'a str, &'a str> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(input)
}

fn aampl_value<'a>(input: &'a str) -> Res<&'a str, AmplValue<'a>> {
    preceded(
        sp,
        alt((
            map(dict, AmplValue::Dict),
            map(array, AmplValue::Array),
            map(set, AmplValue::Set),
            map(boolean, AmplValue::Boolean),
            map(string, |s| AmplValue::String(s)),
            map(parse_double, AmplValue::Number),
        )),
    )(input)
}

fn root<'a>(input: &'a str) -> Res<&'a str, HashMap<&'a str, Vec<AmplValue<'a>>>> {
    preceded(
        sp,
        cut(terminated(
            map(separated_list0(sp, key_value), fold_into_hashmap),
            sp,
        )),
    )(input)
}

fn key_value<'a>(input: &'a str) -> Res<&'a str, (&'a str, AmplValue<'a>)> {
    context(
        "key_value",
        separated_pair(
            preceded(sp, parse_str),
            cut(preceded(sp, char('='))),
            preceded(sp, aampl_value),
        ),
    )(input)
}

fn dict<'a>(input: &'a str) -> Res<&'a str, HashMap<&'a str, Vec<AmplValue<'a>>>> {
    preceded(
        char('{'),
        preceded(
            sp,
            cut(terminated(
                map(separated_list0(sp, key_value), fold_into_hashmap),
                preceded(sp, terminated(char('}'), sp)),
            )),
        ),
    )(input)
}

fn number_value<'a>(input: &'a str) -> Res<&'a str, (i64, AmplValue<'a>)> {
    context(
        "number_value",
        separated_pair(
            preceded(sp, map_res(recognize(digit1), str::parse)),
            cut(preceded(sp, char('='))),
            preceded(sp, aampl_value),
        ),
    )(input)
}

fn array<'a>(input: &'a str) -> Res<&'a str, Vec<AmplValue<'a>>> {
    preceded(
        char('{'),
        preceded(
            sp,
            cut(terminated(
                map(separated_list0(sp, number_value), fold_into_array),
                preceded(sp, terminated(char('}'), sp)),
            )),
        ),
    )(input)
}

fn set<'a>(input: &'a str) -> Res<&'a str, Vec<AmplValue<'a>>> {
    preceded(
        char('{'),
        preceded(
            sp,
            cut(terminated(
                map(separated_list0(sp, string), |d| {
                    d.into_iter().map(|a| AmplValue::String(a)).collect()
                }),
                preceded(sp, terminated(char('}'), sp)),
            )),
        ),
    )(input)
}
fn fold_into_hashmap<'a>(
    tuple_vec: Vec<(&'a str, AmplValue<'a>)>,
) -> HashMap<&'a str, Vec<AmplValue<'a>>> {
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

fn fold_into_array<'a>(tuple_vec: Vec<(i64, AmplValue<'a>)>) -> Vec<AmplValue<'a>> {
    tuple_vec
        .into_iter()
        .fold(Vec::new(), |mut acc, (index, value)| {
            acc.push(value);
            acc
        })
}

#[cfg(test)]
mod tests {
    use core::num;

    use super::*;

    #[test]
    fn sp__given_tabs__returns() {
        let text = "\t\t\n";
        let (_, actual) = sp(text).unwrap();
        assert_eq!(actual, "\t\t\n");
    }

    #[test]
    fn parse_str__given_ordinary_string__return_string() {
        let text = "Name";
        let (_, actual) = parse_str(text).unwrap();
        assert_eq!(actual, "Name");
    }

    #[test]
    fn parse_str__single_quotes__return_string_with_single_quotes() {
        let text = "'Name'";
        let (_, actual) = parse_str(text).unwrap();
        assert_eq!(actual, "'Name'");
    }

    #[test]
    fn bool__given_no__returns_false() {
        let text = "no";
        let (_, actual) = boolean(text).unwrap();
        assert_eq!(actual, false);
    }
    #[test]
    fn bool__given_yes__returns_true() {
        let text = "yes";
        let (_, actual) = boolean(text).unwrap();
        assert_eq!(actual, true);
    }

    #[test]
    fn string__given_escaped_test__returns_text() {
        let text = "\"text\"";
        let (_, actual) = string(text).unwrap();
        assert_eq!(actual, "text");
    }

    #[test]
    fn string__given_escaped_test_contains_space__returns_text() {
        let text = "\"te xt\"";
        let (_, actual) = string(text).unwrap();
        assert_eq!(actual, "te xt");
    }

    #[test]
    fn key_value__given_assignment__returns_assignment() {
        let text = "name=\"Empire\"";
        let (_, actual) = key_value(text).unwrap();
        assert_eq!(actual, ("name", AmplValue::String("Empire")));
    }

    #[test]
    fn number_value__given_assignment__returns_assignment() {
        let text = "0=\"Empire\"";
        let (_, actual) = number_value(text).unwrap();
        assert_eq!(actual, (0, AmplValue::String("Empire")));
    }

    #[test]
    fn dict__given_assignement__returns_assignment() {
        let text = r###"{
            name="Empire"
        }
        "###;
        let (_, actual) = dict(text).unwrap();

        assert!(actual.contains_key("name"));
        let entry = actual.get("name").unwrap();
        assert_eq!(entry, &vec![AmplValue::String("Empire")]);
    }

    #[test]
    fn dict__given_duplicate_keys__returns_assignment_both_values() {
        let text = r###"{
            name="Empire"
            name="Empire"
        }
        "###;

        let (_, actual) = dict(text).unwrap();

        assert!(actual.contains_key("name"));
        let entry = actual.get("name").unwrap();
        assert_eq!(
            entry,
            &vec![AmplValue::String("Empire"), AmplValue::String("Empire")]
        );
    }

    #[test]
    fn dict__given_different_keys__returns_assignment_both_values() {
        let text = r###"{
            alias="Empire"
            name="Kingdom"
        }
        "###;

        let (_, actual) = dict(text).unwrap();

        assert!(actual.contains_key("name"));
        assert!(actual.contains_key("alias"));

        let name = actual.get("name").unwrap();
        let alias = actual.get("alias").unwrap();

        assert_eq!(name, &vec![AmplValue::String("Kingdom")]);
        assert_eq!(alias, &vec![AmplValue::String("Empire")]);
    }

    #[test]
    fn set__single_element__returns_assignment_both_values() {
        let text = "{\"Ancient Relics Story Pack\"\n}\n";

        let expected_array = vec![AmplValue::String("Ancient Relics Story Pack")];

        let (_, actual) = set(text).unwrap();

        assert_eq!(actual, expected_array);
    }

    #[test]
    fn set__untagged__returns_assignment_both_values() {
        let text = r###"{
            "Ancient Relics Story Pack"
            "Anniversary Portraits"
            "Apocalypse"
            "Distant Stars Story Pack"
        }
        "###;

        let expected_array = vec![
            AmplValue::String("Ancient Relics Story Pack"),
            AmplValue::String("Anniversary Portraits"),
            AmplValue::String("Apocalypse"),
            AmplValue::String("Distant Stars Story Pack"),
        ];

        let (_, actual) = set(text).unwrap();

        assert_eq!(actual, expected_array);
    }

    #[test]
    fn array__numbered__returns_ordered_list() {
        let text = r###"{
            0="toad"
            1="frog"
            2="amphibian"
            3="polliwog"
        }
        "###;

        let expected_array = vec![
            AmplValue::String("toad"),
            AmplValue::String("frog"),
            AmplValue::String("amphibian"),
            AmplValue::String("polliwog"),
        ];

        let (_, actual) = array(text).unwrap();

        assert_eq!(actual, expected_array);
    }

    #[test]
    fn root__one_liner__returns_dict() {
        let real_file = "version=\"Herbert v3.2.2\"\n";

        let (_, actual) = root(real_file).unwrap();
    }
}
