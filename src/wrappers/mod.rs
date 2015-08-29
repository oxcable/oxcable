//! Tools for wrapping devices.

mod buffered;
mod messaged;

#[doc(inline)]
pub use self::buffered::Buffered;
#[doc(inline)]
pub use self::messaged::Messaged;
