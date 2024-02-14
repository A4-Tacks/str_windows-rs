use std::iter::FusedIterator;


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
        end: nth_indice(input, size),
        size,
    }
}

fn next_indice(s: &str) -> usize {
    for i in 1..=s.len() {
        if s.is_char_boundary(i) {
            return i;
        }
    }
    return 1;
}

fn nth_indice(s: &str, n: usize) -> usize {
    s.char_indices()
        .nth(n)
        .map(|x| x.0)
        .unwrap_or(s.len())
}

struct StrWindowsIter<'a> {
    inner: &'a str,
    end: usize,
    size: usize,
}

impl<'a> FusedIterator for StrWindowsIter<'a> { }

impl<'a> Iterator for StrWindowsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // degenerate case
        if self.size == 0 {
            return Some("");
        }
        if self.end > self.inner.len() {
            return None;
        }
        let inner = &self.inner[..self.end];
        let skip_len = next_indice(self.inner);
        self.end += next_indice(&self.inner[self.end..]);
        self.end -= skip_len;
        self.inner = &self.inner[skip_len..];
        Some(inner)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.size == 0 {
            return (usize::MAX, None);
        }
        let (min, max) = self.inner.chars().size_hint();
        let f = |n: usize|
            n.checked_sub(self.size - 1)
                .unwrap_or(0);
        (f(min), max.map(f))
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

    #[test]
    fn test() {
        let datas = [
            (
                "hello, 你好, hi",
                &[
                    &[
                        "h",
                        "e",
                        "l",
                        "l",
                        "o",
                        ",",
                        " ",
                        "你",
                        "好",
                        ",",
                        " ",
                        "h",
                        "i",
                    ][..],
                    &[
                        "he",
                        "el",
                        "ll",
                        "lo",
                        "o,",
                        ", ",
                        " 你",
                        "你好",
                        "好,",
                        ", ",
                        " h",
                        "hi",
                    ],
                    &[
                        "hel",
                        "ell",
                        "llo",
                        "lo,",
                        "o, ",
                        ", 你",
                        " 你好",
                        "你好,",
                        "好, ",
                        ", h",
                        " hi",
                    ],
                    &[
                        "hell",
                        "ello",
                        "llo,",
                        "lo, ",
                        "o, 你",
                        ", 你好",
                        " 你好,",
                        "你好, ",
                        "好, h",
                        ", hi",
                    ],
                    &[
                        "hello",
                        "ello,",
                        "llo, ",
                        "lo, 你",
                        "o, 你好",
                        ", 你好,",
                        " 你好, ",
                        "你好, h",
                        "好, hi",
                    ],
                ][..]
            ),
            (
                "头尾都是",
                &[
                    &[
                        "头",
                        "尾",
                        "都",
                        "是",
                    ][..],
                    &[
                        "头尾",
                        "尾都",
                        "都是",
                    ],
                    &[
                        "头尾都",
                        "尾都是",
                    ],
                    &[
                        "头尾都是",
                    ],
                ][..]
            ),
            (
                "头部是!",
                &[
                    &[
                        "头",
                        "部",
                        "是",
                        "!",
                    ][..],
                    &[
                        "头部",
                        "部是",
                        "是!",
                    ],
                    &[
                        "头部是",
                        "部是!",
                    ],
                    &[
                        "头部是!",
                    ],
                ][..]
            ),
            (
                ": 尾部是",
                &[
                    &[
                        ":",
                        " ",
                        "尾",
                        "部",
                        "是",
                    ][..],
                    &[
                        ": ",
                        " 尾",
                        "尾部",
                        "部是",
                    ],
                    &[
                        ": 尾",
                        " 尾部",
                        "尾部是",
                    ],
                    &[
                        ": 尾部",
                        " 尾部是",
                    ],
                    &[
                        ": 尾部是",
                    ],
                ][..]
            ),
        ];
        for (src, tests) in datas {
            for (i, &test) in tests.iter().enumerate() {
                assert_eq!(str_windows(src, i+1).collect::<Vec<_>>(), test);
            }
        }
    }

    #[test]
    fn size_hint_test() {
        let src = "abcde";
        assert_eq!(str_windows(src, 0).size_hint(), (usize::MAX, None));
        assert_eq!(str_windows(src, 1).size_hint(), (2, Some(5)));
        assert_eq!(str_windows(src, 2).size_hint(), (1, Some(4)));
        assert_eq!(str_windows(src, 3).size_hint(), (0, Some(3)));
        assert_eq!(str_windows(src, 4).size_hint(), (0, Some(2)));
        assert_eq!(str_windows(src, 5).size_hint(), (0, Some(1)));
        assert_eq!(str_windows(src, 6).size_hint(), (0, Some(0)));
        assert_eq!(str_windows(src, 7).size_hint(), (0, Some(0)));
    }
}
