use nom::{error::VerboseError, IResult};
use std::collections::HashMap;
use time::Date;

type Res<T, S> = IResult<T, S, VerboseError<T>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Val<'a> {
    Dict(Vec<(&'a str, Val<'a>)>),
    NumberedDict(i64, Vec<(&'a str, Val<'a>)>),
    Array(Vec<Val<'a>>),
    Set(Vec<Val<'a>>),
    StringLiteral(&'a str),
    Date(Date),
    Decimal(f64),
    Integer(i64),
    Identifier(&'a str),
}

pub(self) mod unrolled {
    use std::slice::SliceIndex;

    use nom::{error::ParseError, InputTakeAtPosition};

    use super::Res;

    pub fn take_while_unrolled<'a, Condition, Error: ParseError<&'a str>>(
        cond: Condition,
    ) -> impl Fn(&'a str) -> Res<&'a str, &'a str>
    where
        Condition: Fn(char) -> bool,
    {
        // move |i: &'a str| i.split_at_position_complete(|c| !cond(c))
        move |i: &'a str| take_while_unrolled_prime(i, |c| !cond(c))
    }

    pub fn take_while_unrolled_prime<'a, F>(str: &'a str, condition: F) -> Res<&'a str, &'a str>
    where
        F: Fn(char) -> bool,
    {
        if str.is_empty() {
            return Ok(("", ""));
        }
        let mut i = 0usize;
        let len = str.len();
        let chunk_size = 8;

        loop {
            if len - i < chunk_size {
                break;
            };

            if condition((*str.as_bytes().get(i).unwrap()) as char) {
                break;
            }
            i += 1;
            if condition((*str.as_bytes().get(i).unwrap()) as char) {
                break;
            }
            i += 1;
            if condition((*str.as_bytes().get(i).unwrap()) as char) {
                break;
            }
            i += 1;
            if condition((*str.as_bytes().get(i).unwrap()) as char) {
                break;
            }
            i += 1;

            if condition((*str.as_bytes().get(i).unwrap()) as char) {
                break;
            }
            i += 1;
            if condition((*str.as_bytes().get(i).unwrap()) as char) {
                break;
            }
            i += 1;
            if condition((*str.as_bytes().get(i).unwrap()) as char) {
                break;
            }
        }
        if len - i < chunk_size {
            loop {
                let byte = (*str.as_bytes().get(i).unwrap()) as char;

                if condition(byte) {
                    break;
                }
                i += 1;

                if i == len {
                    break;
                }
            }
        }

        let (before, after) = str.split_at(i);

        Ok((after, before))
    }
}

pub(self) mod tables {

    const fn punctuation_table() -> [bool; 256] {
        let mut table = [false; 256];

        table[b'!' as usize] = true;
        table[b'"' as usize] = true;
        table[b'#' as usize] = true;
        table[b'$' as usize] = true;
        table[b'%' as usize] = true;
        table[b'&' as usize] = true;
        table[b'\'' as usize] = true;
        table[b'(' as usize] = true;
        table[b')' as usize] = true;
        table[b'*' as usize] = true;
        table[b'+' as usize] = true;
        table[b',' as usize] = true;
        table[b'-' as usize] = true;
        table[b'.' as usize] = true;
        table[b'/' as usize] = true;
        table[b':' as usize] = true;
        table[b';' as usize] = true;
        table[b'<' as usize] = true;
        table[b'=' as usize] = true;
        table[b'>' as usize] = true;
        table[b'?' as usize] = true;
        table[b'@' as usize] = true;
        table[b'[' as usize] = true;
        table[b'\\' as usize] = true;
        table[b']' as usize] = true;
        table[b'^' as usize] = true;
        table[b'_' as usize] = true;
        table[b'`' as usize] = true;
        table[b'{' as usize] = true;
        table[b'|' as usize] = true;
        table[b'}' as usize] = true;
        table[b'~' as usize] = true;
        table
    }
    const fn string_litteral_content_table() -> [bool; 256] {
        let mut table = [false; 256];
        table[b'a' as usize] = true;
        table[b'b' as usize] = true;
        table[b'c' as usize] = true;
        table[b'd' as usize] = true;
        table[b'e' as usize] = true;
        table[b'f' as usize] = true;
        table[b'g' as usize] = true;
        table[b'h' as usize] = true;
        table[b'i' as usize] = true;
        table[b'j' as usize] = true;
        table[b'k' as usize] = true;
        table[b'l' as usize] = true;
        table[b'm' as usize] = true;
        table[b'n' as usize] = true;
        table[b'o' as usize] = true;
        table[b'p' as usize] = true;
        table[b'q' as usize] = true;
        table[b'r' as usize] = true;
        table[b's' as usize] = true;
        table[b't' as usize] = true;
        table[b'u' as usize] = true;
        table[b'v' as usize] = true;
        table[b'w' as usize] = true;
        table[b'x' as usize] = true;
        table[b'y' as usize] = true;
        table[b'z' as usize] = true;

        table[b'A' as usize] = true;
        table[b'B' as usize] = true;
        table[b'C' as usize] = true;
        table[b'D' as usize] = true;
        table[b'E' as usize] = true;
        table[b'F' as usize] = true;
        table[b'G' as usize] = true;
        table[b'H' as usize] = true;
        table[b'I' as usize] = true;
        table[b'J' as usize] = true;
        table[b'K' as usize] = true;
        table[b'L' as usize] = true;
        table[b'M' as usize] = true;
        table[b'N' as usize] = true;
        table[b'O' as usize] = true;
        table[b'P' as usize] = true;
        table[b'Q' as usize] = true;
        table[b'R' as usize] = true;
        table[b'S' as usize] = true;
        table[b'T' as usize] = true;
        table[b'U' as usize] = true;
        table[b'V' as usize] = true;
        table[b'W' as usize] = true;
        table[b'X' as usize] = true;
        table[b'Y' as usize] = true;
        table[b'Z' as usize] = true;

        table[b'0' as usize] = true;
        table[b'1' as usize] = true;
        table[b'2' as usize] = true;
        table[b'3' as usize] = true;
        table[b'4' as usize] = true;
        table[b'5' as usize] = true;
        table[b'6' as usize] = true;
        table[b'7' as usize] = true;
        table[b'8' as usize] = true;
        table[b'9' as usize] = true;

        table[b' ' as usize] = true;
        table[b'\n' as usize] = true;
        table[b'\t' as usize] = true;
        table[b'\r' as usize] = true;

        table[b'!' as usize] = true;
        table[b'"' as usize] = true;
        table[b'#' as usize] = true;
        table[b'$' as usize] = true;
        table[b'%' as usize] = true;
        table[b'&' as usize] = true;
        table[b'\'' as usize] = true;
        table[b'(' as usize] = true;
        table[b')' as usize] = true;
        table[b'*' as usize] = true;
        table[b'+' as usize] = true;
        table[b',' as usize] = true;
        table[b'-' as usize] = true;
        table[b'.' as usize] = true;
        table[b'/' as usize] = true;
        table[b':' as usize] = true;
        table[b';' as usize] = true;
        table[b'<' as usize] = true;
        table[b'>' as usize] = true;
        table[b'?' as usize] = true;
        table[b'@' as usize] = true;
        table[b'[' as usize] = true;
        table[b'\\' as usize] = true;
        table[b']' as usize] = true;
        table[b'^' as usize] = true;
        table[b'_' as usize] = true;
        table[b'`' as usize] = true;
        table[b'{' as usize] = true;
        table[b'|' as usize] = true;
        table[b'}' as usize] = true;
        table[b'~' as usize] = true;

        table[b'"' as usize] = false;
        table[b'=' as usize] = false;
        table[b'}' as usize] = false;
        table[b'{' as usize] = false;

        table
    }
    const fn identifier_table() -> [bool; 256] {
        let mut table = [false; 256];
        table[b'a' as usize] = true;
        table[b'b' as usize] = true;
        table[b'c' as usize] = true;
        table[b'd' as usize] = true;
        table[b'e' as usize] = true;
        table[b'f' as usize] = true;
        table[b'g' as usize] = true;
        table[b'h' as usize] = true;
        table[b'i' as usize] = true;
        table[b'j' as usize] = true;
        table[b'k' as usize] = true;
        table[b'l' as usize] = true;
        table[b'm' as usize] = true;
        table[b'n' as usize] = true;
        table[b'o' as usize] = true;
        table[b'p' as usize] = true;
        table[b'q' as usize] = true;
        table[b'r' as usize] = true;
        table[b's' as usize] = true;
        table[b't' as usize] = true;
        table[b'u' as usize] = true;
        table[b'v' as usize] = true;
        table[b'w' as usize] = true;
        table[b'x' as usize] = true;
        table[b'y' as usize] = true;
        table[b'z' as usize] = true;
        table[b'A' as usize] = true;
        table[b'B' as usize] = true;
        table[b'C' as usize] = true;
        table[b'D' as usize] = true;
        table[b'E' as usize] = true;
        table[b'F' as usize] = true;
        table[b'G' as usize] = true;
        table[b'H' as usize] = true;
        table[b'I' as usize] = true;
        table[b'J' as usize] = true;
        table[b'K' as usize] = true;
        table[b'L' as usize] = true;
        table[b'M' as usize] = true;
        table[b'N' as usize] = true;
        table[b'O' as usize] = true;
        table[b'P' as usize] = true;
        table[b'Q' as usize] = true;
        table[b'R' as usize] = true;
        table[b'S' as usize] = true;
        table[b'T' as usize] = true;
        table[b'U' as usize] = true;
        table[b'V' as usize] = true;
        table[b'W' as usize] = true;
        table[b'X' as usize] = true;
        table[b'Y' as usize] = true;
        table[b'Z' as usize] = true;
        table[b'_' as usize] = true;
        table[b'0' as usize] = true;
        table[b'1' as usize] = true;
        table[b'2' as usize] = true;
        table[b'3' as usize] = true;
        table[b'4' as usize] = true;
        table[b'5' as usize] = true;
        table[b'6' as usize] = true;
        table[b'7' as usize] = true;
        table[b'8' as usize] = true;
        table[b'9' as usize] = true;
        table
    }
    const fn alphabet_table() -> [bool; 256] {
        let mut table = [false; 256];
        table[b'a' as usize] = true;
        table[b'b' as usize] = true;
        table[b'c' as usize] = true;
        table[b'd' as usize] = true;
        table[b'e' as usize] = true;
        table[b'f' as usize] = true;
        table[b'g' as usize] = true;
        table[b'h' as usize] = true;
        table[b'i' as usize] = true;
        table[b'j' as usize] = true;
        table[b'k' as usize] = true;
        table[b'l' as usize] = true;
        table[b'm' as usize] = true;
        table[b'n' as usize] = true;
        table[b'o' as usize] = true;
        table[b'p' as usize] = true;
        table[b'q' as usize] = true;
        table[b'r' as usize] = true;
        table[b's' as usize] = true;
        table[b't' as usize] = true;
        table[b'u' as usize] = true;
        table[b'v' as usize] = true;
        table[b'w' as usize] = true;
        table[b'x' as usize] = true;
        table[b'y' as usize] = true;
        table[b'z' as usize] = true;
        table[b'A' as usize] = true;
        table[b'B' as usize] = true;
        table[b'C' as usize] = true;
        table[b'D' as usize] = true;
        table[b'E' as usize] = true;
        table[b'F' as usize] = true;
        table[b'G' as usize] = true;
        table[b'H' as usize] = true;
        table[b'I' as usize] = true;
        table[b'J' as usize] = true;
        table[b'K' as usize] = true;
        table[b'L' as usize] = true;
        table[b'M' as usize] = true;
        table[b'N' as usize] = true;
        table[b'O' as usize] = true;
        table[b'P' as usize] = true;
        table[b'Q' as usize] = true;
        table[b'R' as usize] = true;
        table[b'S' as usize] = true;
        table[b'T' as usize] = true;
        table[b'U' as usize] = true;
        table[b'V' as usize] = true;
        table[b'W' as usize] = true;
        table[b'X' as usize] = true;
        table[b'Y' as usize] = true;
        table[b'Z' as usize] = true;
        table
    }
    const fn reserved_table() -> [bool; 256] {
        let mut table = [false; 256];
        table[b'\"' as usize] = true;
        table[b'=' as usize] = true;
        table[b'}' as usize] = true;
        table[b'{' as usize] = true;
        table
    }
    const fn number_table() -> [bool; 256] {
        let mut table = [false; 256];

        table[b'0' as usize] = true;
        table[b'1' as usize] = true;
        table[b'2' as usize] = true;
        table[b'3' as usize] = true;
        table[b'4' as usize] = true;
        table[b'5' as usize] = true;
        table[b'6' as usize] = true;
        table[b'7' as usize] = true;
        table[b'8' as usize] = true;
        table[b'9' as usize] = true;

        table
    }
    const fn token_table() -> [bool; 256] {
        let mut table = [false; 256];
        table[b'=' as usize] = true;
        table[b'{' as usize] = true;
        table[b'}' as usize] = true;
        table
    }

    pub fn is_string_litteral_contents(char: char) -> bool {
        string_litteral_content_table()[char as usize]
    }
    pub fn is_identifier_char(char: char) -> bool {
        identifier_table()[char as usize]
    }

    pub fn is_alphabetic(char: char) -> bool {
        alphabet_table()[char as usize]
    }

    const fn space_table() -> [bool; 256] {
        let mut table = [false; 256];
        table[b' ' as usize] = true;
        table[b'\n' as usize] = true;
        table[b'\t' as usize] = true;
        table[b'\r' as usize] = true;
        table
    }

    pub fn is_space(c: char) -> bool {
        space_table()[c as usize]
    }

    pub fn is_digit(char: char) -> bool {
        char.is_digit(10)
    }

    pub fn is_token(char: char) -> bool {
        token_table()[char as usize]
    }

    pub fn is_reserved(char: char) -> bool {
        reserved_table()[char as usize]
    }
}

pub(self) mod space {
    use super::{tables::is_space, unrolled::take_while_unrolled, Res};
    use nom::{bytes::complete::take_while, combinator::verify, error::VerboseError};

    pub fn opt_space<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        take_while_unrolled::<'a, _, VerboseError<&'a str>>(move |character| is_space(character))(
            input,
        )
    }

    pub fn req_space<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        verify(opt_space, |spaces: &str| !spaces.is_empty())(input)
    }
}

pub(self) mod decimal {
    use nom::{
        character::complete::{char, digit1},
        combinator::{map, map_res, opt, recognize},
        sequence::tuple,
    };

    use super::{Res, Val};

    pub fn decimal<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(
            map_res(
                recognize(tuple((opt(char('-')), digit1, char('.'), digit1))),
                str::parse,
            ),
            |float: f64| Val::Decimal(float),
        )(input)
    }
}
pub(self) mod integer {
    use nom::{
        character::complete::{char, digit1},
        combinator::{map, map_res, opt, recognize, verify},
        sequence::tuple,
    };

    use super::{Res, Val};

    pub fn int<'a>(input: &'a str) -> Res<&'a str, i64> {
        map_res(
            verify(recognize(tuple((opt(char('-')), digit1))), |s: &str| {
                !s.is_empty()
            }),
            str::parse,
        )(input)
    }

    pub fn integer<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(int, |integer: i64| Val::Integer(integer))(input)
    }
}

pub(self) mod identifier {
    use nom::{
        combinator::{map, verify},
        error::VerboseError,
    };

    use super::{
        tables::{is_digit, is_identifier_char},
        unrolled::take_while_unrolled,
        Res, Val,
    };

    pub fn identifier<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(
            verify(
                take_while_unrolled::<'a, _, VerboseError<&'a str>>(is_identifier_char),
                |s: &str| !s.is_empty() && !(is_digit(s.chars().next().unwrap())),
            ),
            |s: &str| Val::Identifier(s),
        )(input)
    }
}

pub(self) mod unquoted {
    use nom::branch::alt;

    use super::{decimal::decimal, identifier::identifier, integer::integer, Res, Val};

    pub fn unquoted<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        alt((decimal, integer, identifier))(input)
    }
}

pub(self) mod date {
    use std::{
        error::Error,
        fmt::{self, Debug, Display, Formatter},
    };

    use nom::{
        character::complete::{char, digit1},
        combinator::{map, map_res, recognize},
        sequence::tuple,
    };

    use time::{Date, Month};
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
    use super::{Res, Val};

    pub fn date<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(
            map_res(
                recognize(tuple((digit1, char('.'), digit1, char('.'), digit1))),
                map_to_date,
            ),
            |date: Date| Val::Date(date),
        )(input)
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
}

pub(self) mod string_literal {
    use nom::{combinator::map, error::VerboseError};

    use super::{
        tables::{is_reserved, is_string_litteral_contents},
        unrolled::take_while_unrolled,
        Res, Val,
    };

    pub fn string_literal_contents<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        take_while_unrolled::<'a, _, VerboseError<&'a str>>(is_string_litteral_contents)(input)
    }

    pub fn string_literal<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(string_literal_contents, |s: &str| Val::StringLiteral(s))(input)
    }
}

pub(self) mod quoted {
    use nom::{branch::alt, character::complete::char, combinator::cut, sequence::delimited};

    use super::{date::date, string_literal::string_literal, Res, Val};

    pub fn quoted<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        delimited(char('\"'), cut(alt((date, string_literal))), char('\"'))(input)
    }
}

pub(self) mod dict {
    use std::collections::HashMap;

    use nom::{
        branch::alt,
        character::complete::char,
        combinator::{cut, map, verify},
        error::VerboseError,
        multi::separated_list0,
        sequence::{delimited, preceded, separated_pair},
    };

    use super::{
        space::{opt_space, req_space},
        string_literal::string_literal_contents,
        tables::{is_digit, is_identifier_char},
        unrolled::take_while_unrolled,
        value::value,
        Res, Val,
    };

    pub fn unquoted_key<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        verify(
            take_while_unrolled::<'a, _, VerboseError<&'a str>>(is_identifier_char),
            |s: &str| !s.is_empty() && !(is_digit(s.chars().next().unwrap())),
        )(input)
    }

    pub fn quoted_key<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        delimited(char('\"'), string_literal_contents, char('\"'))(input)
    }

    pub fn key<'a>(input: &'a str) -> Res<&'a str, &'a str> {
        alt((unquoted_key, quoted_key))(input)
    }

    pub fn key_value<'a>(input: &'a str) -> Res<&'a str, (&'a str, Val<'a>)> {
        separated_pair(
            preceded(opt_space, key),
            cut(preceded(opt_space, char('='))),
            preceded(opt_space, value),
        )(input)
    }

    pub fn hash_map<'a>(input: &'a str) -> Res<&'a str, Vec<(&'a str, Val<'a>)>> {
        separated_list0(req_space, key_value)(input)
    }

    pub fn dict<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(hash_map, Val::Dict)(input)
    }
}
pub(self) mod array {
    use nom::{
        character::complete::{char, digit1},
        combinator::{cut, map, map_res, recognize, verify},
        multi::separated_list0,
        sequence::{preceded, separated_pair},
    };

    use super::{
        space::{opt_space, req_space},
        value::value,
        Res, Val,
    };

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

    pub fn array<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(
            separated_list0(req_space, number_value),
            |number_value_pairs| Val::Array(fold_into_array(number_value_pairs)),
        )(input)
    }

    pub fn fold_into_array<'a>(mut tuple_vec: Vec<(usize, Val<'a>)>) -> Vec<Val<'a>> {
        tuple_vec.sort_by(|(a_index, _), (b_index, _)| a_index.partial_cmp(b_index).unwrap());
        tuple_vec.into_iter().map(|(_, val)| val).collect()
    }
}
pub(self) mod set {
    use nom::{branch::alt, combinator::map, multi::separated_list0};

    use super::{
        space::{opt_space, req_space},
        value::value,
        Res, Val,
    };

    pub fn set<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        alt((
            map(separated_list0(req_space, value), |s: Vec<Val>| Val::Set(s)),
            map(opt_space, |_s: &str| Val::Set(vec![])),
        ))(input)
    }
}
pub(self) mod bracketed {
    use nom::{
        bytes::complete::take,
        character::complete::char,
        combinator::{cut, map},
        error::VerboseError,
        multi::separated_list0,
        sequence::delimited,
    };

    use super::{
        array::array,
        dict::dict,
        integer::integer,
        numbered_dict::numbered_dict,
        set::set,
        space::{opt_space, req_space},
        tables::is_token,
        unrolled::take_while_unrolled,
        Res, Val,
    };

    pub fn set_of_collections<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(separated_list0(req_space, bracketed), |vals| Val::Set(vals))(input)
    }
    pub fn contents<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        let (remainder, maybe_key_number_idenentifier): (&'a str, &'a str) =
            take_while_unrolled::<'a, _, VerboseError<&'a str>>(move |character| {
                !is_token(character)
            })(input)?;

        let (_remainder, next_token) = take(1 as usize)(remainder)?;

        if next_token == "}" {
            return cut(set)(input);
        } else if next_token == "=" {
            return if integer(maybe_key_number_idenentifier).is_ok() {
                cut(array)(input)
            } else {
                cut(dict)(input)
            };
        } else if next_token == "{" {
            return if integer(maybe_key_number_idenentifier).is_ok() {
                cut(numbered_dict)(input)
            } else {
                cut(set_of_collections)(input)
            };
        } else {
            println!("AFTER: {}", input);
            println!("{}", next_token);
            panic!("Token = or }} not found, possibly missing a closing brace somewhere?")
        };
    }

    pub fn bracketed<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        delimited(
            char('{'),
            cut(delimited(opt_space, contents, opt_space)),
            char('}'),
        )(input)
    }
}

pub(self) mod numbered_dict {
    use std::collections::HashMap;

    use nom::{
        character::complete::{char, digit1},
        combinator::{map, map_res, recognize, verify},
        sequence::{delimited, tuple},
    };

    use super::{
        dict::hash_map,
        space::{opt_space, req_space},
        Res, Val,
    };

    pub fn numbered_dict<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(
            tuple((
                map_res(
                    verify(recognize(digit1), |s: &str| !s.is_empty()),
                    str::parse,
                ),
                req_space,
                delimited(
                    char('{'),
                    delimited(opt_space, hash_map, opt_space),
                    char('}'),
                ),
            )),
            |(number, _, map): (i64, &str, Vec<(&'a str, Val<'a>)>)| Val::NumberedDict(number, map),
        )(input)
    }
}
pub(self) mod value {
    use nom::{branch::alt, combinator::map};

    use super::{
        bracketed::bracketed, dict::hash_map, quoted::quoted, unquoted::unquoted, Res, Val,
    };

    pub fn value<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        alt((bracketed, quoted, unquoted))(input)
    }

    pub fn root<'a>(input: &'a str) -> Res<&'a str, Val<'a>> {
        map(hash_map, Val::Dict)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod helper {
        use std::fmt::Debug;

        use crate::parser::clausewitz::Res;

        pub fn assert_result_ok<T: Debug + Clone>(result: Res<&str, T>) {
            let result2 = result.clone();
            if result2.is_err() {
                match result2.clone().err().unwrap() {
                    nom::Err::Incomplete(e) => println!("{:#?}", e),
                    nom::Err::Error(e) => println!("{:#?}", e),
                    nom::Err::Failure(e) => println!("{:#?}", e),
                };
            }
            assert!(result.is_ok());
        }

        pub fn assert_result_err<T: Debug + Clone>(result: Res<&str, T>) {
            let result2 = result.clone();
            if result2.is_ok() {
                println!("{:#?}", result2.unwrap().1);
            }
            assert!(result.is_err());
        }
    }
    #[cfg(test)]
    mod space {
        use crate::parser::clausewitz::space::{opt_space, req_space};

        #[test]
        fn opt_space__empty_string__accepted() {
            let text = "";

            let (remainder, parse_output) = opt_space(text).unwrap();
            assert_eq!(remainder, "");
            assert_eq!(parse_output, "");
        }

        #[test]
        fn opt_space__all_space_chars__accepted() {
            let text = " \t\n\r";

            let (remainder, parse_output) = opt_space(text).unwrap();
            assert_eq!(remainder, "");
            assert_eq!(parse_output, " \t\n\r");
        }

        #[test]
        fn req_space__empty_string__rejected() {
            let text = "";
            assert!(req_space(text).is_err())
        }

        #[test]
        fn req_space__all_space_chars__accepted() {
            let text = " \t\n\r";

            let (remainder, parse_output) = req_space(text).unwrap();
            assert_eq!(remainder, "");
            assert_eq!(parse_output, " \t\n\r");
        }
    }

    #[cfg(test)]
    mod unquoted {
        use crate::parser::clausewitz::unquoted::unquoted;

        use super::*;

        #[test]
        fn unquoted__integer__integer() {
            let text = "0";
            let (_remainder, parse_output) = unquoted(text).unwrap();
            assert_eq!(parse_output, Val::Integer(0));
        }
        #[test]
        fn unquoted__decimal__decimal() {
            let text = "0.0";
            let (_remainder, parse_output) = unquoted(text).unwrap();
            assert_eq!(parse_output, Val::Decimal(0.0));
        }
        #[test]
        fn unquoted__identifier__identifier() {
            let text = "zer0";
            let (_remainder, parse_output) = unquoted(text).unwrap();
            assert_eq!(parse_output, Val::Identifier("zer0"));
        }

        #[cfg(test)]
        mod identifier_tests {
            use crate::parser::clausewitz::identifier::identifier;

            use super::*;

            #[test]
            fn identifire__alphanumeric_with_underscore__accepted() {
                let text = "alpha_numeric1234567890";
                let (remainder, parse_output) = identifier(text).unwrap();
                assert_eq!(parse_output, Val::Identifier(text));
                assert!(remainder.is_empty());
            }

            #[test]
            fn identifire__begins_with_number__rejectec() {
                let text = "0alpha_numeric1234567890";
                assert!(identifier(text).is_err());
            }

            #[test]
            fn identifire__empty__rejectec() {
                let text = "";
                assert!(identifier(text).is_err());
            }
        }
        #[cfg(test)]
        mod integer {
            use crate::parser::clausewitz::integer::integer;

            use super::*;

            #[test]
            fn integer__empty__rejected() {
                let text = "";
                assert!(integer(text).is_err());
            }
            #[test]
            fn integer__zero__accepted() {
                let text = "0";
                let (remainder, parse_output) = integer(text).unwrap();
                assert_eq!(parse_output, Val::Integer(0));
                assert!(remainder.is_empty());
            }

            #[test]
            fn integer__negative_number__accepted() {
                let text = "-1";
                let (remainder, parse_output) = integer(text).unwrap();
                assert_eq!(parse_output, Val::Integer(-1));
                assert!(remainder.is_empty());
            }

            #[test]
            fn integer__all_digits__accepted() {
                let text = "1234567890";
                let (remainder, parse_output) = integer(text).unwrap();
                assert_eq!(parse_output, Val::Integer(1234567890));
                assert!(remainder.is_empty());
            }

            #[test]
            fn integer__dots__accepted_up_to_dot_then_remainder() {
                let text = "-12345.6789";
                let (remainder, parse_output) = integer(text).unwrap();
                assert_eq!(parse_output, Val::Integer(-12345));
                assert_eq!(remainder, ".6789");
            }

            #[test]
            fn integer__letters__int_up_to_letter_then_remainder() {
                let text = "-1234567d89.098098";
                let (remainder, parse_output) = integer(text).unwrap();
                assert_eq!(parse_output, Val::Integer(-1234567));
                assert_eq!(remainder, "d89.098098");
            }
        }

        #[cfg(test)]
        mod decimal_tests {
            use crate::parser::clausewitz::decimal::decimal;

            use super::*;

            #[test]
            fn decimal__small_number__accepted() {
                let text = "0.00001011110110132";
                let (remainder, parse_output) = decimal(text).unwrap();
                assert_eq!(parse_output, Val::Decimal(0.00001011110110132));
                assert!(remainder.is_empty());
            }

            #[test]
            fn decimal__negative_number__accepted() {
                let text = "-0.1";
                let (remainder, parse_output) = decimal(text).unwrap();
                assert_eq!(parse_output, Val::Decimal(-0.1));
                assert!(remainder.is_empty());
            }

            #[test]
            fn decimal__all_digits__accepted() {
                let text = "-12345.6789";
                let (remainder, parse_output) = decimal(text).unwrap();
                assert_eq!(parse_output, Val::Decimal(-12345.6789));
                assert!(remainder.is_empty());
            }

            #[test]
            fn decimal__too_many_dots__accepted_with_remainder() {
                let text = "-12345.6789.098098";
                let (remainder, parse_output) = decimal(text).unwrap();
                assert_eq!(parse_output, Val::Decimal(-12345.6789));
                assert_eq!(remainder, ".098098");
            }

            #[test]
            fn decimal__letters__float_up_to_letter_then_remainder() {
                let text = "-12345.67d89.098098";
                let (remainder, parse_output) = decimal(text).unwrap();
                assert_eq!(parse_output, Val::Decimal(-12345.67));
                assert_eq!(remainder, "d89.098098");
            }
        }
    }

    #[cfg(test)]
    mod quoted {
        use time::{Date, Month};

        use crate::parser::clausewitz::{quoted::quoted, Val};

        #[test]
        fn quoted__date__date() {
            let text = "\"2200.01.01\"";
            let (_remainder, parse_output) = quoted(text).unwrap();
            assert_eq!(
                parse_output,
                Val::Date(Date::from_calendar_date(2200, Month::January, 01).unwrap())
            );
        }

        #[test]
        fn quoted__not_date__string() {
            let text = "\"2200.011\"";
            let (_remainder, parse_output) = quoted(text).unwrap();
            assert_eq!(parse_output, Val::StringLiteral("2200.011"));
        }

        #[cfg(test)]
        mod date_test {
            use time::Month;

            use crate::parser::clausewitz::date::date;

            use super::*;
            #[test]
            fn date__decimal_separated_yyyy_mm_date__accepted() {
                let text = "2200.01.01";
                let (_remainder, parse_output) = date(text).unwrap();
                assert_eq!(
                    parse_output,
                    Val::Date(Date::from_calendar_date(2200, Month::January, 01).unwrap())
                );
            }

            #[test]
            fn date__4digit_year__accepted() {
                let text = "2200.01.01";
                let (_remainder, parse_output) = date(text).unwrap();
                assert_eq!(
                    parse_output,
                    Val::Date(Date::from_calendar_date(2200, Month::January, 01).unwrap())
                );
            }

            #[test]
            fn date__3digit_year__accepted() {
                let text = "200.01.01";
                let (_remainder, parse_output) = date(text).unwrap();
                assert_eq!(
                    parse_output,
                    Val::Date(Date::from_calendar_date(200, Month::January, 01).unwrap())
                );
            }

            #[test]
            fn date__2digit_year__accepted() {
                let text = "20.01.01";
                let (_remainder, parse_output) = date(text).unwrap();
                assert_eq!(
                    parse_output,
                    Val::Date(Date::from_calendar_date(20, Month::January, 01).unwrap())
                );
            }

            #[test]
            fn date__1digit_year__accepted() {
                let text = "2.01.01";
                let (_remainder, parse_output) = date(text).unwrap();
                assert_eq!(
                    parse_output,
                    Val::Date(Date::from_calendar_date(2, Month::January, 01).unwrap())
                );
            }
        }

        #[cfg(test)]
        mod string_literal_test {
            use crate::parser::clausewitz::string_literal::string_literal;

            use super::*;
            #[test]
            fn string_literal__string__accepted() {
                let text =
                    "this is a string with a bun1234567890ch of special characters!@#$%^&*(_()";
                let (_remainder, parse_output) = string_literal(text).unwrap();
                assert_eq!(parse_output, Val::StringLiteral(text));
            }

            #[test]
            fn string_literal__decimal_separated_yyyy_mm_string_litteral__accepted() {
                let (remainder_quote, result_quote) = string_literal("\"").unwrap();
                let (remainder_eq, result_eq) = string_literal("=").unwrap();
                let (remainder_lbracket, result_lbracket) = string_literal("{").unwrap();
                let (remainder_rbracket, result_rbracket) = string_literal("}").unwrap();

                assert_eq!(result_quote, Val::StringLiteral(""));
                assert_eq!(result_eq, Val::StringLiteral(""));
                assert_eq!(result_lbracket, Val::StringLiteral(""));
                assert_eq!(result_rbracket, Val::StringLiteral(""));

                assert_eq!(remainder_quote, "\"");
                assert_eq!(remainder_eq, "=");
                assert_eq!(remainder_lbracket, "{");
                assert_eq!(remainder_rbracket, "}");
            }
        }
    }

    #[cfg(test)]
    mod bracketed {
        use crate::parser::clausewitz::bracketed::bracketed;

        use super::helper::assert_result_ok;

        #[test]
        fn bracketed__dict__dict() {
            let text = r###"{
                first="first"
                second="second"
        }"###;
            let result = bracketed(text);
            assert_result_ok(result)
        }

        #[test]
        fn bracketed__array__array() {
            let text = r###"{
            0="first"
            1="second"
        }"###;
            let result = bracketed(text);
            assert_result_ok(result)
        }

        #[test]
        fn bracketed__set__set() {
            let text = r###"{
            "first"
            "second"
        }"###;
            let result = bracketed(text);
            assert_result_ok(result)
        }
        #[cfg(test)]
        mod key_value {
            use crate::parser::clausewitz::{dict::key_value, tests::helper::assert_result_ok};

            #[test]
            fn key_value__unquoted__accepted() {
                let text = r###"key="value"
                "###;
                let result = key_value(text);
                assert_result_ok(result)
            }

            #[test]
            fn key_value__quoted__accepted() {
                let text = r###""key"=0
                "###;
                let result = key_value(text);
                assert_result_ok(result)
            }
        }
        #[cfg(test)]
        mod dict {}

        #[cfg(test)]
        mod number_value {}

        #[cfg(test)]
        mod array {}
        #[cfg(test)]
        mod set {}
    }

    #[cfg(test)]
    mod edge_cases {
        use crate::parser::clausewitz::value::root;

        use super::helper::assert_result_ok;

        #[test]
        fn basics() {
            let text = r###"vers_ion0="Herbert v3.2.2"
            version_control_revision=83287
            date="2200.05.01"
            date="0.05.01"
            float=-0.123939887
            "###;

            let result = root(text);
            assert_result_ok(result);
        }

        #[test]
        fn set_numbers_same_line() {
            let text = r###"set_of_numbers={
                40 41
            }
            "###;
            let result = root(text);
            assert_result_ok(result);
        }
        #[test]
        fn space_not_new_line() {
            let text = r###"modules={
                0=shipyard				1=trading_hub			}
                "###;
            let result = root(text);
            assert_result_ok(result);
        }

        #[test]
        fn intel_numbered_dicts() {
            let text = r###"intel={
                                    {
                                        14 {
                                            intel=0
                                            stale_intel={
                                            }
                                        }
                                    }
                                    {
                                        19 {
                                            intel=0
                                            stale_intel={
                                            }
                                        }
                                    }
                                }
"###;
            let result = root(text);

            assert_result_ok(result);
        }

        #[test]
        fn dict_of_dicts() {
            let text = r###"dict_of_dicts={
                icon={
                    category="human"
                    file="flag_human_9.dds"
                }
                background={
                    category="backgrounds"
                    file="00_solid.dds"
                }
                colors={
                    "blue"
                    "black"
                    "null"
                    "null"
                }
            }"###;
            let result = root(text);
            assert_result_ok(result);
        }

        #[test]
        fn quoted__key__ok() {
            let text = r###""The name Of A Ship"=0
            "###;
            let result = root(text);
            assert_result_ok(result);
        }

        #[test]
        fn empty__set__set() {
            let text = r###"empty_set={}
            "###;
            let result = root(text);
            assert_result_ok(result);
        }

        #[test]
        fn root__set_of_strings__accepted() {
            let text = r###"set_of_strings={
                "Ancient Relics Story Pack"
                "Anniversary Portraits"
                "Apocalypse"
            }
            "###;
            let result = root(text);
            assert_result_ok(result);
        }

        #[test]
        fn array__of__arrays() {
            let text = r###"array_of_arrays={
                0={
                    0="a"
                }
                1={
                    0="one"
                }
                2={
                    0="two"
                }
            }
            "###;
            let result = root(text);
            assert_result_ok(result);
        }

        #[test]
        fn identifier__with__underscore() {
            let text = r###"identifier=identi_fire
            "###;
            let result = root(text);
            assert_result_ok(result);
        }

        #[test]
        fn dict__key_identifier_pairs__ok() {
            let text = r###"dict={
                alpha=a
                beta=b
                cthulhu=ilhjok
            }
            "###;
            let result = root(text);
            assert_result_ok(result);
        }
    }

    #[cfg(test)]
    mod files {
        use std::{
            fs::{self, File},
            io::Read,
            time::Instant,
        };

        use memmap::Mmap;

        use crate::parser::clausewitz::value::root;

        use super::helper::assert_result_ok;
        #[test]
        fn meta() {
            let text = fs::read_to_string("/home/michael/Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/meta").unwrap();
            let result = root(text.as_str());

            assert_result_ok(result);
        }

        #[test]
        fn gamestate() {
            let text = fs::read_to_string("/home/michael/Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/gamestate").unwrap();

            let start = Instant::now();

            let result = root(text.as_str());

            let duration = start.elapsed();
            println!("no mmap: Time elapsed is: {:?}", duration);

            assert_result_ok(result);
        }

        #[test]
        fn gamestate_memmap__for_epic_files() {
            let filename = "/home/michael/Dev/stellarust/res/test_data/campaign_raw/unitednationsofearth_-15512622/autosave_2200.02.01/gamestate";
            let file = File::open(filename).expect("File not found");

            let mmap =
                unsafe { Mmap::map(&file).expect(&format!("Error mapping file {:?}", file)) };

            let str = std::str::from_utf8(&mmap[..]).unwrap();

            let start = Instant::now();

            let result = root(str);

            let duration = start.elapsed();
            println!("mmap: Time elapsed is: {:?}", duration);

            assert_result_ok(result);
        }
    }
}
