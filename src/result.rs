//! [`Result`] extensions
//!
//! This module provides extension traits, containing useful methods
//! for converting between [`Result`] value types.

/// Convert [`Result`]'s [`Ok`] value into another
/// using `T2`'s [`From<T1>`] implementation leaving [`Err`] untouched.
///
/// For a nicer, more convenient conversion, see [`ResultOkInto`].
///
/// # Examples
///
/// Using [`From`]:
///
/// ```
/// let r1: Result<u8, ()> = Ok(42);
/// let r2: Result<u16, _> = r1.map(From::from);
///
/// assert_eq!(Ok(42u16), r2);
/// ```
///
/// Using [`ResultOkFrom`]:
///
/// ```
/// # use wetutil::result::ResultOkFrom as _;
/// let r1: Result<u8, ()> = Ok(42);
/// let r2: Result<u16, _> = Result::ok_from(r1);
///
/// assert_eq!(Ok(42u16), r2);
/// ```
pub trait ResultOkFrom<T1, T2, E1> {
	/// Performs the conversion between [`Ok`] values.
	fn ok_from(val: Result<T1, E1>) -> Self;
}

impl<T1, T2, E1> ResultOkFrom<T1, T2, E1> for Result<T2, E1>
where
	T2: From<T1>,
{
	#[inline]
	fn ok_from(val: Result<T1, E1>) -> Self {
		val.map(From::from)
	}
}

/// Convert [`Result`]'s [`Ok`] value into another
/// using `T2`'s [`From<T1>`] implementation leaving [`Err`] untouched.
///
/// This trait to [`ResultOkFrom`] is what [`Into`] is to [`From`].
///
/// # Examples
///
/// Using [`Into`]:
///
/// ```
/// let r1: Result<u8, ()> = Ok(42);
/// let r2: Result<u16, _> = r1.map(|v| v.into());
///
/// assert_eq!(Ok(42u16), r2);
/// ```
///
/// Using [`ResultOkInto`]:
///
/// ```
/// # use wetutil::result::ResultOkInto as _;
/// let r1: Result<u8, ()> = Ok(42);
/// let r2: Result<u16, _> = r1.ok_into();
///
/// assert_eq!(Ok(42u16), r2);
/// ```
pub trait ResultOkInto<T1, T2, E1> {
	/// Performs the conversion between [`Ok`] values.
	fn ok_into(self) -> Result<T2, E1>;
}

impl<T1, T2, E1> ResultOkInto<T1, T2, E1> for Result<T1, E1>
where
	T2: From<T1>,
{
	#[inline]
	fn ok_into(self) -> Result<T2, E1> {
		ResultOkFrom::ok_from(self)
	}
}

/// Convert [`Result`]'s [`Err`] value into another
/// using `E2`'s [`From<E1>`] implementation leaving [`Err`] untouched.
///
/// For a nicer, more convenient conversion, see [`ResultErrInto`].
///
/// # Examples
///
/// Using [`From`]:
///
/// ```
/// let r1: Result<(), u8> = Err(42);
/// let r2: Result<_, u16> = r1.map_err(From::from);
///
/// assert_eq!(Err(42u16), r2);
/// ```
///
/// Using [`ResultErrFrom`]:
///
/// ```
/// # use wetutil::result::ResultErrFrom as _;
/// let r1: Result<(), u8> = Err(42);
/// let r2: Result<_, u16> = Result::err_from(r1);
///
/// assert_eq!(Err(42u16), r2);
/// ```
pub trait ResultErrFrom<T1, E1, E2> {
	/// Performs the conversion between [`Err`] values.
	fn err_from(val: Result<T1, E1>) -> Self;
}

impl<T1, E1, E2> ResultErrFrom<T1, E1, E2> for Result<T1, E2>
where
	E2: From<E1>,
{
	#[inline]
	fn err_from(val: Result<T1, E1>) -> Self
	where
		E2: From<E1>,
	{
		val.map_err(From::from)
	}
}

/// Convert [`Result`]'s [`Err`] value into another
/// using `E2`'s [`From<E1>`] implementation leaving [`Ok`] untouched.
///
/// This trait to [`ResultErrFrom`] is what [`Into`] is to [`From`].
///
/// # Examples
///
/// Using [`Into`]:
///
/// ```
/// let r1: Result<(), u8> = Err(42);
/// let r2: Result<_, u16> = r1.map_err(|v| v.into());
///
/// assert_eq!(Err(42u16), r2);
/// ```
///
/// Using [`ResultErrInto`]:
///
/// ```
/// # use wetutil::result::ResultErrInto as _;
/// let r1: Result<(), u8> = Err(42);
/// let r2: Result<_, u16> = r1.err_into();
///
/// assert_eq!(Err(42u16), r2);
/// ```
pub trait ResultErrInto<T1, E1, E2> {
	/// Performs the conversion between [`Err`] values.
	fn err_into(self) -> Result<T1, E2>;
}

impl<T1, E1, E2> ResultErrInto<T1, E1, E2> for Result<T1, E1>
where
	E2: From<E1>,
{
	#[inline]
	fn err_into(self) -> Result<T1, E2> {
		ResultErrFrom::err_from(self)
	}
}

/// Convert [`Result`]'s [`Ok`] value into another
/// using `T2`'s [`From<T1>`] implementation,
/// and also [`Err`] value into another
/// using `E2`'s [`From<E1>`] implementation.
///
/// For a nicer, more convenient conversion, see [`ResultValueInto`].
///
/// # Examples
///
/// Using [`From`]:
///
/// ```
/// let r1: Result<u8, &str> = Ok(42);
/// let r2: Result<u16, String> = r1.map(From::from).map_err(From::from);
///
/// assert_eq!(Ok(42u16), r2);
/// ```
///
/// Using [`ResultValueFrom`]:
///
/// ```
/// # use wetutil::result::ResultValueFrom as _;
/// let r1: Result<u8, &str> = Ok(42);
/// let r2: Result<u16, String> = Result::val_from(r1);
///
/// assert_eq!(Ok(42u16), r2);
/// ```
pub trait ResultValueFrom<T1, T2, E1, E2> {
	/// Performs the conversion between [`Ok`] and [`Err`] values.
	fn val_from(val: Result<T1, E1>) -> Self;
}

impl<T1, T2, E1, E2> ResultValueFrom<T1, T2, E1, E2> for Result<T2, E2>
where
	T2: From<T1>,
	E2: From<E1>,
{
	#[inline]
	fn val_from(val: Result<T1, E1>) -> Self {
		val.err_into().ok_into()
	}
}

/// Convert [`Result`]'s [`Ok`] value into another
/// using `T2`'s [`From<T1>`] implementation,
/// and also [`Err`] value into another
/// using `E2`'s [`From<E1>`] implementation.
///
/// This trait to [`ResultValueFrom`] is what [`Into`] is to [`From`].
///
/// # Examples
///
/// Using [`Into`]:
///
/// ```
/// let r1: Result<u8, &str> = Ok(42);
/// let r2: Result<u16, String> = r1.map(|v| v.into()).map_err(|e| e.into());
///
/// assert_eq!(Ok(42u16), r2);
/// ```
///
/// Using [`ResultValueInto`]:
///
/// ```
/// # use wetutil::result::ResultValueInto as _;
/// let r1: Result<u8, &str> = Ok(42);
/// let r2: Result<u16, String> = r1.val_into();
///
/// assert_eq!(Ok(42u16), r2);
/// ```
pub trait ResultValueInto<T1, T2, E1, E2> {
	/// Performs the conversion between [`Ok`] and [`Err`] values.
	fn val_into(self) -> Result<T2, E2>;
}

impl<T1, T2, E1, E2> ResultValueInto<T1, T2, E1, E2> for Result<T1, E1>
where
	T2: From<T1>,
	E2: From<E1>,
{
	#[inline]
	fn val_into(self) -> Result<T2, E2> {
		ResultValueFrom::val_from(self)
	}
}
