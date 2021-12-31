use nom::{error::VerboseError, IResult};
use time::Date;

#[cfg(test)]
pub(self) mod tests;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse2"
))]
pub(self) mod simd;

pub(self) mod bracketed;
pub(self) mod quoted;
pub mod root;
pub(self) mod space;
pub(self) mod tables;
pub(self) mod unquoted;
pub(self) mod value;

pub(crate) type Res<T, S> = IResult<T, S, VerboseError<T>>;

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
