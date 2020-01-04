//! String suffix handling.

mod iter;
pub use iter::SuffixIter;

/// Returns an iterator over the suffixes of a `&str` or `&[T]`, treating it as
/// the first suffix if non-empty.
///
/// # Examples
///
/// Every iteration drops one element from the front:
///
/// ```
/// let s = "hello";
/// let mut iter = diffin::suffix::iter(s);
///
/// assert_eq!(iter.next(), Some("hello"));
/// assert_eq!(iter.next(), Some("ello"));
/// assert_eq!(iter.next(), Some("llo"));
/// assert_eq!(iter.next(), Some("lo"));
/// assert_eq!(iter.next(), Some("o"));
/// assert_eq!(iter.next(), None);
/// ```
#[inline]
pub const fn iter<'a, S: ?Sized>(s: &'a S) -> SuffixIter<'a, S> {
    SuffixIter::new(s)
}
