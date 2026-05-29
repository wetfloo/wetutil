mod consume;
mod discard;
mod inspect;

use crate::iter::discard::{DiscardError, DiscardNone, DiscardOk};
use consume::{ConsumeError, ConsumeOk};
use inspect::{InspectError, InspectOk, InspectSome};

pub trait IterExt: Iterator {
	/// Allows you to inspect any [Ok] contents without modifying the iterator.
	///
	/// ```
	/// # use wetutil::iter::IterExt;
	/// let results: [Result<u8, u8>; _] = [
	///     Ok(1),
	///     Err(2),
	///     Ok(3),
	///     Ok(4),
	///     Err(5),
	///     Err(6),
	/// ];
	/// let mut inspect_storage = Vec::new();
	///
	/// let mut iter = results
	///     .into_iter()
	///     .inspect_ok(|&v| inspect_storage.push(v));
	///
	/// assert_eq!(Some(Ok(1)), iter.next());
	/// assert_eq!(Some(Err(2)), iter.next());
	/// assert_eq!(Some(Ok(3)), iter.next());
	/// assert_eq!(Some(Ok(4)), iter.next());
	/// assert_eq!(Some(Err(5)), iter.next());
	/// assert_eq!(Some(Err(6)), iter.next());
	/// assert_eq!(None, iter.next());
	///
	/// assert_eq!(vec![1, 3, 4], inspect_storage);
	/// ```
	#[inline]
	fn inspect_ok<T, E, F>(self, inspect: F) -> InspectOk<Self, F>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
		F: FnMut(&T),
	{
		InspectOk::new(self, inspect)
	}

	/// Allows you to inspect any [Err] contents without modifying the iterator.
	///
	/// ```
	/// # use wetutil::iter::IterExt;
	/// let results: [Result<u8, u8>; _] = [
	///     Ok(1),
	///     Err(2),
	///     Ok(3),
	///     Ok(4),
	///     Err(5),
	///     Err(6),
	/// ];
	/// let mut inspect_storage = Vec::new();
	///
	/// let mut iter = results
	///     .into_iter()
	///     .inspect_err(|&v| inspect_storage.push(v));
	///
	/// assert_eq!(Some(Ok(1)), iter.next());
	/// assert_eq!(Some(Err(2)), iter.next());
	/// assert_eq!(Some(Ok(3)), iter.next());
	/// assert_eq!(Some(Ok(4)), iter.next());
	/// assert_eq!(Some(Err(5)), iter.next());
	/// assert_eq!(Some(Err(6)), iter.next());
	/// assert_eq!(None, iter.next());
	///
	/// assert_eq!(vec![2, 5, 6], inspect_storage);
	/// ```
	#[inline]
	fn inspect_err<T, E, F>(self, inspect: F) -> InspectError<Self, F>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
		F: FnMut(&E),
	{
		InspectError::new(self, inspect)
	}

	/// Allows you to inspect any [Some] contents without modifying the iterator.
	///
	/// ```
	/// # use wetutil::iter::IterExt;
	/// let results: [Option<u8>; _] = [
	///     None,
	///     Some(2),
	///     None,
	///     None,
	///     Some(5),
	///     Some(6),
	/// ];
	/// let mut inspect_storage = Vec::new();
	///
	/// let mut iter = results
	///     .into_iter()
	///     .inspect_some(|&v| inspect_storage.push(v));
	///
	/// assert_eq!(Some(None), iter.next());
	/// assert_eq!(Some(Some(2)), iter.next());
	/// assert_eq!(Some(None), iter.next());
	/// assert_eq!(Some(None), iter.next());
	/// assert_eq!(Some(Some(5)), iter.next());
	/// assert_eq!(Some(Some(6)), iter.next());
	/// assert_eq!(None, iter.next());
	///
	/// assert_eq!(vec![2, 5, 6], inspect_storage);
	/// ```
	#[inline]
	fn inspect_some<T, F>(self, inspect: F) -> InspectSome<Self, F>
	where
		Self: Iterator<Item = Option<T>> + Sized,
		F: FnMut(&T),
	{
		InspectSome::new(self, inspect)
	}

	/// Drops any [Ok], passing along only [Err] contents.
	///
	/// ```
	/// # use wetutil::iter::IterExt;
	/// let results: [Result<u8, u8>; _] = [
	///     Ok(1),
	///     Err(2),
	///     Ok(3),
	///     Ok(4),
	///     Err(5),
	///     Err(6),
	/// ];
	///
	/// let mut filtered_iter = results
	///     .into_iter()
	///     .discard_ok();
	///
	/// assert_eq!(Some(2), filtered_iter.next());
	/// assert_eq!(Some(5), filtered_iter.next());
	/// assert_eq!(Some(6), filtered_iter.next());
	/// assert_eq!(None, filtered_iter.next());
	/// ```
	#[inline]
	fn discard_ok<T, E>(self) -> DiscardOk<Self>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
	{
		DiscardOk::new(self)
	}

	/// Drops any [Err], passing along only [Ok] contents.
	///
	/// ```
	/// # use wetutil::iter::IterExt;
	/// let results: [Result<u8, u8>; _] = [
	///     Ok(1),
	///     Err(2),
	///     Ok(3),
	///     Ok(4),
	///     Err(5),
	///     Err(6),
	/// ];
	///
	/// let mut filtered_iter = results
	///     .into_iter()
	///     .discard_err();
	///
	/// assert_eq!(Some(1), filtered_iter.next());
	/// assert_eq!(Some(3), filtered_iter.next());
	/// assert_eq!(Some(4), filtered_iter.next());
	/// assert_eq!(None, filtered_iter.next());
	/// ```
	#[inline]
	fn discard_err<T, E>(self) -> DiscardError<Self>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
	{
		DiscardError::new(self)
	}

	/// Drops any [None], passing along only [Some] contents.
	///
	/// ```
	/// # use wetutil::iter::IterExt;
	/// let results: [Option<u8>; _] = [
	///     None,
	///     Some(2),
	///     None,
	///     None,
	///     Some(5),
	///     Some(6),
	/// ];
	///
	/// let mut filtered_iter = results
	///     .into_iter()
	///     .discard_none();
	///
	/// assert_eq!(Some(2), filtered_iter.next());
	/// assert_eq!(Some(5), filtered_iter.next());
	/// assert_eq!(Some(6), filtered_iter.next());
	/// assert_eq!(None, filtered_iter.next());
	/// ```
	#[inline]
	fn discard_none<T>(self) -> DiscardNone<Self>
	where
		Self: Iterator<Item = Option<T>> + Sized,
	{
		DiscardNone::new(self)
	}

	#[inline]
	fn consume_ok<T, E, F>(self, f: F) -> ConsumeOk<Self, F>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
		F: FnMut(T),
	{
		ConsumeOk::new(self, f)
	}

	#[inline]
	fn consume_err<T, E, F>(self, f: F) -> ConsumeError<Self, F>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
		F: FnMut(E),
	{
		ConsumeError::new(self, f)
	}
}

impl<I> IterExt for I where I: Iterator {}
