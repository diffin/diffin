use std::{iter, mem};

/// A simple iterator over the suffixes of a UTF-8 string (`&str`) or an
/// arbitrary slice (`&[T]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuffixIter<'a, S: ?Sized> {
    suffix: &'a S,
}

impl<'a, S: ?Sized> From<&'a S> for SuffixIter<'a, S> {
    #[inline]
    fn from(suffix: &'a S) -> Self {
        Self { suffix }
    }
}

impl<'a> Iterator for SuffixIter<'a, str> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        fn next_char_boundary(s: &str) -> Option<usize> {
            if s.is_empty() {
                None
            } else if s.is_char_boundary(1) {
                Some(1)
            } else if s.is_char_boundary(2) {
                Some(2)
            } else if s.is_char_boundary(3) {
                Some(3)
            } else {
                // Rust's UTF-8 strings are assumed to have unicode scalars that
                // are at most 4 bytes long.
                Some(4)
            }
        }
        let next_index = next_char_boundary(self.suffix)?;
        let next = unsafe { self.suffix.get_unchecked(next_index..) };
        Some(mem::replace(&mut self.suffix, next))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Adapted from `str::Chars::size_hint`.
        let len = self.suffix.len();
        ((len + 3) / 4, Some(len))
    }

    fn count(self) -> usize {
        self.suffix.as_bytes().iter().fold(0, |n, &b| {
            // Taken from `str::is_char_boundary`:
            // "This is bit magic equivalent to: b < 128 || b >= 192"
            let is_char_boundary = (b as i8) >= -0x40;
            n + is_char_boundary as usize
        })
    }
}

impl<'a> iter::FusedIterator for SuffixIter<'a, str> {}

impl<'a, T> Iterator for SuffixIter<'a, [T]> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.suffix.get(1..)?;
        Some(mem::replace(&mut self.suffix, next))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.suffix.len(), Some(self.suffix.len()))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl<'a, T> ExactSizeIterator for SuffixIter<'a, [T]> {
    #[inline]
    fn len(&self) -> usize {
        self.suffix.len()
    }
}

impl<'a, T> iter::FusedIterator for SuffixIter<'a, [T]> {}

impl<'a, S: ?Sized> SuffixIter<'a, S> {
    /// Creates a new instance from `suffix`, treating it as the first suffix if
    /// non-empty.
    #[inline]
    pub const fn new(suffix: &'a S) -> Self {
        Self { suffix }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHARS: &[&str] = &[
        // 1 byte
        "\u{0000}",
        "\u{007F}",
        // 2 bytes
        "\u{0080}",
        "\u{07FF}",
        // 3 bytes
        "\u{0800}",
        "\u{FFFF}",
        // 4 bytes
        "\u{10000}",
        "\u{10FFFF}",
    ];

    #[test]
    fn str_count() {
        assert_eq!(SuffixIter::new("").count(), 0);

        for a in CHARS {
            for b in CHARS {
                for c in CHARS {
                    for d in CHARS {
                        let s = format!("{}{}{}{}", a, b, c, d);
                        let s = s.as_str();
                        let count = SuffixIter::new(s).count();

                        assert_eq!(count, s.chars().count());

                        assert_eq!(
                            count,
                            SuffixIter::new(s).fold(0, |n, _| n + 1)
                        );
                    }
                }
            }
        }
    }
}
