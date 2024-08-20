use crate as rivets;
use crate::lua::luastate;
use rivets_macros::import;

#[import(lua_gettop)]
pub extern "C" fn get_top(lua_state: *mut luastate::lua_State) -> i64 {}
