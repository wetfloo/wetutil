//! [`Debug`](core::fmt::Debug) implementation generation.

/// Generate [`Debug`](core::fmt::Debug) implementation from the type name.
///
/// Example:
/// ```
/// # use wetutil::impl_gen::debug::from_type_name;
/// struct MyType;
///
/// from_type_name!(MyType);
///
/// assert_eq!("MyType", format!("{:?}", MyType).as_str());
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! _core_fmt_debug__from_type_name {
	($type:ty$(,)?) => {
		impl ::core::fmt::Debug for $type {
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
				f.write_str(&$crate::pretty_type_name_alloc::pretty_type_name::<Self>())
			}
		}
	};
}

#[cfg_attr(docsrs, doc(cfg(feature = "type-name")))]
#[doc(inline)]
pub use _core_fmt_debug__from_type_name as from_type_name;
