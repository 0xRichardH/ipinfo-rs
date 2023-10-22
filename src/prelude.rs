/// Re-expose the `Error` type.
pub use crate::error::RequestIpinfoError;

/// An alias for the `Result` type
pub type Result<T> = core::result::Result<T, RequestIpinfoError>;
