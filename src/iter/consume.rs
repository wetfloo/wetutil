use std::iter::FusedIterator;

pub type ConsumeOk<N, F> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnOk<F>>;
pub type ConsumeError<N, F> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnError<F>>;
pub type ConsumeNone<N> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnNone>;

pub type DiscardOk<N> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnOk<()>>;
pub type DiscardError<N> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnError<()>>;
pub type DiscardNone<N> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnNone>;

pub struct ConsumeSpecialCase<N, F> {
	inner_iter: N,
	/// Function that will be called on every [Iterator::next] call.
	/// Should only be called if deemed appropriate.
	f: F,
}

impl<N, F> Iterator for ConsumeSpecialCase<N, F>
where
	N: Iterator,
	F: ConsumeSpecialCaseFn<N::Item>,
{
	type Item = F::Unconsumed;

	fn next(&mut self) -> Option<Self::Item> {
		for val in self.inner_iter.by_ref() {
			match self.f.call(val) {
				// Consume the original value,
				// trying to find next that won't be consumed.
				None => continue,
				// Found the value that didn't get consumed.
				v => return v,
			}
		}

		None
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, self.inner_iter.size_hint().1)
	}
}

impl<T, N, F> DoubleEndedIterator for ConsumeSpecialCase<N, F>
where
	N: DoubleEndedIterator<Item = T>,
	F: ConsumeSpecialCaseFn<N::Item>,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		for val in self.inner_iter.by_ref().rev() {
			match self.f.call(val) {
				// Consume the original value,
				// trying to find next that won't be consumed.
				None => continue,
				// Found the value that didn't get consumed.
				v => return v,
			}
		}

		None
	}
}

impl<T, N, F> FusedIterator for ConsumeSpecialCase<N, F>
where
	N: FusedIterator<Item = T>,
	F: ConsumeSpecialCaseFn<N::Item>,
{
}

pub trait ConsumeSpecialCaseFn<T> {
	type Unconsumed;

	fn call(&mut self, val: T) -> Option<Self::Unconsumed>;
}

impl<N, F> ConsumeOk<N, F> {
	#[inline]
	pub(crate) fn new<T, E>(inner_iter: N, f: F) -> Self
	where
		N: Iterator<Item = Result<T, E>>,
	{
		Self {
			inner_iter,
			f: ConsumeSpecialCaseFnOk(f),
		}
	}
}

pub struct ConsumeSpecialCaseFnOk<F>(F);

impl<T, E, F> ConsumeSpecialCaseFn<Result<T, E>> for ConsumeSpecialCaseFnOk<F>
where
	F: FnMut(T),
{
	type Unconsumed = E;

	#[inline]
	fn call(&mut self, val: Result<T, E>) -> Option<Self::Unconsumed> {
		match val {
			Ok(v) => {
				self.0(v);
				None
			},
			Err(e) => Some(e),
		}
	}
}

impl<T, E> ConsumeSpecialCaseFn<Result<T, E>> for ConsumeSpecialCaseFnOk<()> {
	type Unconsumed = E;

	#[inline]
	fn call(&mut self, val: Result<T, E>) -> Option<Self::Unconsumed> {
		val.err()
	}
}

impl<N, F> ConsumeError<N, F> {
	#[inline]
	pub(crate) fn new<T, E>(inner_iter: N, f: F) -> Self
	where
		N: Iterator<Item = Result<T, E>>,
	{
		Self {
			inner_iter,
			f: ConsumeSpecialCaseFnError(f),
		}
	}
}

pub struct ConsumeSpecialCaseFnError<F>(F);

impl<T, E, F> ConsumeSpecialCaseFn<Result<T, E>> for ConsumeSpecialCaseFnError<F>
where
	F: FnMut(E),
{
	type Unconsumed = T;

	#[inline]
	fn call(&mut self, val: Result<T, E>) -> Option<Self::Unconsumed> {
		match val {
			Ok(v) => Some(v),
			Err(e) => {
				self.0(e);
				None
			},
		}
	}
}

impl<T, E> ConsumeSpecialCaseFn<Result<T, E>> for ConsumeSpecialCaseFnError<()> {
	type Unconsumed = T;

	#[inline]
	fn call(&mut self, val: Result<T, E>) -> Option<Self::Unconsumed> {
		val.ok()
	}
}

impl<N> ConsumeNone<N> {
	#[inline]
	pub(crate) fn new<T>(inner_iter: N) -> Self
	where
		N: Iterator<Item = Option<T>>,
	{
		Self {
			inner_iter,
			f: ConsumeSpecialCaseFnNone,
		}
	}
}

pub struct ConsumeSpecialCaseFnNone;

impl<T> ConsumeSpecialCaseFn<Option<T>> for ConsumeSpecialCaseFnNone {
	type Unconsumed = T;

	#[inline]
	fn call(&mut self, val: Option<T>) -> Option<Self::Unconsumed> {
		val
	}
}
