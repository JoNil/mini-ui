use super::xlib::Display;
use std::ffi::{c_char, c_ulong};

#[link(name = "Xcursor")]
extern "C" {
    pub fn XcursorLibraryLoadCursor(_2: *mut Display, _1: *const c_char) -> c_ulong;
}
