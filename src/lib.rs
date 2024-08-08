//! Welcome to Rivets, a Factorio mod loader written in Rust!
//! Rivets injects code into the Factorio binary via DLL injection, providing a powerful toolset for modding and enhancing the game.

extern crate rivets_macros;
pub use rivets_macros::detour;
extern crate rivets_shared;
pub use rivets_shared::inject;
pub use rivets_shared::start_stream;
pub mod defines;

/// Repersents any opaque FFI data.
pub type Opaque = *const std::ffi::c_void;