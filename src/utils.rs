/// Greater than check. Simpler than implementing [`PartialOrd`].
pub trait Gt {
    /// Check if self is greater than other.
    fn gt(&self, other: &Self) -> bool;
}

/// Good old bubble sort. Slower but much smaller than the built-in sort function.
pub fn bubble_sort<T: Gt>(apps: &mut [T]) {
    let len = apps.len();
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 0..len - 1 {
            if apps[i].gt(&apps[i + 1]) {
                apps.swap(i, i + 1);
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
