use std::ffi::OsStr;
use std::os::windows::prelude::OsStrExt;

pub fn wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[macro_export]
macro_rules! wpanic_ifeq {
    ( $code:expr, $compared:expr ) => {{
        let res = unsafe { $code };
        if res == $compared {
            std::panic::panic_any(std::io::Error::last_os_error());
        }
        res
    }};
}

#[macro_export]
macro_rules! wpanic_ifnull {
    ( $code:expr ) => {{
        let res = unsafe { $code };
        if res as winapi::shared::minwindef::LPVOID == winapi::shared::ntdef::NULL {
            std::panic::panic_any(std::io::Error::last_os_error());
        }
        res
    }};
}

#[macro_export]
macro_rules! wpanic_ifisnull {
    ( $code:expr ) => {{
        let res = unsafe { $code };
        if res.is_null() {
            std::panic::panic_any(std::io::Error::last_os_error());
        }
        res
    }};
}
