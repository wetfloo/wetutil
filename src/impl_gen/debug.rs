/// Generate a debug implementation from the type name.
///
/// Examples:
/// ```
/// # use wetutil::impl_gen::debug::from_type_name;
/// struct MyType;
///
/// from_type_name!(MyType);
///
/// assert_eq!("MyType", format!("{:?}", MyType).as_str());
/// ```
#[macro_export]
macro_rules! from_type_name {
	($type:ty$(,)?) => {
		impl ::core::fmt::Debug for $type {
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
				f.write_str(&::pretty_type_name::pretty_type_name::<
					Self,
				>())
			}
		}
	};
}

pub use from_type_name;
