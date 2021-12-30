#![cfg(windows)]
#![windows_subsystem = "windows"]

mod lib;

use std::ptr;

use gdiplus::enums::SmoothingMode;
use gdiplus::{color, Color, GdiPlus, Graphics, Pen, Result as GdipResult, SolidBrush};
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;

use crate::lib::*;

const WINDOW_CLASS_NAME: &str = "test_draw_line";
const WINDOW_TITLE: &str = "Test draw line";

extern "system" fn wnd_proc(hwnd: HWND, message: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = wpanic_ifnull!(BeginPaint(hwnd, &mut ps));

            let mut rect = RECT::default();
            wpanic_ifeq!(GetClientRect(hwnd, &mut rect), FALSE);

            rect.left -= 1;
            rect.top -= 1;
            rect.right += 1;
            rect.bottom += 1;

            (|| -> GdipResult<()> {
                let mut graphics = Graphics::from_hdc(hdc)?;

                graphics
                    .with_brush(&mut SolidBrush::new(&Color::from(color::LIGHT_GRAY))?)
                    .fill_rectangle(
                        (rect.left as _, rect.top as _),
                        rect.right as _,
                        rect.bottom as _,
                    )?;

                graphics.set_smoothing_mode(SmoothingMode::AntiAlias)?;

                let mut pen = Pen::new(&Color::from(color::RED), 2.5)?;
                let mut pen2 = pen.try_clone()?;

                let last_pos = graphics
                    .with_pen(&mut pen)
                    .move_to((100.0, 100.0))
                    .line_to((150.0, 135.0))?
                    .modify(|pen| {
                        pen.set_color(&Color::from(color::LIME_GREEN))?
                            .set_width(1.0)?;
                        Ok(())
                    })?
                    .line_to((150.0, 160.0))?
                    .replace(pen2.set_width(1.5)?)
                    .line_to((170.0, 145.0))?
                    .modify(|pen| {
                        pen.set_color(&Color::from(color::PLUM))?;
                        Ok(())
                    })?
                    .draw_line((150.0, 170.0), (140.0, 160.0))?
                    .current_pos();

                graphics
                    .with_pen(&mut pen)
                    .move_to(last_pos)
                    .line_to((150.0, 50.0))?;

                Ok(())
            })()
            .unwrap();

            wpanic_ifeq!(EndPaint(hwnd, &ps), FALSE);
        }
        _ => {}
    }
    unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
}

fn main() {
    wpanic_ifeq!(
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2),
        FALSE
    );

    let _gdiplus = GdiPlus::startup(None, None).unwrap();

    let h_inst = wpanic_ifisnull!(GetModuleHandleW(ptr::null()));

    let class = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
        lpfnWndProc: Some(wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: h_inst,
        hIcon: ptr::null_mut(),
        hCursor: unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) },
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null(),
        lpszClassName: wide_string(WINDOW_CLASS_NAME).as_ptr(),
    };

    wpanic_ifeq!(RegisterClassW(&class), 0);

    let hwnd = wpanic_ifisnull!(CreateWindowExW(
        0,
        wide_string(WINDOW_CLASS_NAME).as_ptr(),
        wide_string(WINDOW_TITLE).as_ptr(),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        500,
        500,
        ptr::null_mut(),
        ptr::null_mut(),
        h_inst,
        ptr::null_mut()
    ));

    wpanic_ifisnull!(CreateWindowExW(
        0,
        wide_string("BUTTON").as_ptr(),
        wide_string("Button").as_ptr(),
        WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
        4,
        40,
        100,
        30,
        hwnd,
        ptr::null_mut(),
        h_inst,
        ptr::null_mut(),
    ));

    let mut msg: MSG = unsafe { std::mem::zeroed() };
    unsafe {
        while GetMessageW(&mut msg, hwnd, 0, 0) == TRUE {
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }
}
