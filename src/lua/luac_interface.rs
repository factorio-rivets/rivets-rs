use crate as rivets;
use crate::lua::{lua_Debug, lua_State};
use core::ffi::c_size_t;
use rivets_macros::import;
use std::ffi::{c_char, c_int, c_uint, c_void};

use super::{
    lua_Alloc, lua_CFunction, lua_Hook, lua_Integer, lua_Number, lua_Reader, lua_Unsigned,
    lua_Writer,
};

#[import(lua_absindex)]
pub extern "C-unwind" fn lua_absindex(lua_state: *mut lua_State, index: c_uint) -> c_int {}

#[import(lua_arith)]
pub extern "C-unwind" fn lua_arith(lua_state: *mut lua_State, op: c_int) {}

#[import(lua_atpanic)]
pub extern "C-unwind" fn lua_atpanic(
    lua_state: *mut lua_State,
    panicf: lua_CFunction,
) -> lua_CFunction {
}

#[import(lua_callk)]
pub extern "C-unwind" fn lua_callk(
    lua_state: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    ctx: c_int,
    k: Option<lua_CFunction>,
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

#[import(lua_getallocf)]
pub extern "C-unwind" fn lua_getallocf(
    lua_state: *mut lua_State,
    ud: *mut *mut c_void,
) -> lua_Alloc {
}

#[import(lua_getctx)]
pub extern "C-unwind" fn lua_getctx(lua_state: *mut lua_State, ctx: *mut c_int) -> c_int {}

#[import(lua_getfield)]
pub extern "C-unwind" fn lua_getfield(
    lua_state: *mut lua_State,
    index: c_int,
    k: *const c_char,
) -> c_int {
}

#[import(lua_getglobal)]
pub extern "C-unwind" fn lua_getglobal(
    lua_state: *mut lua_State,
    global_varible_name: *const c_char,
) -> c_int {
}

#[import(lua_gethook)]
pub extern "C-unwind" fn lua_gethook(lua_state: *mut lua_State) -> Option<lua_Hook> {}

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
pub extern "C-unwind" fn lua_getmetatable(lua_state: *mut lua_State, object_index: c_int) -> c_int {
}

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

#[import(lua_isuserdata)]
pub extern "C-unwind" fn lua_isuserdata(lua_state: *mut lua_State, index: c_int) -> c_int {}

#[import(lua_len)]
pub extern "C-unwind" fn lua_len(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_load)]
pub extern "C-unwind" fn lua_load(
    lua_state: *mut lua_State,
    reader: lua_Reader,
    data: *mut c_void,
    chunk_name: *const c_char,
    mode: *const c_char,
) -> c_int {
}

// lua_newstate

#[import(lua_newthread)]
pub extern "C-unwind" fn lua_newthread(lua_state: *mut lua_State) -> *mut lua_State {}

#[import(lua_newuserdata)]
pub extern "C-unwind" fn lua_newuserdata(lua_state: *mut lua_State, size: c_size_t) -> *mut c_void {
}

#[import(lua_next)]
pub extern "C-unwind" fn lua_next(lua_state: *mut lua_State, index: c_int) -> c_int {}

#[import(lua_pcallk)]
pub extern "C-unwind" fn lua_pcallk(
    lua_state: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    error_function: c_int,
    ctx: c_int,
    k: Option<lua_CFunction>,
) -> c_int {
}

#[import(lua_pushboolean)]
pub extern "C-unwind" fn lua_pushboolean(lua_state: *mut lua_State, b: c_int) {}

#[import(lua_pushcclosure)]
pub extern "C-unwind" fn lua_pushcclosure(lua_state: *mut lua_State, f: lua_CFunction, n: c_int) {}

#[import(lua_pushfstring)]
pub extern "C-unwind" fn lua_pushfstring(
    lua_state: *mut lua_State,
    format: *const c_char,
    ...
) -> *const c_char {
}

#[import(lua_pushinteger)]
pub extern "C-unwind" fn lua_pushinteger(lua_state: *mut lua_State, n: lua_Integer) {}

#[import(lua_pushlightuserdata)]
pub extern "C-unwind" fn lua_pushlightuserdata(lua_state: *mut lua_State, p: *mut c_void) {}

#[import(lua_pushlstring)]
pub extern "C-unwind" fn lua_pushlstring(
    lua_state: *mut lua_State,
    s: *const c_char,
    len: c_size_t,
) -> *const c_char {
}

#[import(lua_pushnil)]
pub extern "C-unwind" fn lua_pushnil(lua_state: *mut lua_State) {}

#[import(lua_pushnumber)]
pub extern "C-unwind" fn lua_pushnumber(lua_state: *mut lua_State, n: lua_Number) {}

#[import(lua_pushstring)]
pub extern "C-unwind" fn lua_pushstring(
    lua_state: *mut lua_State,
    s: *const c_char,
) -> *const c_char {
}

#[import(lua_pushthread)]
pub extern "C-unwind" fn lua_pushthread(lua_state: *mut lua_State) -> c_int {}

#[import(lua_pushunsigned)]
pub extern "C-unwind" fn lua_pushunsigned(lua_state: *mut lua_State, number: lua_Unsigned) {}

#[import(lua_pushvalue)]
pub extern "C-unwind" fn lua_pushvalue(lua_state: *mut lua_State, index: c_int) {}

// lua_pushvfstring

#[import(lua_rawequal)]
pub extern "C-unwind" fn lua_rawequal(
    lua_state: *mut lua_State,
    index_1: c_int,
    index_2: c_int,
) -> c_int {
}

#[import(lua_rawget)]
pub extern "C-unwind" fn lua_rawget(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_rawgeti)]
pub extern "C-unwind" fn lua_rawgeti(lua_state: *mut lua_State, index: c_int, n: c_int) {}

#[import(lua_rawgetp)]
pub extern "C-unwind" fn lua_rawgetp(lua_state: *mut lua_State, index: c_int, p: *const c_void) {}

#[import(lua_rawlen)]
pub extern "C-unwind" fn lua_rawlen(lua_state: *mut lua_State, index: c_int) -> c_size_t {}

#[import(lua_rawset)]
pub extern "C-unwind" fn lua_rawset(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_rawseti)]
pub extern "C-unwind" fn lua_rawseti(lua_state: *mut lua_State, index: c_int, n: c_int) {}

#[import(lua_rawsetp)]
pub extern "C-unwind" fn lua_rawsetp(
    lua_state: *mut lua_State,
    index: c_int,
    p: *const c_void,
) -> c_void {
}

#[import(lua_remove)]
pub extern "C-unwind" fn lua_remove(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_replace)]
pub extern "C-unwind" fn lua_replace(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_setallocf)]
pub extern "C-unwind" fn lua_setallocf(
    lua_state: *mut lua_State,
    f: lua_Alloc,
    ud: *mut c_void,
) -> c_void {
}

#[import(lua_setfield)]
pub extern "C-unwind" fn lua_setfield(
    lua_state: *mut lua_State,
    index: c_int,
    k: *const c_char,
) -> c_void {
}

#[import(lua_setglobal)]
pub extern "C-unwind" fn lua_setglobal(
    lua_state: *mut lua_State,
    global_varible_name: *const c_char,
) -> c_void {
}

#[import(lua_sethook)]
pub extern "C-unwind" fn lua_sethook(
    lua_state: *mut lua_State,
    hook: Option<lua_Hook>,
    mask: c_int,
    count: c_int,
) {
}

#[import(lua_setlocal)]
pub extern "C-unwind" fn lua_setlocal(
    lua_state: *mut lua_State,
    ar: *const lua_Debug,
    n: c_int,
) -> *const c_char {
}

#[import(lua_setmetatable)]
pub extern "C-unwind" fn lua_setmetatable(lua_state: *mut lua_State, object_index: c_int) -> c_int {
}

#[import(lua_settable)]
pub extern "C-unwind" fn lua_settable(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_settop)]
pub extern "C-unwind" fn lua_settop(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_setupvalue)]
pub extern "C-unwind" fn lua_setupvalue(
    lua_state: *mut lua_State,
    function_index: c_int,
    upvalue_index: c_int,
) -> *const c_char {
}

#[import(lua_setuservalue)]
pub extern "C-unwind" fn lua_setuservalue(lua_state: *mut lua_State, index: c_int) {}

#[import(lua_status)]
pub extern "C-unwind" fn lua_status(lua_state: *mut lua_State) -> c_int {}

#[import(lua_toboolean)]
pub extern "C-unwind" fn lua_toboolean(lua_state: *mut lua_State, index: c_int) -> c_int {}

#[import(lua_tocfunction)]
pub extern "C-unwind" fn lua_tocfunction(
    lua_state: *mut lua_State,
    index: c_int,
) -> Option<lua_CFunction> {
}

#[import(lua_tointegerx)]
pub extern "C-unwind" fn lua_tointegerx(
    lua_state: *mut lua_State,
    index: c_int,
    is_num: *mut c_int,
) -> lua_Integer {
}

#[import(lua_tolstring)]
pub extern "C-unwind" fn lua_tolstring(
    lua_state: *mut lua_State,
    index: c_int,
    len: *mut c_size_t,
) -> *const c_char {
}

#[import(lua_tonumberx)]
pub extern "C-unwind" fn lua_tonumberx(
    lua_state: *mut lua_State,
    index: c_int,
    is_num: *mut c_int,
) -> lua_Number {
}

#[import(lua_topointer)]
pub extern "C-unwind" fn lua_topointer(
    lua_state: *mut lua_State,
    index: c_int,
) -> *const c_void {
}

#[import(lua_tothread)]
pub extern "C-unwind" fn lua_tothread(lua_state: *mut lua_State, index: c_int) -> *mut lua_State {}

#[import(lua_tounsignedx)]
pub extern "C-unwind" fn lua_tounsignedx(
    lua_state: *mut lua_State,
    index: c_int,
    is_num: *mut c_int,
) -> lua_Unsigned {
}

#[import(lua_touserdata)]
pub extern "C-unwind" fn lua_touserdata(
    lua_state: *mut lua_State,
    index: c_int,
) -> *mut c_void {
}

#[import(lua_type)]
pub extern "C-unwind" fn lua_type(lua_state: *mut lua_State, index: c_int) -> c_int {}

#[import(lua_typename)]
pub extern "C-unwind" fn lua_typename(
    lua_state: *mut lua_State,
    type_: c_int,
) -> *const c_char {
}

#[import(lua_upvalueid)]
pub extern "C-unwind" fn lua_upvalueid(
    lua_state: *mut lua_State,
    function_index: c_int,
    n: c_int,
) -> *mut c_void {
}

#[import(lua_upvaluejoin)]
pub extern "C-unwind" fn lua_upvaluejoin(
    lua_state: *mut lua_State,
    function_index_1: c_int,
    n1: c_int,
    function_index_2: c_int,
    n2: c_int,
) {
}

#[import(lua_version)]
pub extern "C-unwind" fn lua_version(lua_state: *mut lua_State) -> *const lua_Number {}

#[import(lua_xmove)]
pub extern "C-unwind" fn lua_xmove(
    from: *mut lua_State,
    to: *mut lua_State,
    n: c_int,
) {
}