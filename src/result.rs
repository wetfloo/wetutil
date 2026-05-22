pub trait ResultValueFrom<T1, T2, E1, E2> {
	/// Converts both [Ok] and [Err] values.
	///
	/// For a nicer, more convenient conversion, see [ResultValueInto].
	///
	/// # Examples
	///
	/// Using [From]:
	///
	/// ```
	/// let r1: Result<u8, &str> = Ok(42);
	/// let r2: Result<u16, String> = r1.map(From::from).map_err(From::from);
	///
	/// assert_eq!(Ok(42u16), r2);
	/// ```
	///
	/// Using [ResultValueFrom]:
	///
	/// ```
	/// # use wetutil::result::ResultValueFrom as _;
	/// let r1: Result<u8, &str> = Ok(42);
	/// let r2: Result<u16, String> = Result::val_from(r1);
	///
	/// assert_eq!(Ok(42u16), r2);
	/// ```
	fn val_from(val: Result<T1, E1>) -> Self;
}

impl<T1, T2, E1, E2> ResultValueFrom<T1, T2, E1, E2> for Result<T2, E2>
where
	T2: From<T1>,
	E2: From<E1>,
{
	fn val_from(val: Result<T1, E1>) -> Self {
		val.map(|v| v.into())
			.map_err(|e| e.into())
	}
}

pub trait ResultValueInto<T1, T2, E1, E2> {
	/// Converts both [Ok] and [Err] values.
	///
	/// # Examples
	///
	/// Using [Into]:
	///
	/// ```
	/// let r1: Result<u8, &str> = Ok(42);
	/// let r2: Result<u16, String> = r1.map(|v| v.into()).map_err(|e| e.into());
	///
	/// assert_eq!(Ok(42u16), r2);
	/// ```
	///
	/// Using [ResultValueInto]:
	///
	/// ```
	/// # use wetutil::result::ResultValueInto as _;
	/// let r1: Result<u8, &str> = Ok(42);
	/// let r2: Result<u16, String> = r1.val_into();
	///
	/// assert_eq!(Ok(42u16), r2);
	/// ```
	fn val_into(self) -> Result<T2, E2>;
}

impl<T1, T2, E1, E2> ResultValueInto<T1, T2, E1, E2> for Result<T1, E1>
where
	T2: From<T1>,
	E2: From<E1>,
{
	fn val_into(self) -> Result<T2, E2> {
		Result::val_from(self)
	}
}
