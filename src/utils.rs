//
// Copyright (c) The yang-rs Core Contributors
//
// SPDX-License-Identifier: MIT
//

use libyang3_sys as ffi;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Convert C String to owned string.
pub(crate) fn char_ptr_to_string(c_str: *const c_char, free: bool) -> String {
    let string =
        unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() };
    if free {
        unsafe { ffi::free(c_str as *mut std::ffi::c_void) };
    }
    string
}

/// Convert C String to optional owned string.
pub(crate) fn char_ptr_to_opt_string(
    c_str: *const c_char,
    free: bool,
) -> Option<String> {
    if c_str.is_null() {
        None
    } else {
        Some(char_ptr_to_string(c_str, free))
    }
}

/// Convert C String to string slice.
pub(crate) fn char_ptr_to_str<'a>(c_str: *const c_char) -> &'a str {
    unsafe { CStr::from_ptr(c_str).to_str().unwrap() }
}

/// Convert C String to optional string slice.
pub(crate) fn char_ptr_to_opt_str<'a>(c_str: *const c_char) -> Option<&'a str> {
    if c_str.is_null() {
        None
    } else {
        Some(char_ptr_to_str(c_str))
    }
}

/// A trait implemented by all types that can be created from a raw C pointer
/// and a generic container type.
pub unsafe trait Binding<'a>
where
    Self: Sized,
    <Self as Binding<'a>>::Container: 'a,
{
    type CType;
    type Container;

    unsafe fn from_raw(
        container: &'a Self::Container,
        raw: *mut Self::CType,
    ) -> Self;

    unsafe fn from_raw_opt(
        container: &'a Self::Container,
        raw: *mut Self::CType,
    ) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self::from_raw(container, raw))
        }
    }
}
