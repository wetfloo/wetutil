//! `wetutil`: One-stop crate for all the [Rust](https://rust-lang.org/)
//! features that I'm missing.
//!
//! This crate implements some useful extension traits
//! for things like [`Result`] and [`Iterator`].
//!
//! It's mostly a mish-mash of things that
//! I'm tired of writing in more tedious ways.

#![no_std]
#![deny(unreachable_pub)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "type-name")]
#[doc(hidden)]
pub use pretty_type_name_alloc;

pub mod convert;
pub mod impl_gen;
pub mod iter;

/// This crate's prelude
///
/// It re-exports extension traits.
pub mod prelude {
	pub use crate::convert::option::OptionValueFrom;
	pub use crate::convert::option::OptionValueInto;
	pub use crate::convert::result::ResultErrFrom;
	pub use crate::convert::result::ResultErrInto;
	pub use crate::convert::result::ResultOkFrom;
	pub use crate::convert::result::ResultOkInto;
	pub use crate::convert::result::ResultValueFrom;
	pub use crate::convert::result::ResultValueInto;
	pub use crate::iter::IterExt;
}
