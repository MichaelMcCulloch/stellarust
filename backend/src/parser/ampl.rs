use nom::{
    branch::alt,
    bytes::complete::{escaped, tag_no_case, take_while},
    character::complete::{alphanumeric0, alphanumeric1, char, one_of},
    combinator::{cut, map, value},
    error::{context, VerboseError},
    multi::separated_list0,
    number::{self, complete::double},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::{collections::HashMap, hash::Hash};

type Res<NEXT_STR, RESULT> = IResult<NEXT_STR, RESULT, VerboseError<NEXT_STR>>;

#[derive(Debug, PartialEq)]
pub enum AmplValue<'a> {
    Boolean(bool),
    String(&'a str),
    Number(f64),
    Dict(HashMap<&'a str, Vec<AmplValue<'a>>>),
    Array(Vec<AmplValue<'a>>),
}

fn sp<'a>(i: &'a str) -> Res<&'a str, &'a str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}
fn parse_str<'a>(i: &'a str) -> Res<&'a str, &'a str> {
    escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
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
            // map(array, AamplValue::Array),
            map(boolean, AmplValue::Boolean),
            map(string, |s| AmplValue::String(s)),
            map(double, AmplValue::Number),
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
                map(
                    separated_list0(sp, key_value),
                    |tuple_vec: Vec<(&str, AmplValue)>| fold_into_hashmap(tuple_vec),
                ),
                preceded(sp, char('}')),
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
    fn test_parse_str__given_ordinary_string__return_string() {
        let text = "Name";
        let (_, actual) = parse_str(text).unwrap();
        assert_eq!(actual, "Name");
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
    fn key_value__given_assignment__returns_assignment() {
        let text = "name=\"Empire\"";
        let (_, actual) = key_value(text).unwrap();
        assert_eq!(actual, ("name", AmplValue::String("Empire")));
    }

    #[test]
    fn dict__given_assignement__returns_assignment() {
        let text = "{name=\"Empire\"\n}";
        let (_, actual) = dict(text).unwrap();

        println!("{:?}", actual);
    }
    #[test]
    fn dict__given_duplicate_keys__returns_assignment_both_values() {
        let text = r###"{
	name="Empire"
	name="Empire"
}"###;

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
}"###;

        let (_, actual) = dict(text).unwrap();

        assert!(actual.contains_key("name"));
        assert!(actual.contains_key("alias"));

        let name = actual.get("name").unwrap();
        let alias = actual.get("alias").unwrap();

        assert_eq!(name, &vec![AmplValue::String("Kingdom")]);
        assert_eq!(alias, &vec![AmplValue::String("Empire")]);
    }
}
