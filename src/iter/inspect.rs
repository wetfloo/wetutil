use std::iter::FusedIterator;

pub type InspectOk<N, F> = InspectSpecialCase<N, InspectSpecialCaseFnOk<F>>;
pub type InspectError<N, F> = InspectSpecialCase<N, InspectSpecialCaseFnError<F>>;
pub type InspectSome<N, F> = InspectSpecialCase<N, InspectSpecialCaseFnSome<F>>;

pub struct InspectSpecialCase<N, F> {
	inner_iter: N,
	/// Function that will be called on every [Iterator::next] call.
	/// Should only be called if deemed appropriate.
	f: F,
}

impl<N, F> Iterator for InspectSpecialCase<N, F>
where
	N: Iterator,
	F: InspectSpecialCaseFn<N::Item>,
{
	type Item = N::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner_iter
			.next()
			.inspect(|val| self.f.call(val))
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner_iter.size_hint()
	}
}

impl<N, T, E, F> DoubleEndedIterator for InspectSpecialCase<N, F>
where
	N: DoubleEndedIterator<Item = Result<T, E>>,
	F: InspectSpecialCaseFn<N::Item>,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		self.inner_iter
			.next_back()
			.inspect(|val| self.f.call(val))
	}
}

impl<N, T, E, F> ExactSizeIterator for InspectSpecialCase<N, F>
where
	N: ExactSizeIterator<Item = Result<T, E>>,
	F: InspectSpecialCaseFn<N::Item>,
{
	#[inline]
	fn len(&self) -> usize {
		self.inner_iter.len()
	}
}

impl<N, T, E, F> FusedIterator for InspectError<N, F>
where
	N: FusedIterator<Item = Result<T, E>>,
	F: FnMut(&E),
{
}

impl<N, F> InspectOk<N, F> {
	#[inline]
	pub(crate) fn new<T, E>(inner_iter: N, f: F) -> Self
	where
		N: Iterator<Item = Result<T, E>>,
		F: FnMut(&T),
	{
		Self {
			inner_iter,
			f: InspectSpecialCaseFnOk(f),
		}
	}
}

impl<N, F> InspectError<N, F> {
	#[inline]
	pub(crate) fn new<T, E>(inner_iter: N, f: F) -> Self
	where
		N: Iterator<Item = Result<T, E>>,
		F: FnMut(&E),
	{
		Self {
			inner_iter,
			f: InspectSpecialCaseFnError(f),
		}
	}
}

impl<N, F> InspectSome<N, F> {
	#[inline]
	pub(crate) fn new<T>(inner_iter: N, f: F) -> Self
	where
		N: Iterator<Item = Option<T>>,
		F: FnMut(&T),
	{
		Self {
			inner_iter,
			f: InspectSpecialCaseFnSome(f),
		}
	}
}

pub(crate) trait InspectSpecialCaseFn<T> {
	fn call(&mut self, val: &T);
}

pub struct InspectSpecialCaseFnOk<F>(F);

impl<T, E, F> InspectSpecialCaseFn<Result<T, E>> for InspectSpecialCaseFnOk<F>
where
	F: FnMut(&T),
{
	#[inline]
	fn call(&mut self, val: &Result<T, E>) {
		if let Ok(v) = val.as_ref() {
			self.0(v)
		}
	}
}

pub struct InspectSpecialCaseFnError<F>(F);

impl<T, E, F> InspectSpecialCaseFn<Result<T, E>> for InspectSpecialCaseFnError<F>
where
	F: FnMut(&E),
{
	#[inline]
	fn call(&mut self, val: &Result<T, E>) {
		if let Err(e) = val.as_ref() {
			self.0(e)
		}
	}
}

pub struct InspectSpecialCaseFnSome<F>(F);

impl<T, F> InspectSpecialCaseFn<Option<T>> for InspectSpecialCaseFnSome<F>
where
	F: FnMut(&T),
{
	#[inline]
	fn call(&mut self, val: &Option<T>) {
		if let Some(v) = val.as_ref() {
			self.0(v)
		}
	}
}
