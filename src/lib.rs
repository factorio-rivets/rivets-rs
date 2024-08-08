#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

extern crate rivets_macros;
pub use rivets_macros::detour;
extern crate rivets_shared;
pub use rivets_shared::inject;
pub use rivets_shared::start_stream;
pub mod defines;

/// Repersents a pointer to any opaque FFI data. (normally detour args or FFI struct pointers)
/// 
/// # Examples
/// ```
/// #[detour(...)]
/// fn run(
///    this: Opaque,
///    lua_event_type: i32,
///    map_tick_type: Opaque,
///    lua_game_script: Opaque,
///    game_action: Opaque,
/// ) {
///     unsafe { back(this, lua_event_type, map_tick_type, lua_game_script, game_action) }
/// }
/// ```
pub type Opaque = *const std::ffi::c_void;