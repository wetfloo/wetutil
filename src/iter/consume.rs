use std::iter::FusedIterator;

use crate::iter::inspect::InspectSpecialCaseFnOk;

pub type ConsumeOk<N, F> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnOk<F>>;
// pub type ConsumeError<N, F> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnError<F>>;
// pub type ConsumeSome<N, F> = ConsumeSpecialCase<N, ConsumeSpecialCaseFnSome<F>>;

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
		loop {
			match self.f.call(self.inner_iter.next()?) {
				// Consume the original value,
				// trying to find next that won't be consumed.
				None => continue,
				// Found the value that didn't get consumed.
				v => return v,
			}
		}
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
		loop {
			match self
				.f
				.call(self.inner_iter.next_back()?)
			{
				// Consume the original value,
				// trying to find next that won't be consumed.
				None => continue,
				// Found the value that didn't get consumed.
				v => return v,
			}
		}
	}
}

impl<T, N, F> FusedIterator for ConsumeSpecialCase<N, F>
where
	N: FusedIterator<Item = T>,
	F: ConsumeSpecialCaseFn<N::Item>,
{
}

impl<N, F> ConsumeOk<N, F> {
	#[inline]
	pub(crate) fn new<T, E>(inner_iter: N, f: F) -> Self {
		Self {
			inner_iter,
			f: ConsumeSpecialCaseFnOk(f),
		}
	}
}

pub(crate) trait ConsumeSpecialCaseFn<T> {
	type Unconsumed;

	fn call(&mut self, val: T) -> Option<Self::Unconsumed>;
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
