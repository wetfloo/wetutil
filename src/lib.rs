pub mod iter;
pub mod result;

pub mod prelude {
	pub use crate::iter::IterExt;
	pub use crate::result::{ResultErrInto, ResultOkInto, ResultValueInto};
}
