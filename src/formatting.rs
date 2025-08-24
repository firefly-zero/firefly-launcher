//! This module contains string formatting functions.
//!
//! You can achieve the same using [`alloc::format`].
//! However, writing application-specific formatters
//! (and taking into account application-specific constraints)
//! produces smaller and faster code.
use alloc::string::String;
use alloc::vec;

/// Format the given date as YYYY-MM-DD.
pub fn format_date(date: (u16, u8, u8)) -> String {
    let (y, m, d) = date;
    let y = (y % 100) as u8;
    let bytes = vec![
        b'2',
        b'0',
        digit2(y),
        digit1(y),
        b'-',
        digit2(m),
        digit1(m),
        b'-',
        digit2(d),
        digit1(d),
    ];
    unsafe { String::from_utf8_unchecked(bytes) }
}

/// Convert to a byte character the second digit of an integer.
const fn digit2(dig: u8) -> u8 {
    digit1(dig / 10)
}

/// Convert to a byte character the first digit of an integer.
const fn digit1(dig: u8) -> u8 {
    b'0' + (dig % 10)
}
