use alloc::string::String;
use alloc::vec;

/// Greater than check. Simpler than implementing [`PartialOrd`].
pub trait Gt {
    /// Check if self is greater than other.
    fn gt(&self, other: &Self) -> bool;
}

/// Good old bubble sort. Slower but much smaller than the built-in sort function.
pub fn bubble_sort<T: Gt>(items: &mut [T]) {
    let len = items.len();
    if len <= 1 {
        return;
    }
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..len - 1 {
            if items[i].gt(&items[i + 1]) {
                items.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

/// Case-insensitive comparison of two ASCII strings.
pub fn ascii_gt(s1: &str, s2: &str) -> bool {
    for (c1, c2) in s1.as_bytes().iter().zip(s2.as_bytes()) {
        let c1 = c1.to_ascii_lowercase();
        let c2 = c2.to_ascii_lowercase();
        if c1 != c2 {
            return c1 > c2;
        }
    }
    s1.len() > s2.len()
}

/// Format the given date as YYYY-MM-DD.
/// You can achieve the same using [`alloc::format`].
/// However, writing application-specific formatters
/// (and taking into account application-specific constraints)
/// produces smaller and faster code.
pub fn format_date(date: (u16, u8, u8)) -> String {
    let (y, m, d) = date;
    let y = (y % 100) as u8;
    let bytes = vec![
        b'2',
        b'0',
        format_digit2(y),
        format_digit1(y),
        b'-',
        format_digit2(m),
        format_digit1(m),
        b'-',
        format_digit2(d),
        format_digit1(d),
    ];
    unsafe { String::from_utf8_unchecked(bytes) }
}

/// Convert to a byte character the second digit of an integer.
const fn format_digit2(dig: u8) -> u8 {
    format_digit1(dig / 10)
}

/// Convert to a byte character the first digit of an integer.
const fn format_digit1(dig: u8) -> u8 {
    b'0' + (dig % 10)
}
