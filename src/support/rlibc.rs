// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A bare-metal library supplying functions rustc may lower code to
//!
//! This library is not intended for general use, and is superseded by a system
//! libc if one is available. In a freestanding context, however, common
//! functions such as memset, memcpy, etc are not implemented. This library
//! provides an implementation of these functions which are either required by
//! libcore or called by rustc implicitly.
//!
//! This library is never included by default, and must be manually included if
//! necessary. It is an error to include this library when also linking with
//! the system libc library.

// This library defines the builtin functions, so it would be a shame for
// LLVM to optimize these function calls to themselves!
#![no_builtins]

extern crate core;

#[cfg(test)] #[macro_use] extern crate std;

#[no_mangle]
pub unsafe extern fn __aeabi_memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn __aeabi_memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}

#[no_mangle]
pub unsafe extern fn __aeabi_memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    return 0;
}

#[cfg(test)]
mod test {
    use core::str::StrExt;
    use core::slice::SliceExt;

    use super::{__aeabi_memcmp, __aeabi_memset, __aeabi_memcpy};

    #[test]
    fn memcmp_single_byte_pointers() {
        unsafe {
            assert_eq!(__aeabi_memcmp(&0xFAu8, &0xFAu8, 1), 0x00);
            assert!(__aeabi_memcmp(&0xEFu8, &0xFEu8, 1) < 0x00);
        }
    }

    #[test]
    fn memcmp_strings() {
        {
            let (x, z) = ("Hello!", "Good Bye.");
            let l = x.len();
            unsafe {
                assert_eq!(__aeabi_memcmp(x.as_ptr(), x.as_ptr(), l), 0);
                assert!(__aeabi_memcmp(x.as_ptr(), z.as_ptr(), l) > 0);
                assert!(__aeabi_memcmp(z.as_ptr(), x.as_ptr(), l) < 0);
            }
        }
        {
            let (x, z) = ("hey!", "hey.");
            let l = x.len();
            unsafe {
                assert!(__aeabi_memcmp(x.as_ptr(), z.as_ptr(), l) < 0);
            }
        }
    }

    #[test]
    fn memset_single_byte_pointers() {
        let mut x: u8 = 0xFF;
        unsafe {
            __aeabi_memset(&mut x, 0xAA, 1);
            assert_eq!(x, 0xAA);
            __aeabi_memset(&mut x, 0x00, 1);
            assert_eq!(x, 0x00);
            x = 0x01;
            __aeabi_memset(&mut x, 0x12, 0);
            assert_eq!(x, 0x01);
        }
    }

    #[test]
    fn memset_array() {
        let mut buffer = [b'X';  100];
        unsafe {
            __aeabi_memset(buffer.as_mut_ptr(), b'#' as i32, buffer.len());
        }
        for byte in buffer.iter() { assert_eq!(*byte, b'#'); }
    }

    #[test]
    fn memcpy_and_memcmp_arrays() {
        let (src, mut dst) = ([b'X';  100], [b'Y';  100]);
        unsafe {
            assert!(__aeabi_memcmp(src.as_ptr(), dst.as_ptr(), 100) != 0);
            let _ = __aeabi_memcpy(dst.as_mut_ptr(), src.as_ptr(), 100);
            assert_eq!(__aeabi_memcmp(src.as_ptr(), dst.as_ptr(), 100), 0);
        }
    }
}
