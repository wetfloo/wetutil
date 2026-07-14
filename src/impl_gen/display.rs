//! [`Display`](core::fmt::Display) implementation generation.

/// Generate [`Display`](core::fmt::Display) implementation from the type name.
///
/// Example:
/// ```
/// # use wetutil::impl_gen::display::from_type_name;
/// struct MyType;
///
/// from_type_name!(MyType);
///
/// assert_eq!("MyType", format!("{}", MyType).as_str());
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! _core_fmt_display__from_type_name {
	($type:ty$(,)?) => {
		$crate::_core_fmt_common__from_type_name!(::core::fmt::Display | $type);
	};
}

#[cfg_attr(docsrs, doc(cfg(feature = "type-name")))]
#[doc(inline)]
pub use _core_fmt_display__from_type_name as from_type_name;
