use std::mem;

use crate::types::{Error, Result};

#[repr(i32)]
#[derive(Debug)]
pub enum Status {
    Ok,
    GenericError,
    InvalidParameter,
    OutOfMemory,
    ObjectBusy,
    InsufficientBuffer,
    NotImplemented,
    Win32Error,
    WrongState,
    Aborted,
    FileNotFound,
    ValueOverflow,
    AccessDenied,
    UnknownImageFormat,
    FontFamilyNotFound,
    FontStyleNotFound,
    NotTrueTypeFont,
    UnsupportedGdiplusVersion,
    GdiplusNotInitialized,
    PropertyNotFound,
    PropertyNotSupported,
    ProfileNotFound,
}
impl TryFrom<i32> for Status {
    type Error = Error;

    fn try_from(val: i32) -> Result<Self> {
        if val >= 0 && val <= 21 {
            Ok(unsafe { mem::transmute(val) })
        } else {
            Err(Error::Code(val))
        }
    }
}

#[repr(i32)]
#[derive(Debug)]
pub enum SmoothingMode {
    Invalid = -1,
    Default,
    HighSpeed,
    HighQuality,
    None,
    AntiAlias,
}
