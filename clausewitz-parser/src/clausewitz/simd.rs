use nom::error::VerboseError;
use std::arch::x86_64::{
    _mm_cmpestri, _mm_loadu_si128, _SIDD_CMP_RANGES, _SIDD_LEAST_SIGNIFICANT, _SIDD_UBYTE_OPS,
};
use std::cmp::min;

//the range of all the characters which should be REJECTED
pub const SPACE_RANGES: &[u8] = b"\x00\x08\x0e\x1f\x21\xff";

pub const TOKEN_RANGES: &[u8] = b"\x00\x3c\x3e\x7a\x7c\x7c\x7e\xff";
pub const NOT_TOKEN_RANGES: &[u8] = b"\x3d\x3d\x7b\x7b\x7d\x7d";

pub const NUMBER_RANGES: &[u8] = b"\x00\x2f\x3a\xff";
pub const ALPHABET_RANGES: &[u8] = b"\x00\x40\x5b\x60\x7b\xff";
pub const STRING_LITTERAL_CONTENT_RANGES: &[u8] =
    b"\x00\x08\x0e\x1f\x22\x22\x3d\x3d\x7b\x7b\x7d\x7d\x7f\xff";
pub const IDENTIFIER_RANGES: &[u8] = b"\x00\x2f\x3a\x40\x5b\x5e\x60\x60\x7b\xff";

use nom::error::ParseError;

use super::Res;

pub fn take_while<'a, Condition, Error: ParseError<&'a str>>(
    cond: Condition,
    ranges: &'static [u8],
) -> impl Fn(&'a str) -> Res<&'a str, &'a str>
where
    Condition: Fn(char) -> bool,
{
    // move |i: &'a str| take_while_unrolled(i, |c| !cond(c))
    move |input: &'a str| take_while_simd(input, |c| cond(c), ranges)
}

//TODO: make private
pub fn take_while_simd<'a, F>(str: &'a str, condition: F, ranges: &[u8]) -> Res<&'a str, &'a str>
where
    F: Fn(char) -> bool,
{
    let len = str.len();
    if len == 0 {
        return Ok(("", ""));
    } else if len >= 16 {
        // println!("simd");
        let start = str.as_ptr() as usize;
        let mut i = str.as_ptr() as usize;
        // ranges is a byte array of pair-wise ranges. 0-3;4-9;etc
        // ie ranges : &[u8] = b"\x00\xff"
        let ranges16 = unsafe { _mm_loadu_si128(ranges.as_ptr() as *const _) };
        let ranges_len = ranges.len() as i32;
        loop {
            let s1 = unsafe { _mm_loadu_si128(i as *const _) };

            let idx = unsafe {
                _mm_cmpestri(
                    ranges16,
                    ranges_len,
                    s1,
                    16,
                    _SIDD_LEAST_SIGNIFICANT | _SIDD_CMP_RANGES | _SIDD_UBYTE_OPS,
                )
            };

            if idx != 16 {
                i += idx as usize;
                break;
            }
            i += 16;
        }

        let index = i - start;
        let (before, after) = str.split_at(min(index, len));
        return Ok((after, before));
    } else {
        let mut index = 0usize;

        loop {
            if len - index < 8 {
                break;
            }
            if !condition((*str.as_bytes().get(index).unwrap()) as char) {
                break;
            }
            index += 1;

            if !condition((*str.as_bytes().get(index).unwrap()) as char) {
                break;
            }
            index += 1;

            if !condition((*str.as_bytes().get(index).unwrap()) as char) {
                break;
            }
            index += 1;

            if !condition((*str.as_bytes().get(index).unwrap()) as char) {
                break;
            }
            index += 1;

            if !condition((*str.as_bytes().get(index).unwrap()) as char) {
                break;
            }
            index += 1;

            if !condition((*str.as_bytes().get(index).unwrap()) as char) {
                break;
            }
            index += 1;

            if !condition((*str.as_bytes().get(index).unwrap()) as char) {
                break;
            }
            index += 1;
        }
        if len - index < 8 {
            loop {
                if !condition((*str.as_bytes().get(index).unwrap()) as char) {
                    break;
                }
                index += 1;

                if index == len {
                    break;
                }
            }
        }

        let (before, after) = str.split_at(index);
        return Ok((after, before));
    }
}

#[cfg(test)]
mod tests {
    use crate::clausewitz::tables::is_space;

    use super::*;
    #[test]
    fn take_while_simd__string_with_leading_whitespace__whitespace_collected_remainder_returned() {
        let text = " \t\n\r|Stop this is a big long string";
        let ranges = SPACE_RANGES;
        let (remainder, parsed) = take_while_simd(text, is_space, ranges).unwrap();
        assert_eq!(remainder, "|Stop this is a big long string");
        assert_eq!(parsed, " \t\n\r");
    }

    #[test]
    fn take_while_simd__string_with_many_leading_whitespace__whitespace_collected_remainder_returned(
    ) {
        let text = "\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t|Stop this is a big long string";
        let ranges = SPACE_RANGES;
        let (remainder, parsed) = take_while_simd(text, is_space, ranges).unwrap();
        assert_eq!(remainder, "|Stop this is a big long string");
        assert_eq!(parsed, "\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t");
    }

    #[test]
    fn take_while_simd__short_string__whitespace_collected_remainder_returned() {
        let text = "\t\t\ts";
        let ranges = SPACE_RANGES;
        let (remainder, parsed) = take_while_simd(text, is_space, ranges).unwrap();
        assert_eq!(remainder, "s");
        assert_eq!(parsed, "\t\t\t");
    }

    #[test]
    fn take_while_simd__all_white_space__whitespace_collected_remainder_returned() {
        let text = " \t\n\r";
        let ranges = SPACE_RANGES;
        let (remainder, parsed) = take_while_simd(text, is_space, ranges).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(parsed, " \t\n\r");
    }
}
