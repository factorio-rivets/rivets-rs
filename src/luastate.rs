#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    clippy::wildcard_imports
)]

use core::ffi::*;

type size_t = c_ulong; // c_size_t is nightly only, defaults to usize currently -> 64 bit
type lu_byte = c_char;
type lua_Number = c_double;
type lua_CFunction = unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int;
type lua_Hook = unsafe extern "C-unwind" fn(L: *mut lua_State, ar: *mut lua_Debug);
type lua_Alloc = unsafe extern "C-unwind" fn(
    ud: *mut c_void,
    ptr: *mut c_void,
    osize: usize,
    nsize: usize,
) -> *mut c_void;
type Instruction = c_uint; // 32 bit
type ptrdiff_t = isize; // https://en.cppreference.com/w/cpp/types/ptrdiff_t
type luai_jmpbuf = c_int;
type lu_mem = size_t;
type l_mem = ptrdiff_t;

#[derive(Clone, Copy)]
#[repr(C)]
struct GCheader {
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
}

#[derive(Clone, Copy)]
#[repr(C)]
union L_Umaxalign {
    u: c_double,
    s: *mut c_void,
    l: c_longlong,
}

#[derive(Clone, Copy)]
#[repr(C)]
union TString {
    dummy: L_Umaxalign,
    tsv: TStringInner,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct TStringInner {
    // common header
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
    // common header
    extra: lu_byte,
    hash: c_uint,
    len: size_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
union Udata {
    dummy: L_Umaxalign,
    uv: UdataInner,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct UdataInner {
    // common header
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
    // common header
    metatable: *mut Table,
    env: *mut Table,
    len: c_ulong, // c_size_t is nightly only, defaults to usize currently -> 64 bit
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Table {
    // common header
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
    // common header
    flags: lu_byte,
    lsizenode: lu_byte,
    customflags: lu_byte,
    metatable: *mut Table,
    array: *mut TValue,
    node: *mut Node,
    lastfree: *mut Node,
    gclist: *mut GCObject,
    sizearray: c_int,
    firstadded: *mut Node,
    lastadded: *mut Node,
    customdata: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct TValue {
    value_: Value,
    tt_: c_int,
}

type StkId = *mut TValue;

#[derive(Clone, Copy)]
#[repr(C)]
union Value {
    gc: *mut GCObject,
    p: *mut c_void,
    b: bool,
    f: lua_CFunction,
    n: lua_Number,
}

#[derive(Clone, Copy)]
#[repr(C)]
union TKey {
    nk: TKeyInner,
    tvk: TValue,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct TKeyInner {
    value_: Value,
    tt_: c_int,
    next: *mut Node,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Node {
    i_val: TValue,
    i_key: TKey,
    next: *mut Node,
    prev: *mut Node,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct CClosure {
    // closure header
    // common header
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
    // common header
    nupvalues: lu_byte,
    gclist: *mut GCObject,
    // closure header
    f: lua_CFunction,
    upvalue: [TValue; 1],
}

#[derive(Clone, Copy)]
#[repr(C)]
struct LClosure {
    // closure header
    // common header
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
    // common header
    nupvalues: lu_byte,
    gclist: *mut GCObject,
    // closure header
    p: *mut Proto,
    upvals: *mut [UpVal; 1],
}

#[derive(Clone, Copy)]
#[repr(C)]
union Closure {
    c: CClosure,
    l: LClosure,
}

#[derive(Clone, Copy)]
#[repr(C)]
union GCObject {
    gch: GCheader,
    ts: TString,
    u: Udata,
    cl: Closure,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Proto {
    // common header
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
    // common header
    k: *mut TValue,
    code: *mut Instruction,
    p: *mut *mut Proto,
    lineinfo: *mut c_int,
    locvars: *mut LocVar,
    upvalues: *mut Upvaldesc,
    cache: *mut Closure,
    source: *mut TString,
    sizeupvalues: c_int,
    sizek: c_int,
    sizecode: c_int,
    sizelineinfo: c_int,
    sizep: c_int,
    sizelocvars: c_int,
    linedefined: c_int,
    lastlinedefined: c_int,
    gclist: *mut GCObject,
    numparams: lu_byte,
    is_vararg: lu_byte,
    maxstacksize: lu_byte,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct UpVal {
    // common header
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
    // common header
    v: *mut TValue,
    u: UpValInternal,
}

#[derive(Clone, Copy)]
#[repr(C)]
union UpValInternal {
    value: TValue,
    l: UpValInternalInternal,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct UpValInternalInternal {
    prev: *mut UpVal,
    next: *mut UpVal,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct LocVar {
    varname: *mut TString,
    startpc: c_int,
    endpc: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Upvaldesc {
    name: *mut TString,
    instack: lu_byte,
    idx: lu_byte,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct CallInfo {
    func: StkId,             /* function index in the stack */
    top: StkId,              /* top for this function */
    previous: *mut CallInfo, /* dynamic call link */
    next: *mut CallInfo,     /* dynamic call link */
    nresults: c_short,       /* expected number of results from this function */
    callstatus: lu_byte,
    extra: ptrdiff_t,
    u: CallInfoInternal,
}

#[derive(Clone, Copy)]
#[repr(C)]
union CallInfoInternal {
    l: CallInfoInternalL,
    c: CallInfoInternalC,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct CallInfoInternalL {
    base: StkId,
    savedpc: *const Instruction,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct CallInfoInternalC {
    ctx: c_int,
    k: lua_CFunction,
    old_errfunc: ptrdiff_t,
    old_allowhook: lu_byte,
    status: lu_byte,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct lua_longjmp {
    previous: *mut lua_longjmp,
    b: luai_jmpbuf,
    status: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct lua_State {
    // common header
    previous: *mut GCObject,
    next: *mut GCObject,
    tt: lu_byte,
    marked: lu_byte,
    // common header
    status: lu_byte,
    top: StkId, /* first free slot in the stack */
    l_G: *mut global_State,
    ci: *mut CallInfo,         /* call info for current function */
    oldpc: *const Instruction, /* last pc traced */
    stack_last: StkId,         /* last free slot in the stack */
    stack: StkId,              /* stack base */
    stacksize: c_int,
    nny: c_ushort,     /* number of non-yieldable calls in stack */
    nCcalls: c_ushort, /* number of nested C calls */
    hookmask: lu_byte,
    allowhook: lu_byte,
    basehookcount: c_int,
    hookcount: c_int,
    hook: lua_Hook,
    openupval: *mut GCObject, /* list of open upvalues in this stack */
    gclist: *mut GCObject,
    errorJmp: *mut lua_longjmp, /* current error recover point */
    errfunc: ptrdiff_t,         /* current error handling function (stack index) */
    base_ci: CallInfo,          /* CallInfo for first level (C calling Lua) */
    userData1: *mut c_void,     /* custom user-data pointer */
    userData2: *mut c_void,     /* custom user-data pointer */
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Mbuffer {
    buffer: *mut c_char,
    n: size_t,
    buffsize: size_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct stringtable {
    hash: *mut *mut GCObject,
    nuse: c_uint,
    size: c_int,
}

const TM_N: usize = 17; // count of variants in this enum (excluding the last one) https://github.com/Rseding91/Factorio-Lua/blob/ce12474c7fcee694bde1aa0f668dce488aca0806/src/ltm.h#L18
const LUA_NUMTAGS: usize = 9;

#[derive(Clone, Copy)]
#[repr(C)]
struct global_State {
    frealloc: lua_Alloc,
    ud: *mut c_void,
    totalbytes: lu_mem,
    GCdebt: l_mem,
    GCmemtrav: lu_mem,
    GCestimate: lu_mem,
    strt: stringtable,
    l_registry: TValue,
    seed: c_uint,
    currentwhite: lu_byte,
    gcstate: lu_byte,
    gckind: lu_byte,
    gcrunning: lu_byte,
    gcblocked: lu_byte,
    sweepstrgc: c_int,
    allgc: *mut GCObject,
    finobj: *mut GCObject,
    sweepgc: *mut *mut GCObject,
    sweepfin: *mut *mut GCObject,
    gray: *mut GCObject,
    grayagain: *mut GCObject,
    weak: *mut GCObject,
    ephemeron: *mut GCObject,
    allweak: *mut GCObject,
    tobefnz: *mut GCObject,
    uvhead: UpVal,
    buff: Mbuffer,
    gcpause: c_int,
    gcmajorinc: c_int,
    gcstepmul: c_int,
    panic: lua_CFunction,
    mainthread: *mut lua_State,
    version: *const lua_Number,
    memerrmsg: *mut TString,
    tmname: *mut [TString; TM_N],
    mt: *mut [Table; LUA_NUMTAGS],
}

const LUA_IDSIZE: usize = 60;

#[derive(Clone, Copy)]
#[repr(C)]
struct lua_Debug {
    event: c_int,
    name: *const c_char,
    namewhat: *const c_char,
    what: *const c_char,
    source: *const c_char,
    currentline: c_int,
    currentpc: c_int,
    linedefined: c_int,
    lastlinedefined: c_int,
    nups: c_uchar,
    nparams: c_uchar,
    isvararg: c_char,
    istailcall: c_char,
    short_src: [c_char; LUA_IDSIZE],
    i_ci: *mut CallInfo,
}
