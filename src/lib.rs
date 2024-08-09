#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod defines;

extern crate rivets_macros;
pub use rivets_macros::detour;
pub use rivets_macros::initialize;

extern crate rivets_shared;
pub use rivets_shared::inject;

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
