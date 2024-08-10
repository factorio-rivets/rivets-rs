#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod defines;

extern crate rivets_macros;
pub use rivets_macros::detour;
pub use rivets_macros::initialize;

extern crate rivets_shared;
pub use rivets_shared::inject;
pub use rivets_shared::Opaque;
