#![allow(dead_code)]
#![cfg(windows)]

use std::ptr;
use std::ffi::c_void;
use std::mem::{size_of, zeroed};
use std::os::raw::c_ulong;

#[link(name = "advapi32")]
unsafe extern "system" {
    fn OpenProcessToken(process: *mut c_void, access: c_ulong, token: *mut *mut c_void) -> i32;
    fn GetTokenInformation(
        token_handle: *mut c_void,
        token_info_class: c_ulong,
        token_info: *mut c_void,
        token_info_length: c_ulong,
        return_length: *mut c_ulong,
    ) -> i32;
    fn GetCurrentProcess() -> *mut c_void;
    fn CloseHandle(handle: *mut c_void) -> i32;
}

const TOKEN_QUERY: c_ulong = 0x0008;
const TOKEN_ELEVATION: c_ulong = 20;

#[repr(C)]
struct TokenElevation {
    token_is_elevated: c_ulong,
}

/// A safe wrapper around a Windows access token handle.
struct AccessToken {
    handle: *mut c_void,
}

impl AccessToken {
    /// Opens the access token for the current process.
    fn open() -> Option<Self> {
        let process = unsafe { GetCurrentProcess() };
        let mut token_handle: *mut c_void = ptr::null_mut();

        let success = unsafe { OpenProcessToken(process, TOKEN_QUERY, &mut token_handle) };
        if success == 0 {
            return None;
        }

        Some(Self { handle: token_handle })
    }

    /// Checks if the token is elevated (i.e., running as an administrator).
    fn is_elevated(&self) -> bool {
        let mut elevation: TokenElevation = unsafe { zeroed() };
        let mut return_length: c_ulong = 0;

        let success = unsafe {
            GetTokenInformation(
                self.handle,
                TOKEN_ELEVATION,
                &mut elevation as *mut _ as *mut c_void,
                size_of::<TokenElevation>() as c_ulong,
                &mut return_length,
            )
        };

        success != 0 && elevation.token_is_elevated != 0
    }
}

impl Drop for AccessToken {
    /// Ensures the token handle is always closed properly.
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}

/// Safe function to check if the user is an administrator.
pub fn is_admin() -> bool {
    if let Some(token) = AccessToken::open() {
        token.is_elevated()
    } else {
        false
    }
}