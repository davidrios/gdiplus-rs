#![cfg(windows)]

use gdiplus::enums::Status;
use gdiplus_sys2::{Status_Ok, Status_PropertyNotFound, Status_Win32Error};

#[test]
fn test_status_enum() {
    assert_eq!(Status::Ok as i32, Status_Ok);
    assert_eq!(Status::Win32Error as i32, Status_Win32Error);
    assert_eq!(Status::PropertyNotFound as i32, Status_PropertyNotFound);
}
