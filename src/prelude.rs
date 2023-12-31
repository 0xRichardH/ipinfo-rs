/// Re-expose the `Error` type.
pub use crate::error::RequestIpinfoError;

/// Re-expose the `IpInfo` type.
pub use crate::utils::ip_info::IpInfo;

/// An alias for the `Result` type
pub type Result<T> = core::result::Result<T, RequestIpinfoError>;
