

/// Returns substrings of length `size`, similar to `slice::windows`.
///
/// # Examples
///
/// ```
/// use str_windows::str_windows;
///
/// let input = "s 😀😁";
/// let mut iter = str_windows(input, 3);
/// assert_eq!(iter.next(), Some("s 😀"));
/// assert_eq!(iter.next(), Some(" 😀😁"));
/// assert!(iter.next().is_none());
/// ```
pub fn str_windows<'a>(input: &'a str, size: usize) -> impl Iterator<Item=&'a str> {
    StrWindowsIter {
        inner: input,
        size,
    }
}

struct StrWindowsIter<'a> {
    inner: &'a str,
    size: usize,
}

impl<'a> Iterator for StrWindowsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // degenerate case
        if self.size == 0 {
            return Some("");
        }
        let mut char_count = 0;
        let mut first_char_len = 0;
        let mut found_first_char = false;
        for pos in 1..=self.inner.len() {
            if self.inner.is_char_boundary(pos) {
                char_count += 1;
                if ! found_first_char {
                    first_char_len = pos;
                    found_first_char = true;
                }
            }
            if char_count == self.size {
                let inner = self.inner;
                self.inner = &inner[first_char_len..];
                return Some(&inner[..pos]);
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::str_windows;
    #[test]
    fn it_works() {
        let test_str = "test str_😃";
        let expected_arr = ["te", "es", "st", "t ", " s", "st", "tr", "r_", "_😃"];
        for (test, &expected) in str_windows(test_str, 2).zip(expected_arr.iter()) {
            println!("{} = {}", test, expected);
            assert_eq!(test, expected);
        }
        assert_eq!(str_windows(test_str, 2).count(), expected_arr.len());
    }

    #[test]
    fn degenerate() {
        let test_str = "any string";
        let mut iter = str_windows(test_str, 0);
        for _ in 0..100 {
            assert_eq!(iter.next(), Some(""));
        }
    }
}
