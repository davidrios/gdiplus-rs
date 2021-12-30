#![cfg(windows)]

use gdiplus::color::{self, Color};

#[test]
fn test_color() {
    let black = Color::from(color::BLACK).colorref();
    assert_eq!(black, 0);

    let white: u32 = Color::from((255, 255, 255)).argb();
    assert_eq!(white, 0xffffffff);

    let red: u32 = Color::from((255, 0, 0)).argb();
    assert_eq!(red, 0xffff0000);

    let green: u32 = Color::from((0, 255, 0)).argb();
    assert_eq!(green, 0xff00ff00);

    let blue: u32 = Color::from((0, 0, 255)).argb();
    assert_eq!(blue, 0xff0000ff);

    let transparent: u32 = Color::from((0, 255, 255, 255)).argb();
    assert_eq!(transparent, 0x00ffffff);
}
