use std::iter::FusedIterator;

pub type DiscardOk<N> = DiscardSpecialCase<N, DiscardSpecialCaseFnOk>;
pub type DiscardError<N> = DiscardSpecialCase<N, DiscardSpecialCaseFnError>;
pub type DiscardNone<N> = DiscardSpecialCase<N, DiscardSpecialCaseFnNone>;

pub struct DiscardSpecialCase<N, F> {
	inner_iter: N,
	f: F,
}

impl<N> DiscardOk<N> {
	#[inline]
	pub(crate) fn new<T, E>(inner_iter: N) -> Self
	where
		N: Iterator<Item = Result<T, E>>,
	{
		Self {
			inner_iter,
			f: DiscardSpecialCaseFnOk,
		}
	}
}

impl<N> DiscardError<N> {
	#[inline]
	pub(crate) fn new<T, E>(inner_iter: N) -> Self
	where
		N: Iterator<Item = Result<T, E>>,
	{
		Self {
			inner_iter,
			f: DiscardSpecialCaseFnError,
		}
	}
}

impl<N> DiscardNone<N> {
	#[inline]
	pub(crate) fn new<T>(inner_iter: N) -> Self
	where
		N: Iterator<Item = Option<T>>,
	{
		Self {
			inner_iter,
			f: DiscardSpecialCaseFnNone,
		}
	}
}

impl<N, F> Iterator for DiscardSpecialCase<N, F>
where
	N: Iterator,
	F: DiscardSpecialCaseFn<N::Item>,
{
	type Item = F::Out;

	fn next(&mut self) -> Option<Self::Item> {
		for val in self.inner_iter.by_ref() {
			match self.f.call(val) {
				Some(mapped) => return Some(mapped),
				None => continue,
			}
		}
		None
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, self.inner_iter.size_hint().1)
	}
}

impl<N, T, E, F> DoubleEndedIterator for DiscardSpecialCase<N, F>
where
	N: DoubleEndedIterator<Item = Result<T, E>>,
	F: DiscardSpecialCaseFn<N::Item>,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		for val in self.inner_iter.by_ref().rev() {
			match self.f.call(val) {
				Some(mapped) => return Some(mapped),
				None => continue,
			}
		}
		None
	}
}

impl<N, T, E> FusedIterator for DiscardError<N> where N: FusedIterator<Item = Result<T, E>> {}

pub trait DiscardSpecialCaseFn<T> {
	type Out;

	fn call(&mut self, val: T) -> Option<Self::Out>;
}

pub struct DiscardSpecialCaseFnOk;

impl<T, E> DiscardSpecialCaseFn<Result<T, E>> for DiscardSpecialCaseFnOk {
	type Out = E;

	#[inline]
	fn call(&mut self, val: Result<T, E>) -> Option<Self::Out> {
		val.err()
	}
}

pub struct DiscardSpecialCaseFnError;

impl<T, E> DiscardSpecialCaseFn<Result<T, E>> for DiscardSpecialCaseFnError {
	type Out = T;

	#[inline]
	fn call(&mut self, val: Result<T, E>) -> Option<Self::Out> {
		val.ok()
	}
}

pub struct DiscardSpecialCaseFnNone;

impl<T> DiscardSpecialCaseFn<Option<T>> for DiscardSpecialCaseFnNone {
	type Out = T;

	#[inline]
	fn call(&mut self, val: Option<T>) -> Option<Self::Out> {
		val
	}
}
