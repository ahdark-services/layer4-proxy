extern crate libc;

use libc::{c_int, c_longlong, size_t, ssize_t};

extern "C" {
    pub fn splice(fd_in: c_int, off_in: *mut c_longlong, fd_out: c_int, off_out: *mut c_longlong, len: size_t, flags: c_int) -> ssize_t;
}

pub const SPLICE_F_MOVE: c_int = 1;
