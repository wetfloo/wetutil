//! `wetutil`: One-stop crate for all the rust features that I'm missing.
//!
//! This crate implements some useful extension traits
//! for things like [`Result`] and [`Iterator`].
//! It's mostly a mish-mash of things that I'm tired of writing in more tedious ways.

#![deny(unreachable_pub)]

pub mod iter;
pub mod result;

/// This crate's prelude
///
/// It re-exports extension traits.
pub mod prelude {
	pub use crate::iter::IterExt;

	pub use crate::result::ResultErrFrom;
	pub use crate::result::ResultOkFrom;
	pub use crate::result::ResultValueFrom;

	pub use crate::result::ResultErrInto;
	pub use crate::result::ResultOkInto;
	pub use crate::result::ResultValueInto;
}
