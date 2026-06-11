//! [`Iterator`] extensions
//!
//! This module provides [`IterExt`], containing useful methods
//! for iterators containing [`Result`] and/or [`Option`] values.

mod consume;
mod inspect;

use consume::ConsumeResultErr;
use consume::ConsumeResultOk;
use inspect::InspectError;
use inspect::InspectOk;
use inspect::InspectSome;

use crate::iter::consume::DiscardNone;
use crate::iter::consume::DiscardResultErr;
use crate::iter::consume::DiscardResultOk;

/// [`Iterator`] helpers.
///
/// See individual method documentation for more details.
pub trait IterExt: Iterator {
	/// Allows you to inspect any [`Ok`] contents without modifying the iterator.
	///
	/// This is equivalent to using [`Iterator::inspect`] and matching on [`Ok`] manually:
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Result<Value, Error>>();
	/// #
	/// # struct Error;
	/// struct Value;
	///
	/// let iter = iter.inspect(|res| if let Ok(v) = res.as_ref() {
	///     inspect_action(v);
	/// });
	///
	/// // is the same as:
	///
	/// let iter = iter.inspect_ok(|v| {
	///     inspect_action(v);
	/// });
	///
	/// fn inspect_action(_: &Value) {
	///     // some logging here...
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
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
	fn inspect_ok<T, E, F>(self, f: F) -> InspectOk<Self, F>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
		F: FnMut(&T),
	{
		InspectOk::new(self, f)
	}

	/// Allows you to inspect any [`Err`] contents without modifying the iterator.
	///
	/// This is equivalent to using [`Iterator::inspect`] and matching on [`Err`] manually:
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Result<Value, Error>>();
	/// #
	/// # struct Value;
	/// struct Error;
	///
	/// let iter = iter.inspect(|res| if let Err(e) = res.as_ref() {
	///     inspect_action(e);
	/// });
	///
	/// // is the same as:
	///
	/// let iter = iter.inspect_err(|e| {
	///     inspect_action(e);
	/// });
	///
	/// fn inspect_action(_: &Error) {
	///     // some logging here...
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
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
	fn inspect_err<T, E, F>(self, f: F) -> InspectError<Self, F>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
		F: FnMut(&E),
	{
		InspectError::new(self, f)
	}

	/// Allows you to inspect any [`Some`] contents without modifying the iterator.
	///
	/// This is equivalent to using [`Iterator::inspect`] and matching on [`Some`] manually:
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Option<Value>>();
	/// #
	/// struct Value;
	///
	/// let iter = iter.inspect(|res| if let Some(v) = res.as_ref() {
	///     inspect_action(v);
	/// });
	///
	/// // is the same as:
	///
	/// let iter = iter.inspect_some(|v| {
	///     inspect_action(v);
	/// });
	///
	/// fn inspect_action(_: &Value) {
	///     // some logging here...
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
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
	fn inspect_some<T, F>(self, f: F) -> InspectSome<Self, F>
	where
		Self: Iterator<Item = Option<T>> + Sized,
		F: FnMut(&T),
	{
		InspectSome::new(self, f)
	}

	/// Drops any [`Ok`], passing along only [`Err`] contents.
	///
	/// This is equivalent to using [`Iterator::filter_map`] and only allowing [`Err`] through:
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Result<Value, Error>>();
	/// #
	/// # struct Value;
	/// # struct Error;
	/// #
	/// let mut consumed = Vec::new();
	///
	/// for e in iter.clone().filter_map(|res| match res {
	///     Err(e) => Some(e),
	///     Ok(_) => None,
	/// }) {
	///     consumed.push(e);
	/// }
	///
	/// // is the same as:
	///
	/// for e in iter.clone().discard_ok() {
	///     consumed.push(e);
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
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
	fn discard_ok<T, E>(self) -> DiscardResultOk<Self>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
	{
		DiscardResultOk::new(self, ())
	}

	/// Drops any [`Err`], passing along only [`Ok`] contents.
	///
	/// This is equivalent to using [`Iterator::filter_map`] and only allowing [`Ok`] through:
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Result<Value, Error>>();
	/// #
	/// # struct Value;
	/// # struct Error;
	/// #
	/// let mut consumed = Vec::new();
	///
	/// for v in iter.clone().filter_map(|res| match res {
	///     Ok(v) => Some(v),
	///     Err(_) => None,
	/// }) {
	///     consumed.push(v);
	/// }
	///
	/// // is the same as:
	///
	/// for v in iter.clone().discard_err() {
	///     consumed.push(v);
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
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
	fn discard_err<T, E>(self) -> DiscardResultErr<Self>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
	{
		DiscardResultErr::new(self, ())
	}

	/// Drops any [`None`], passing along only [`Some`] contents.
	///
	/// This is equivalent to using [`Iterator::filter_map`] and only allowing [`Some`] through:
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Option<Value>>();
	/// #
	/// # struct Value;
	/// #
	/// let mut consumed = Vec::new();
	///
	/// for v in iter.clone().filter_map(|opt| opt) {
	///     consumed.push(v);
	/// }
	///
	/// // is the same as:
	///
	/// for v in iter.clone().discard_none() {
	///     consumed.push(v);
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
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

	/// Passes any contents of [`Ok`] to `f`,
	/// returning an [`Iterator`] of [`Err`] contents.
	///
	/// This is equivalent to using [`Iterator::filter_map`] and only allowing [`Err`] through,
	/// consuming any [`Ok`] in progress:
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Result<Value, Error>>();
	/// #
	/// # struct Value;
	/// # struct Error;
	/// #
	/// let mut consumed_ok = Vec::new();
	/// let mut consumed_err = Vec::new();
	///
	/// for e in iter.clone().filter_map(|res| match res {
	///     Err(e) => Some(e),
	///     Ok(v) => {
	///         consumed_ok.push(v);
	///         None
	///     }
	/// }) {
	///     consumed_err.push(e);
	/// }
	///
	/// // is the same as:
	///
	/// for e in iter.clone().consume_ok(|v| consumed_ok.push(v)) {
	///     consumed_err.push(e);
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// let results: [Result<u8, u8>; _] = [
	///     Ok(1),
	///     Err(2),
	///     Ok(3),
	///     Ok(4),
	///     Err(5),
	///     Err(6),
	/// ];
	/// let mut consumed_values = Vec::new();
	///
	/// let mut filtered_iter = results
	///     .into_iter()
	///     .consume_ok(|v| consumed_values.push(v));
	///
	/// assert_eq!(Some(2), filtered_iter.next());
	/// assert_eq!(Some(5), filtered_iter.next());
	/// assert_eq!(Some(6), filtered_iter.next());
	/// assert_eq!(None, filtered_iter.next());
	///
	/// assert_eq!(vec![1, 3, 4], consumed_values);
	/// ```
	///
	/// Note that instead of passing a dummy closure like in the following example,
	/// you should use [`discard_ok`](IterExt::discard_ok):
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Result<Value, Error>>();
	/// #
	/// # struct Value;
	/// # struct Error;
	/// #
	/// // bad!
	/// for _ in iter.clone().consume_ok(|_| ()) {}
	///
	/// // good!
	/// for _ in iter.clone().discard_ok() {}
	/// ```
	#[inline]
	fn consume_ok<T, E, F>(self, f: F) -> ConsumeResultOk<Self, F>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
		F: FnMut(T),
	{
		ConsumeResultOk::new(self, f)
	}
	/// Passes any contents of [`Err`] to `f`,
	/// returning an [`Iterator`] of [`Ok`] contents.
	///
	/// This is equivalent to using [`Iterator::filter_map`] and only allowing [`Ok`] through,
	/// consuming any [`Err`] in progress:
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Result<Value, Error>>();
	/// #
	/// # struct Value;
	/// # struct Error;
	/// #
	/// let mut consumed_ok = Vec::new();
	/// let mut consumed_err = Vec::new();
	///
	/// for v in iter.clone().filter_map(|res| match res {
	///     Ok(v) => Some(v),
	///     Err(e) => {
	///         consumed_err.push(e);
	///         None
	///     }
	/// }) {
	///     consumed_ok.push(v);
	/// }
	///
	/// // is the same as:
	///
	/// for v in iter.clone().consume_err(|e| consumed_err.push(e)) {
	///     consumed_ok.push(v);
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// let results: [Result<u8, u8>; _] = [
	///     Ok(1),
	///     Err(2),
	///     Ok(3),
	///     Ok(4),
	///     Err(5),
	///     Err(6),
	/// ];
	/// let mut consumed_values = Vec::new();
	///
	/// let mut filtered_iter = results
	///     .into_iter()
	///     .consume_err(|v| consumed_values.push(v));
	///
	/// assert_eq!(Some(1), filtered_iter.next());
	/// assert_eq!(Some(3), filtered_iter.next());
	/// assert_eq!(Some(4), filtered_iter.next());
	/// assert_eq!(None, filtered_iter.next());
	///
	/// assert_eq!(vec![2, 5, 6], consumed_values);
	/// ```
	///
	/// Note that instead of passing a dummy closure like in the the following example,
	/// you should use [`discard_err`](IterExt::discard_err):
	///
	/// ```
	/// # use wetutil::iter::IterExt as _;
	/// #
	/// # let iter = std::iter::empty::<Result<Value, Error>>();
	/// #
	/// # struct Value;
	/// # struct Error;
	/// #
	/// // bad!
	/// for _ in iter.clone().consume_err(|_| ()) {}
	///
	/// // good!
	/// for _ in iter.clone().discard_err() {}
	/// ```
	#[inline]
	fn consume_err<T, E, F>(self, f: F) -> ConsumeResultErr<Self, F>
	where
		Self: Iterator<Item = Result<T, E>> + Sized,
		F: FnMut(E),
	{
		ConsumeResultErr::new(self, f)
	}
}

impl<I> IterExt for I where I: Iterator {}
