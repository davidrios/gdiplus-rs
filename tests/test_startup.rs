#![cfg(windows)]

use gdiplus::enums::Status;
use gdiplus::{Error, GdiPlus};
use gdiplus_sys2::GdiplusStartupInput;
use winapi::shared::minwindef::FALSE;

#[test]
fn test_startup_success() {
    let gdiplus = GdiPlus::startup(None, None);
    assert!(gdiplus.is_ok());
}

#[test]
fn test_startup_error() {
    let gdiplus = GdiPlus::startup(
        Some(Box::new(GdiplusStartupInput {
            DebugEventCallback: None,
            GdiplusVersion: 0, // invalid version
            SuppressBackgroundThread: FALSE,
            SuppressExternalCodecs: FALSE,
        })),
        None,
    );

    let err = gdiplus.err().expect("Startup should have failed.");

    match err {
        Error::Status(Status::UnsupportedGdiplusVersion) => {}
        _ => {
            panic!("Unexpected result.");
        }
    }
}
