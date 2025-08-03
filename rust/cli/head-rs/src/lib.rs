#![allow(clippy::too_many_arguments)]
use std::num::IntErrorKind;

use bitflags::bitflags;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum XstrtolError {
    IntErrorKind(IntErrorKind),
    InvalidSuffixChar,
}

bitflags! {
    pub struct XToIntFlag:u8 {
        const MinQuiet = 0x01;
        const MaxQuiet = 0x02;
    }
}

pub fn xnumtoint(
    n_str: &str,
    min: i64,
    max: i64,
    suffixes: Option<&str>,
    flags: XToIntFlag,
) -> Result<i64, XstrtolError> {
    let trimmed = n_str.trim();
    let (sign, digits) = match trimmed.split_at(1) {
        ("+", rest) => (1, rest),
        ("-", rest) => (-1, rest),
        _ => (1, trimmed),
    };
    let num_end = digits
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(digits.len());
    let (num_part, suffix_part) = digits.split_at(num_end);

    let base = 10;
    let cutoff = max / base;
    let cutlim = max % base;
    let mut overflow = false;
    let mut result: i64 = 0;
    for c in num_part.bytes() {
        let digit = (c - b'0') as i64;
        if result > cutoff || (result == cutoff && digit > cutlim) {
            overflow = true;
            break;
        } else {
            result *= base;
            result += digit;
        }
    }

    if let Some(suffixes) = suffixes
        && !suffix_part.is_empty()
    {
        let mut xbase = 1024;
        let mut chars = suffix_part.chars().multipeek();

        let mut nth_suffix = 0;
        if let Some(&c1) = chars.peek() {
            if "EGgKkMmPQRTtYZ".contains(c1)
                && suffixes.contains('0')
                && let Some(&c2) = chars.peek()
            {
                if c2 == 'i'
                    && let Some('B') = chars.peek()
                {
                    nth_suffix = 2;
                } else if c2 == 'B' {
                    xbase = 1000;
                    nth_suffix = 1;
                }
            }

            match c1 {
                'b' => result = bkm_scale(result, 512)?,
                'E' => result = bkm_scale_by_power(result, xbase, 6)?,
                'G' | 'g' => result = bkm_scale_by_power(result, xbase, 3)?,
                'K' | 'k' => result = bkm_scale_by_power(result, xbase, 1)?,
                'M' | 'm' => result = bkm_scale_by_power(result, xbase, 2)?,
                'P' => result = bkm_scale_by_power(result, xbase, 5)?,
                'Q' => result = bkm_scale_by_power(result, xbase, 10)?,
                'R' => result = bkm_scale_by_power(result, xbase, 9)?,
                'T' | 't' => result = bkm_scale_by_power(result, xbase, 4)?,
                'w' => result = bkm_scale(result, 2)?,
                'Y' => result = bkm_scale_by_power(result, xbase, 8)?,
                'Z' => result = bkm_scale_by_power(result, xbase, 7)?,
                _ => {
                    return Err(XstrtolError::InvalidSuffixChar);
                }
            }
        }

        chars.nth(nth_suffix);
        if chars.next().is_some() {
            return Err(XstrtolError::InvalidSuffixChar);
        }
    }

    if overflow {
        match sign {
            s if s > 0 && flags.contains(XToIntFlag::MaxQuiet) => Ok(max),
            s if s < 0 && flags.contains(XToIntFlag::MinQuiet) => Ok(min),
            _ => Ok(0),
        }
    } else {
        Ok(sign * result)
    }
}

fn bkm_scale(num: i64, scale_factor: i64) -> Result<i64, XstrtolError> {
    num.checked_mul(scale_factor)
        .ok_or(XstrtolError::IntErrorKind(if num < 0 {
            IntErrorKind::NegOverflow
        } else {
            IntErrorKind::PosOverflow
        }))
}

fn bkm_scale_by_power(num: i64, base: i64, power: i64) -> Result<i64, XstrtolError> {
    (0..power).try_fold(num, |acc, _| bkm_scale(acc, base))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_suffix() {
        assert_eq!(
            xnumtoint(
                "1 z",
                0,
                i64::MAX,
                Some("bKkMmGTPEZYRQ0"),
                XToIntFlag::MaxQuiet
            ),
            Err(XstrtolError::InvalidSuffixChar)
        );
    }

    #[test]
    fn with_suffix() {
        assert_eq!(
            xnumtoint(
                "1E",
                0,
                i64::MAX,
                Some("bKkMmGTPEZYRQ0"),
                XToIntFlag::MaxQuiet
            ),
            Ok(i64::pow(1024, 6))
        );
    }
}
