//! [`Option`] extensions
//!
//! This module provides extension traits, containing useful methods
//! for converting between [`Some`] value types.

/// Convert [`Option`]'s [`Some`] value into another
/// using `T2`'s [`From<T1>`] implementation.
///
/// For a nicer, more convenient conversion, see [`OptionValueInto`].
///
/// # Examples
///
/// Using [`From`]:
///
/// ```
/// let o1: Option<u8> = Some(42);
/// let o2: Option<u16> = o1.map(From::from);
///
/// assert_eq!(Some(42u16), o2);
/// ```
///
/// Using [`OptionValueFrom`]:
///
/// ```
/// # use wetutil::convert::option::OptionValueFrom as _;
/// let o1: Option<u8> = Some(42);
/// let o2: Option<u16> = Option::val_from(o1);
///
/// assert_eq!(Some(42u16), o2);
/// ```
pub trait OptionValueFrom<T1, T2> {
	/// Performs the conversion between [`Some`] values.
	/// For more details, see the [trait's](OptionValueFrom) documentation.
	fn val_from(val: Option<T1>) -> Self;
}

impl<T1, T2> OptionValueFrom<T1, T2> for Option<T2>
where
	T2: From<T1>,
{
	#[inline]
	fn val_from(val: Option<T1>) -> Self {
		val.map(From::from)
	}
}

/// Convert [`Option`]'s [`Some`] value into another
/// using `T2`'s [`From<T1>`] implementation.
///
/// This trait to [`OptionValueFrom`] is what [`Into`] is to [`From`].
///
/// # Examples
///
/// Using [`Into`]:
///
/// ```
/// let o1: Option<u8> = Some(42);
/// let o2: Option<u16> = o1.map(|v| v.into());
///
/// assert_eq!(Some(42u16), o2);
/// ```
///
/// Using [`OptionValueInto`]:
///
/// ```
/// # use wetutil::convert::option::OptionValueInto as _;
/// let o1: Option<u8> = Some(42);
/// let o2: Option<u16> = o1.val_into();
///
/// assert_eq!(Some(42u16), o2);
/// ```
pub trait OptionValueInto<T1, T2> {
	/// Performs the conversion between [`Some`] values.
	/// For more details, see the [trait's](OptionValueInto) documentation.
	fn val_into(self) -> Option<T2>;
}

impl<T1, T2> OptionValueInto<T1, T2> for Option<T1>
where
	T2: From<T1>,
{
	#[inline]
	fn val_into(self) -> Option<T2> {
		OptionValueFrom::val_from(self)
	}
}
