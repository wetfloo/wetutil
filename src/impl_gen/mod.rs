//! Generate implementations of different traits.

#[cfg(feature = "type-name")]
pub mod debug;
#[cfg(feature = "type-name")]
pub mod display;

#[cfg(feature = "type-name")]
#[macro_export]
#[doc(hidden)]
macro_rules! _core_fmt_common__from_type_name {
	($trait:path | $type:ty$(,)?) => {
		impl $trait for $type {
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
				f.write_str(&$crate::pretty_type_name_alloc::pretty_type_name::<Self>())
			}
		}
	};
}
