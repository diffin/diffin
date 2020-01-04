//! String suffix handling.

mod iter;
pub use iter::SuffixIter;

/// Returns an iterator over the suffixes of a `&str` or `&[T]`, treating it as
/// the first suffix if non-empty.
#[inline]
pub const fn iter<'a, S: ?Sized>(s: &'a S) -> SuffixIter<'a, S> {
    SuffixIter::new(s)
}
