#![allow(clippy::too_many_arguments)]
use std::num::IntErrorKind;

use bitflags::bitflags;

bitflags! {
    pub struct XToIntFlag:u8 {
        const MinQuiet = 0x01;
        const MaxQuiet = 0x02;
        const MinRange = 0x04;
        const MaxRange = 0x08;
    }
}

pub fn xnumtoumax(
    n_str: &str,
    base: u32,
    min: usize,
    max: usize,
    suffixes: &str,
    erri: &str,
    err_exit: i32,
    flags: XToIntFlag,
) -> usize {
    let s = n_str.trim();
    let (sign, digits) = match s.split_at(1) {
        ("+", rest) => (1, rest),
        ("-", rest) => (-1, rest),
        _ => (1, s),
    };
    let s = digits;
    0
}

pub fn xstrtol(nptr: &str, base: i32, valid_suffixes: &str) {}

pub fn strtol(nptr: &str, base: i32) -> Result<(i64, usize), IntErrorKind> {
    let bytes = nptr.as_bytes();
    let mut i = 0;

    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    if i == bytes.len() {
        return Err(IntErrorKind::InvalidDigit);
    }

    let mut negative = false;
    if bytes[i] == b'-' {
        negative = true;
        i += 1;
    } else if bytes[i] == b'+' {
        i += 1;
    }

    let mut base = base as i64;
    if i < bytes.len() && bytes[i] == b'0' {
        if (base == 0 || base == 16)
            && i + 1 < bytes.len()
            && bytes[i + 1].eq_ignore_ascii_case(&b'x')
        {
            base = 16;
            i += 2;
        } else if (base == 0 || base == 2)
            && i + 1 < bytes.len()
            && bytes[i + 1].eq_ignore_ascii_case(&b'b')
        {
            base = 2;
            i += 2;
        } else if base == 0 {
            base = 8;
        }
    } else if base == 0 {
        base = 10;
    }

    let cutoff = i64::MAX / base;
    let cutlim = i64::MAX % base;
    let mut invalid = true;
    let mut result: i64 = 0;
    while i < bytes.len() {
        let c = bytes[i];
        let digit = if c.is_ascii_digit() {
            (c - b'0') as i64
        } else if c.is_ascii_alphabetic() {
            (c.to_ascii_uppercase() - b'A') as i64 + 10
        } else {
            break;
        };

        if digit >= base {
            break;
        }

        if result > cutoff || (result == cutoff && digit > cutlim) {
            return Err(IntErrorKind::PosOverflow);
        } else {
            result *= base;
            result += digit;
        }

        invalid = false;
        i += 1;
    }
    if invalid {
        return Err(IntErrorKind::InvalidDigit);
    }

    Ok((if negative { -result } else { result }, i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string() {
        assert_eq!(strtol("123", 10), Ok((123, 3)));
        assert_eq!(strtol("123zz", 10), Ok((123, 3)));
        assert_eq!(strtol("   -42", 10), Ok((-42, 6)));

        assert_eq!(strtol("0xFF", 0), Ok((255, 4)));
        assert_eq!(strtol("0b1011", 0), Ok((11, 6)));
        assert_eq!(strtol("0755", 0), Ok((493, 4)));
        assert_eq!(strtol("zzz", 36), Ok((46655, 3)));

        assert_eq!(strtol("", 10), Err(IntErrorKind::InvalidDigit));
        assert_eq!(strtol("   ", 10), Err(IntErrorKind::InvalidDigit));
        assert_eq!(strtol("zzz", 10), Err(IntErrorKind::InvalidDigit));
        assert_eq!(
            strtol("18446744073709551616", 10),
            Err(IntErrorKind::PosOverflow)
        );
    }
}
