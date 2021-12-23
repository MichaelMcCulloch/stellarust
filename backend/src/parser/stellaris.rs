use nom::{
    branch::alt,
    bytes::complete::{escaped, is_a, tag_no_case, take_while},
    character::{
        complete::{alphanumeric0, alphanumeric1, anychar, char, digit0, digit1, one_of},
        is_alphanumeric,
    },
    combinator::{cut, map, map_res, opt, recognize, value},
    error::{context, ErrorKind, ParseError, VerboseError},
    multi::separated_list0,
    number::complete::{self, double as parse_double, double, i64 as parse_i64},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    AsChar, IResult, InputTakeAtPosition,
};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    io::BufRead,
};
use time::{Date, Month};

type Res<T, S> = IResult<T, S, VerboseError<T>>;

#[derive(Debug, PartialEq)]
pub enum Val<'a> {
    Boolean(bool),
    String(&'a str),
    Integer(i64),
    Decimal(f64),
    Dict(HashMap<&'a str, Vec<Val<'a>>>),
    Array(Vec<Val<'a>>),
    Set(Vec<Val<'a>>),
    Date(Date),
}

fn space<'a>(i: &'a str) -> Res<&'a str, &'a str> {
    let chars = " \t\r\n";
    context("space", take_while(move |c| chars.contains(c)))(i)
}

fn boolean<'a>(input: &'a str) -> Res<&'a str, bool> {
    let parse_true = value(true, tag_no_case("yes"));
    let parse_false = value(false, tag_no_case("no"));
    alt((parse_true, parse_false))(input)
}

fn integer<'a>(i: &'a str) -> Res<&'a str, i64> {
    context(
        "integer",
        map_res(recognize(tuple((opt(char('-')), digit1))), str::parse),
    )(i)
}

fn decimal<'a>(i: &'a str) -> Res<&'a str, f64> {
    context(
        "decimal",
        map_res(
            recognize(tuple((opt(char('-')), digit1, char('.'), digit1))),
            str::parse,
        ),
    )(i)
}

fn raw_string<'a>(i: &'a str) -> Res<&'a str, &'a str> {
    let chars = "\"=";
    context(
        "raw_string",
        take_while(move |c: char| {
            !chars.contains(c)
                && (c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation())
        }),
    )(i)
}

fn raw_date<'a>(i: &'a str) -> Res<&'a str, Date> {
    context(
        "raw_date",
        map_res(
            recognize(tuple((digit1, char('.'), digit1, char('.'), digit1))),
            map_to_date,
        ),
    )(i)
}

fn quoted_string<'a>(i: &'a str) -> Res<&'a str, &'a str> {
    context(
        "quoted_string",
        preceded(char('\"'), cut(terminated(raw_string, char('\"')))),
    )(i)
}

fn quoted_date<'a>(i: &'a str) -> Res<&'a str, Date> {
    context(
        "quoted_date",
        preceded(char('\"'), cut(terminated(raw_date, char('\"')))),
    )(i)
}

fn key_value<'a>(i: &'a str) -> Res<&'a str, (&'a str, Val<'a>)> {
    context(
        "key_value",
        separated_pair(
            preceded(space, raw_string),
            preceded(space, char('=')),
            preceded(space, val),
        ),
    )(i)
}

fn quoted<'a>(i: &'a str) -> Res<&'a str, Val<'a>> {
    context(
        "quoted_date",
        preceded(
            char('\"'),
            cut(terminated(
                alt((
                    map(raw_date, |d| Val::Date(d)),
                    map(raw_string, |s| Val::String(s)),
                )),
                char('\"'),
            )),
        ),
    )(i)
}

fn array<'a>(i: &'a str) -> Res<&'a str, Val<'a>> {
    todo!()
}
fn dict<'a>(i: &'a str) -> Res<&'a str, Val<'a>> {
    todo!()
}
fn set<'a>(i: &'a str) -> Res<&'a str, Val<'a>> {
    todo!()
}

fn bracketed<'a>(i: &'a str) -> Res<&'a str, Val<'a>> {
    todo!()
}

fn val<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
    context(
        "val",
        preceded(
            space,
            alt((
                // map(dict, AmplValue::Dict),
                // map(array, AmplValue::Array),
                // map(set, AmplValue::Set),
                bracketed,
                map(boolean, Val::Boolean),
                quoted,
                map(decimal, Val::Decimal),
                map(integer, Val::Integer),
            )),
        ),
    )(input)
}
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

fn map_to_date<'a>(s: &'a str) -> anyhow::Result<Date> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn space__captures_all_spaces() {
        let text = " \t\n\r";
        let (_, actual) = space(text).unwrap();
        assert_eq!(actual, text);
    }

    #[test]
    fn raw_string__simple_word__parses_word() {
        let text = "Text";
        let (_, actual) = raw_string(text).unwrap();
        assert_eq!(actual, text);
    }

    #[test]
    fn raw_string__two_words__parses_word() {
        let text = "Text part";
        let (_, actual) = raw_string(text).unwrap();
        assert_eq!(actual, text);
    }

    #[test]
    fn raw_string__two_words_and_numbers__parses_word() {
        let text = "Text part 2";
        let (_, actual) = raw_string(text).unwrap();
        assert_eq!(actual, text);
    }

    #[test]
    fn raw_string__two_words_numbers_and_period__parses_word() {
        let text = "Text part 2.";
        let (_, actual) = raw_string(text).unwrap();
        assert_eq!(actual, text);
    }

    #[test]
    fn raw_string__two_words_numbers_period_and_symbol__parses_word() {
        let text = "Text part 2.~!@#$%^&*()_+`1234567890-[];',./{}|:<>?";
        let (_, actual) = raw_string(text).unwrap();
        assert_eq!(actual, text);
    }

    #[test]
    fn quoted_string__simple_word__parses_word() {
        let text = "\"Text\"";
        let (_, actual) = quoted(text).unwrap();
        assert_eq!(actual, Val::String("Text"));
    }

    #[test]
    fn quoted_string__two_words__parses_word() {
        let text = "\"Text part\"";
        let (_, actual) = quoted(text).unwrap();
        assert_eq!(actual, Val::String("Text part"));
    }

    #[test]
    fn quoted_string__two_words_and_numbers__parses_word() {
        let text = "\"Text part 2\"";
        let (_, actual) = quoted(text).unwrap();
        assert_eq!(actual, Val::String("Text part 2"));
    }

    #[test]
    fn quoted_string__two_words_numbers_and_period__parses_word() {
        let text = "\"Text part 2.\"";
        let (_, actual) = quoted(text).unwrap();
        assert_eq!(actual, Val::String("Text part 2."));
    }

    #[test]
    fn quoted_string__two_words_numbers_period_and_symbol__parses_word() {
        let text = "\"Text part 2.~!@#$%^&*()_+`1234567890-[];',./{}|:<>?\"";
        let (_, actual) = quoted(text).unwrap();
        assert_eq!(
            actual,
            Val::String("Text part 2.~!@#$%^&*()_+`1234567890-[];',./{}|:<>?")
        );
    }

    #[test]
    fn integer__zero__returns_0() {
        let text = "0";
        let (_, actual) = integer(text).unwrap();
        assert_eq!(actual, 0i64);
    }
    #[test]
    fn integer__negative__returns_negative_number() {
        let text = "-1";
        let (_, actual) = integer(text).unwrap();
        assert_eq!(actual, -1i64);
    }

    #[test]
    fn integer__a_really_big_number__returns_number() {
        let text = "1234567";
        let (_, actual) = integer(text).unwrap();
        assert_eq!(actual, 1234567i64);
    }
    #[test]
    fn integer__leading0__returns_number() {
        let text = "01234567";
        let (_, actual) = integer(text).unwrap();
        assert_eq!(actual, 1234567i64);
    }

    #[test]
    fn decimal__zero__returns_0() {
        let text = "0.0";
        let (_, actual) = decimal(text).unwrap();
        assert_eq!(actual, 0.0f64);
    }
    #[test]
    fn decimal__negative__returns_negative_number() {
        let text = "-1.0";
        let (_, actual) = decimal(text).unwrap();
        assert_eq!(actual, -1.0f64);
    }

    #[test]
    fn decimal__a_really_big_number__returns_number() {
        let text = "1234567.0";
        let (_, actual) = decimal(text).unwrap();
        assert_eq!(actual, 1234567.0f64);
    }
    #[test]
    fn decimal__leading0__returns_number() {
        let text = "01234567.0";
        let (_, actual) = decimal(text).unwrap();
        assert_eq!(actual, 1234567.0f64);
    }

    #[test]
    fn key_value__simple_string_assignment__returns_key_value_pair() {
        let text = "name=\"Empire\"";
        let (_, (key, val)) = key_value(text).unwrap();

        assert_eq!(key, "name");
        assert_eq!(val, Val::String("Empire"));
    }

    #[test]
    fn key_value__simple_integer_assignment__returns_key_value_pair() {
        let text = "name=0";
        let (_, (key, val)) = key_value(text).unwrap();

        assert_eq!(key, "name");
        assert_eq!(val, Val::Integer(0));
    }

    #[test]
    fn key_value__simple_decimal_assignment__returns_key_value_pair() {
        let text = "name=-0.7";
        let (_, (key, val)) = key_value(text).unwrap();

        assert_eq!(key, "name");
        assert_eq!(val, Val::Decimal(-0.7));
    }

    #[test]
    fn key_value__simple_date_assignment__returns_key_value_pair() {
        let text = "name=\"2200.01.01\"";
        let (_, (key, val)) = key_value(text).unwrap();

        assert_eq!(key, "name");
        assert_eq!(
            val,
            Val::Date(Date::from_calendar_date(2200, Month::January, 01).unwrap())
        );
    }

    #[test]
    fn key_value__simple_boolean_assignment__returns_key_value_pair() {
        let text = "name=no";
        let (_, (key, val)) = key_value(text).unwrap();

        assert_eq!(key, "name");
        assert_eq!(val, Val::Boolean(false));
    }

    #[test]
    fn raw_date__simple_raw_date__returns_date() {
        let text = "2021.12.23";
        let (_, actual) = raw_date(text).unwrap();
        assert_eq!(
            actual,
            Date::from_calendar_date(2021, Month::December, 23).unwrap()
        );
    }

    #[test]
    fn raw_date__min_values___returns_date() {
        let text = "0000.01.01";
        let (_, actual) = raw_date(text).unwrap();
        assert_eq!(
            actual,
            Date::from_calendar_date(0, Month::January, 01).unwrap()
        );
    }

    #[test]
    fn raw_date__max_values___returns_date() {
        let text = "9999.12.31";
        let (_, actual) = raw_date(text).unwrap();
        assert_eq!(
            actual,
            Date::from_calendar_date(9999, Month::December, 31).unwrap()
        );
    }

    #[test]
    fn quoted_date__simple_quoted_date__returns_date() {
        let text = "\"2021.12.23\"";
        let (_, actual) = quoted_date(text).unwrap();
        assert_eq!(
            actual,
            Date::from_calendar_date(2021, Month::December, 23).unwrap()
        );
    }

    #[test]
    fn quoted_date__min_values___returns_date() {
        let text = "\"0000.01.01\"";
        let (_, actual) = quoted_date(text).unwrap();
        assert_eq!(
            actual,
            Date::from_calendar_date(0, Month::January, 01).unwrap()
        );
    }

    #[test]
    fn quoted_date__max_values___returns_date() {
        let text = "\"9999.12.31\"";
        let (_, actual) = quoted_date(text).unwrap();
        assert_eq!(
            actual,
            Date::from_calendar_date(9999, Month::December, 31).unwrap()
        );
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
}
