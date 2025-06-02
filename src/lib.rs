mod core;
mod level;
mod timestamp;

pub use crate::core::DynLogger;
pub use crate::level::{current_level_filter, init, set_level};
pub use crate::timestamp::format_iso8601_timestamp;

#[cfg(feature = "file-trigger")]
pub mod trigger;
#[cfg(feature = "file-trigger")]
pub use crate::trigger::file::enable_file_trigger;

#[cfg(feature = "socket-trigger")]
pub mod trigger;
#[cfg(feature = "socket-trigger")]
pub use crate::trigger::socket::enable_socket_trigger;
