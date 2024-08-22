use crate as rivets;
use crate::lua::{lua_Debug, lua_State};
use rivets_macros::import;
use std::ffi::{c_char, c_int, c_uint, c_void};

use super::{lua_CFunction, lua_Hook, lua_Writer};

#[import(lua_absindex)]
pub extern "C-unwind" fn lua_absindex(lua_state: *mut lua_State, index: c_uint) -> c_int {}

#[import(lua_arith)]
pub extern "C-unwind" fn lua_arith(lua_state: *mut lua_State, op: c_int) {}

#[import(lua_atpanic)]
pub extern "C-unwind" fn lua_atpanic(lua_state: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction {}

#[import(lua_callk)]
pub extern "C-unwind" fn lua_callk(
    lua_state: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    ctx: c_int,
    k: lua_CFunction,
) {
}

#[import(lua_checkstack)]
pub extern "C-unwind" fn lua_checkstack(lua_state: *mut lua_State, size: c_int) -> c_int {}

#[import(lua_close)]
pub extern "C-unwind" fn lua_close(lua_state: *mut lua_State) {}

#[import(lua_compare)]
pub extern "C-unwind" fn lua_compare(
    lua_state: *mut lua_State,
    index_1: c_int,
    index_2: c_int,
    operator: c_int,
) -> c_int {
}

#[import(lua_concat)]
pub extern "C-unwind" fn lua_concat(lua_state: *mut lua_State, n: c_int) {}

#[import(lua_copy)]
pub extern "C-unwind" fn lua_copy(lua_state: *mut lua_State, from_index: c_int, to_index: c_int) {}

#[import(lua_createtable)]
pub extern "C-unwind" fn lua_createtable(lua_state: *mut lua_State, narray: c_int, nrec: c_int) {}

#[import(lua_dump)]
pub extern "C-unwind" fn lua_dump(
    lua_state: *mut lua_State,
    writer: lua_Writer,
    data: *mut c_void,
) -> c_int {
}

#[import(lua_error)]
pub extern "C-unwind" fn lua_error(lua_state: *mut lua_State) {}

#[import(lua_gc)]
pub extern "C-unwind" fn lua_gc(lua_state: *mut lua_State, what: c_int, data: c_int) -> c_int {}

//#[import(lua_getallocf)]
//pub extern "C-unwind" fn lua_getallocf(lua_state: *mut lua_State, ud: *mut *mut c_void) -> luastate::lua_Alloc {}

#[import(lua_getctx)]
pub extern "C-unwind" fn lua_getctx(lua_state: *mut lua_State, ctx: *mut c_int) -> c_int {}

#[import(lua_getfield)]
pub extern "C-unwind" fn lua_getfield(lua_state: *mut lua_State, index: c_int, k: *const c_char) -> c_int {
}

#[import(lua_getglobal)]
pub extern "C-unwind" fn lua_getglobal(
    lua_state: *mut lua_State,
    global_varible_name: *const c_char,
) -> c_int {
}

#[import(lua_gethook)]
pub extern "C-unwind" fn lua_gethook(lua_state: *mut lua_State) -> lua_Hook {}

#[import(lua_gethookcount)]
pub extern "C-unwind" fn lua_gethookcount(lua_state: *mut lua_State) -> c_int {}

#[import(lua_gethookmask)]
pub extern "C-unwind" fn lua_gethookmask(lua_state: *mut lua_State) -> c_int {}

#[import(lua_getinfo)]
pub extern "C-unwind" fn lua_getinfo(
    lua_state: *mut lua_State,
    what: *const c_char,
    ar: *mut lua_Debug,
) -> c_int {
}

#[import(lua_getlocal)]
pub extern "C-unwind" fn lua_getlocal(
    lua_state: *mut lua_State,
    ar: *const lua_Debug,
    n: c_int,
) -> *const c_char {
}

#[import(lua_getmetatable)]
pub extern "C-unwind" fn lua_getmetatable(lua_state: *mut lua_State, object_index: c_int) -> c_int {}

#[import(lua_getstack)]
pub extern "C-unwind" fn lua_getstack(
    lua_state: *mut lua_State,
    level: c_int,
    ar: *mut lua_Debug,
) -> c_int {
}

#[import(lua_gettable)]
pub extern "C-unwind" fn lua_gettable(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_gettop)]
pub extern "C-unwind" fn lua_gettop(lua_state: *mut lua_State) -> c_int {}

#[import(lua_getupvalue)]
pub extern "C-unwind" fn lua_getupvalue(
    lua_state: *mut lua_State,
    function_index: c_int,
    upvalue_index: c_int,
) -> *const c_char {
}

#[import(lua_getuservalue)]
pub extern "C-unwind" fn lua_getuservalue(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_insert)]
pub extern "C-unwind" fn lua_insert(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_iscfunction)]
pub extern "C-unwind" fn lua_iscfunction(lua_state: *mut lua_State, index: c_int) -> c_int {}

#[import(lua_isnumberorstringconvertabletonumber)]
pub extern "C-unwind" fn lua_isnumber(lua_state: *mut lua_State, index: c_int) -> c_int {}

#[import(lua_isstringornumberconvertabletostring)]
pub extern "C-unwind" fn lua_isstring(lua_state: *mut lua_State, index: c_int) -> c_int {}
