use std::mem;

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
        (self.suffix.len(), None)
    }
}

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
}

impl<'a, T> ExactSizeIterator for SuffixIter<'a, [T]> {
    #[inline]
    fn len(&self) -> usize {
        self.suffix.len()
    }
}

impl<'a, S: ?Sized> SuffixIter<'a, S> {
    /// Creates a new instance from `suffix`, treating it as the first suffix if
    /// non-empty.
    #[inline]
    pub const fn new(suffix: &'a S) -> Self {
        Self { suffix }
    }
}
