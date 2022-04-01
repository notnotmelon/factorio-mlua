//! Contains definitions from `lua.h`.

use std::marker::{PhantomData, PhantomPinned};
use std::os::raw::{c_char, c_double, c_int, c_uchar, c_uint, c_void};
use std::ptr;

// Mark for precompiled code (`<esc>Lua`)
pub const LUA_SIGNATURE: &[u8] = b"\x1bLua";

// Option for multiple returns in 'lua_pcall' and 'lua_call'
pub const LUA_MULTRET: c_int = -1;

// Size of the Lua stack
pub const LUAI_MAXSTACK: c_int = 1000000;

//
// Pseudo-indices
//
pub const LUA_REGISTRYINDEX: c_int = -LUAI_MAXSTACK - 1000;

pub const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_REGISTRYINDEX - i
}

//
// Thread status
//
pub const LUA_OK: c_int = 0;
pub const LUA_YIELD: c_int = 1;
pub const LUA_ERRRUN: c_int = 2;
pub const LUA_ERRSYNTAX: c_int = 3;
pub const LUA_ERRMEM: c_int = 4;
pub const LUA_ERRGCMM: c_int = 5;
pub const LUA_ERRERR: c_int = 6;

/// A raw Lua state associated with a thread.
#[repr(C)]
pub struct lua_State {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

//
// Basic types
//
pub const LUA_TNONE: c_int = -1;

pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TLIGHTUSERDATA: c_int = 2;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TSTRING: c_int = 4;
pub const LUA_TTABLE: c_int = 5;
pub const LUA_TFUNCTION: c_int = 6;
pub const LUA_TUSERDATA: c_int = 7;
pub const LUA_TTHREAD: c_int = 8;

pub const LUA_NUMTAGS: c_int = 9;

/// Minimum Lua stack available to a C function
pub const LUA_MINSTACK: c_int = 20;

// Predefined values in the registry
pub const LUA_RIDX_MAINTHREAD: lua_Integer = 1;
pub const LUA_RIDX_GLOBALS: lua_Integer = 2;
pub const LUA_RIDX_LAST: lua_Integer = LUA_RIDX_GLOBALS;

/// A Lua number, usually equivalent to `f64`
pub type lua_Number = c_double;

/// A Lua integer, usually equivalent to `i64`
pub type lua_Integer = isize;

/// A Lua unsigned integer, equivalent to `u32` in Lua 5.2
pub type lua_Unsigned = c_uint;

/// Type for native C functions that can be passed to Lua
pub type lua_CFunction = unsafe extern "C" fn(L: *mut lua_State) -> c_int;

// Type for functions that read/write blocks when loading/dumping Lua chunks
pub type lua_Reader =
    unsafe extern "C" fn(L: *mut lua_State, ud: *mut c_void, sz: *mut usize) -> *const c_char;
pub type lua_Writer =
    unsafe extern "C" fn(L: *mut lua_State, p: *const c_void, sz: usize, ud: *mut c_void) -> c_int;

/// Type for memory-allocation functions
pub type lua_Alloc = unsafe extern "C" fn(
    ud: *mut c_void,
    ptr: *mut c_void,
    osize: usize,
    nsize: usize,
) -> *mut c_void;

extern "C" {
    pub fn lua_registertracehandler(handler: Option<extern "C" fn(*const c_char)>);
    pub fn lua_trace(message: *const c_char);
    pub fn lua_traceandabort(message: *const c_char) -> c_int;
}

extern "C" {
    //
    // State manipulation
    //
    pub fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State;
    pub fn lua_close(L: *mut lua_State);
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;

    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;

    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;

    //
    // Basic stack manipulation
    //
    pub fn lua_absindex(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    pub fn lua_pushvalue(L: *mut lua_State, idx: c_int);
    pub fn lua_remove(L: *mut lua_State, idx: c_int);
    pub fn lua_insert(L: *mut lua_State, idx: c_int);
    pub fn lua_replace(L: *mut lua_State, idx: c_int);
    pub fn lua_copy(L: *mut lua_State, fromidx: c_int, toidx: c_int);
    pub fn lua_checkstack(L: *mut lua_State, sz: c_int) -> c_int;

    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);

    //
    // Access functions (stack -> C)
    //
    pub fn lua_isnumberorstringconvertabletonumber(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isstringornumberconvertabletostring(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_iscfunction(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_typename(L: *mut lua_State, tp: c_int) -> *const c_char;
    pub fn lua_getstring(L: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char;

    pub fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;
    #[link_name = "lua_tointegerx"]
    pub fn lua_tointegerx_(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Integer;
    pub fn lua_tounsignedx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Unsigned;
    pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char;
    pub fn lua_uncheckedtolstring(L: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char;
    pub fn lua_uncheckedaslstring(L: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char;
    pub fn lua_rawlen(L: *mut lua_State, idx: c_int) -> usize;
    pub fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> Option<lua_CFunction>;
    pub fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    pub fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State;
    pub fn lua_topointer(L: *mut lua_State, idx: c_int) -> *const c_void;
}

//
// Comparison and arithmetic functions
//
pub const LUA_OPADD: c_int = 0;
pub const LUA_OPSUB: c_int = 1;
pub const LUA_OPMUL: c_int = 2;
pub const LUA_OPDIV: c_int = 3;
pub const LUA_OPMOD: c_int = 4;
pub const LUA_OPPOW: c_int = 5;
pub const LUA_OPUNM: c_int = 6;

extern "C" {
    pub fn lua_arith(L: *mut lua_State, op: c_int);
}

pub const LUA_OPEQ: c_int = 0;
pub const LUA_OPLT: c_int = 1;
pub const LUA_OPLE: c_int = 2;

extern "C" {
    pub fn lua_rawequal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;
    pub fn lua_compare(L: *mut lua_State, idx1: c_int, idx2: c_int, op: c_int) -> c_int;
}

extern "C" {
    //
    // Push functions (C -> stack)
    //
    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    pub fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned);
    #[link_name = "lua_pushlstring"]
    pub fn lua_pushlstring_(L: *mut lua_State, s: *const c_char, l: usize) -> *const c_char;
    pub fn lua_pushstring(L: *mut lua_State, s: *const c_char) -> *const c_char;
    // lua_pushvfstring
    pub fn lua_pushfstring(L: *mut lua_State, fmt: *const c_char, ...) -> *const c_char;
    pub fn lua_pushcclosure(L: *mut lua_State, f: lua_CFunction, n: c_int);
    pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    pub fn lua_pushthread(L: *mut lua_State) -> c_int;

    //
    // Get functions (Lua -> stack)
    //
    #[link_name = "lua_getglobal"]
    pub fn lua_getglobal_(L: *mut lua_State, name: *const c_char);
    #[link_name = "lua_gettable"]
    pub fn lua_gettable_(L: *mut lua_State, idx: c_int);
    pub fn lua_getfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    #[link_name = "lua_getfield"]
    pub fn lua_getfield_(L: *mut lua_State, idx: c_int, k: *const c_char);
    #[link_name = "lua_rawget"]
    pub fn lua_rawget_(L: *mut lua_State, idx: c_int);
    #[link_name = "lua_rawgeti"]
    pub fn lua_rawgeti_(L: *mut lua_State, idx: c_int, n: c_int);
    #[link_name = "lua_rawgetp"]
    pub fn lua_rawgetp_(L: *mut lua_State, idx: c_int, p: *const c_void);
    pub fn lua_rawgetglobal(L: *mut lua_State, var: *const c_char);
    pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    pub fn lua_newuserdata(L: *mut lua_State, sz: usize) -> *mut c_void;
    pub fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    #[link_name = "lua_getuservalue"]
    pub fn lua_getuservalue_(L: *mut lua_State, idx: c_int);
    pub fn lua_rawwgetTablesizes(L: *mut lua_State, idx: c_int, outarraysize: *mut c_int, outhashsize: *mut c_int);

    //
    // Set functions (stack -> Lua)
    //
    pub fn lua_setglobal(L: *mut lua_State, var: *const c_char);
    pub fn lua_settable(L: *mut lua_State, idx: c_int);
    pub fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    pub fn lua_rawset(L: *mut lua_State, idx: c_int);
    #[link_name = "lua_rawseti"]
    pub fn lua_rawseti_(L: *mut lua_State, idx: c_int, n: c_int);
    pub fn lua_rawsetp(L: *mut lua_State, idx: c_int, p: *const c_void);
    pub fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    pub fn lua_setuservalue(L: *mut lua_State, idx: c_int);

    //
    // 'load' and 'call' functions (load and run Lua code)
    //
    pub fn lua_callk(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        ctx: c_int,
        k: Option<lua_CFunction>,
    );
    pub fn lua_pcallk(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        errfunc: c_int,
        ctx: c_int,
        k: Option<lua_CFunction>,
    ) -> c_int;

    pub fn lua_getctx(L: *mut lua_State, ctx: *mut c_int) -> c_int;

    pub fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        data: *mut c_void,
        chunkname: *const c_char,
        mode: *const c_char,
    ) -> c_int;
    #[link_name = "lua_dump"]
    pub fn lua_dump_(L: *mut lua_State, writer: lua_Writer, data: *mut c_void) -> c_int;
}

#[inline(always)]
pub unsafe fn lua_call(L: *mut lua_State, n: c_int, r: c_int) {
    lua_callk(L, n, r, 0, None)
}

#[inline(always)]
pub unsafe fn lua_pcall(L: *mut lua_State, n: c_int, r: c_int, f: c_int) -> c_int {
    lua_pcallk(L, n, r, f, 0, None)
}

/* // Disabled in Factorio lua
extern "C" {
    //
    // Coroutine functions
    //
    pub fn lua_yieldk(
        L: *mut lua_State,
        nresults: c_int,
        ctx: c_int,
        k: Option<lua_CFunction>,
    ) -> c_int;
    #[link_name = "lua_resume"]
    pub fn lua_resume_(L: *mut lua_State, from: *mut lua_State, narg: c_int) -> c_int;
    pub fn lua_status(L: *mut lua_State) -> c_int;
}

#[inline(always)]
pub unsafe fn lua_yield(L: *mut lua_State, n: c_int) -> c_int {
    lua_yieldk(L, n, 0, None)
}
*/

//
// Garbage-collection function and options
//
pub const LUA_GCSTOP: c_int = 0;
pub const LUA_GCRESTART: c_int = 1;
pub const LUA_GCCOLLECT: c_int = 2;
pub const LUA_GCCOUNT: c_int = 3;
pub const LUA_GCCOUNTB: c_int = 4;
pub const LUA_GCSTEP: c_int = 5;
pub const LUA_GCSETPAUSE: c_int = 6;
pub const LUA_GCSETSTEPMUL: c_int = 7;
pub const LUA_GCSETMAJORINC: c_int = 8;
pub const LUA_GCISRUNNING: c_int = 9;
pub const LUA_GCGEN: c_int = 10;
pub const LUA_GCINC: c_int = 11;

extern "C" {
    pub fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;
}

extern "C" {
    //
    // Miscellaneous functions
    //
    pub fn lua_error(L: *mut lua_State) -> !;
    pub fn lua_next(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_concat(L: *mut lua_State, n: c_int);
    pub fn lua_len(L: *mut lua_State, idx: c_int);
    pub fn lua_tablesize(L: *mut lua_State, idx: c_int, fuzzy: c_int) -> c_int;
    pub fn lua_getnparams(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tableresize(L: *mut lua_State, idx: c_int, narr: c_int, nrec: c_int);
    pub fn lua_isvalidIndex(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc;
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut c_void);
}

//
// Some useful macros (implemented as Rust functions)
//
#[inline(always)]
pub unsafe fn lua_tonumber(L: *mut lua_State, i: c_int) -> lua_Number {
    lua_tonumberx(L, i, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_tointeger_(L: *mut lua_State, i: c_int) -> lua_Integer {
    lua_tointegerx_(L, i, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_tounsigned(L: *mut lua_State, i: c_int) -> lua_Unsigned {
    lua_tounsignedx(L, i, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    lua_settop(L, -n - 1)
}

#[inline(always)]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
}

#[inline(always)]
pub unsafe fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, n)
}

#[inline(always)]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0)
}

#[inline(always)]
pub unsafe fn lua_isfunction(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TFUNCTION) as c_int
}

#[inline(always)]
pub unsafe fn lua_istable(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TTABLE) as c_int
}

#[inline(always)]
pub unsafe fn lua_islightuserdata(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TLIGHTUSERDATA) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnil(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TNIL) as c_int
}

#[inline(always)]
pub unsafe fn lua_isboolean(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TBOOLEAN) as c_int
}

#[inline(always)]
pub unsafe fn lua_isthread(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TTHREAD) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnone(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) == LUA_TNONE) as c_int
}

#[inline(always)]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: c_int) -> c_int {
    (lua_type(L, n) <= 0) as c_int
}

#[inline(always)]
pub unsafe fn lua_pushliteral(L: *mut lua_State, s: &'static str) -> *const c_char {
    use std::ffi::CString;
    let c_str = CString::new(s).unwrap();
    lua_pushlstring_(L, c_str.as_ptr(), c_str.as_bytes().len())
}

#[inline(always)]
pub unsafe fn lua_pushglobaltable(L: *mut lua_State) {
    lua_rawgeti_(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS as _)
}

#[inline(always)]
pub unsafe fn lua_tostring(L: *mut lua_State, i: c_int) -> *const c_char {
    lua_tolstring(L, i, ptr::null_mut())
}

//
// Debug API
//

// Maximum size for the description of the source of a function in debug information.
const LUA_IDSIZE: usize = 60;

// Event codes
pub const LUA_HOOKCALL: c_int = 0;
pub const LUA_HOOKRET: c_int = 1;
pub const LUA_HOOKLINE: c_int = 2;
pub const LUA_HOOKCOUNT: c_int = 3;
pub const LUA_HOOKTAILCALL: c_int = 4;

// Event masks
pub const LUA_MASKCALL: c_int = 1 << (LUA_HOOKCALL as usize);
pub const LUA_MASKRET: c_int = 1 << (LUA_HOOKRET as usize);
pub const LUA_MASKLINE: c_int = 1 << (LUA_HOOKLINE as usize);
pub const LUA_MASKCOUNT: c_int = 1 << (LUA_HOOKCOUNT as usize);

/// Type for functions to be called on debug events.
pub type lua_Hook = unsafe extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug);

extern "C" {
    pub fn lua_getstack(L: *mut lua_State, level: c_int, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getinfo(L: *mut lua_State, what: *const c_char, ar: *mut lua_Debug) -> c_int;
    pub fn lua_getnresults(L: *mut lua_State) -> c_int;
    pub fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_char;
    pub fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: c_int) -> *const c_char;
    pub fn lua_getupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;
    pub fn lua_setupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) -> *const c_char;

    pub fn lua_upvalueid(L: *mut lua_State, fidx: c_int, n: c_int) -> *mut c_void;
    pub fn lua_upvaluejoin(L: *mut lua_State, fidx1: c_int, n1: c_int, fidx2: c_int, n2: c_int);

    pub fn lua_sethook(L: *mut lua_State, func: Option<lua_Hook>, mask: c_int, count: c_int);
    pub fn lua_gethook(L: *mut lua_State) -> Option<lua_Hook>;
    pub fn lua_gethookmask(L: *mut lua_State) -> c_int;
    pub fn lua_gethookcount(L: *mut lua_State) -> c_int;
}

#[repr(C)]
pub struct lua_Debug {
    pub event: c_int,
    pub name: *const c_char,
    pub namewhat: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub currentline: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub nups: c_uchar,
    pub nparams: c_uchar,
    pub isvararg: c_char,
    pub istailcall: c_char,
    pub short_src: [c_char; LUA_IDSIZE],
    // lua.h mentions this is for private use
    i_ci: *mut c_void,
}
