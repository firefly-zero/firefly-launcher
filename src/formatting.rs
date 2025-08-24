//! This module contains string formatting functions.
//!
//! You can achieve the same using [`alloc::format`].
//! However, writing application-specific formatters,
//! taking into account application-specific constraints,
//! produces much smaller and faster code.
use alloc::string::String;

/// Format the given date as YYYY-MM-DD.
pub fn format_date(date: (u16, u8, u8)) -> String {
    let (y, m, d) = date;
    let mut buf = String::new();
    format_int04(&mut buf, y);
    buf.push('-');
    format_int02(&mut buf, m);
    buf.push('-');
    format_int02(&mut buf, d);
    buf
}

/// Format a 4-digit integer padding it with zeros on the left.
pub fn format_int04(buf: &mut String, val: u16) {
    format_digit(buf, (val / 1000) as u8);
    format_digit(buf, ((val / 100) % 10) as u8);
    format_digit(buf, ((val / 10) % 10) as u8);
    format_digit(buf, (val % 10) as u8);
}

/// Format a 2-digit integer padding it with zeros on the left.
pub fn format_int02(buf: &mut String, val: u8) {
    format_digit(buf, val / 10);
    format_digit(buf, val % 10);
}

/// Format a single-digit integer.
pub fn format_digit(buf: &mut String, dig: u8) {
    buf.push((b'0' + dig) as char);
}
