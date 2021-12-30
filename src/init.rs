use std::mem::MaybeUninit;

use gdiplus_sys2::{GdiplusShutdown, GdiplusStartup, GdiplusStartupInput, GdiplusStartupOutput};
use winapi::shared::minwindef::FALSE;
use winapi::shared::ntdef::NULL;

use crate::return_iferror;
use crate::types::Result;

pub struct GdiPlus {
    token: usize,
    input: Option<Box<GdiplusStartupInput>>,
    output: Option<Box<GdiplusStartupOutput>>,
}
impl GdiPlus {
    pub fn startup(
        input: Option<Box<GdiplusStartupInput>>,
        mut output: Option<Box<GdiplusStartupOutput>>,
    ) -> Result<GdiPlus> {
        let mut token = MaybeUninit::uninit();
        let input = input.unwrap_or_else(|| {
            Box::new(GdiplusStartupInput {
                DebugEventCallback: None,
                GdiplusVersion: 1,
                SuppressBackgroundThread: FALSE,
                SuppressExternalCodecs: FALSE,
            })
        });

        if let Some(ref mut output) = output {
            return_iferror!(GdiplusStartup(
                token.as_mut_ptr(),
                input.as_ref(),
                output.as_mut()
            ));
        } else {
            return_iferror!(GdiplusStartup(
                token.as_mut_ptr(),
                input.as_ref(),
                NULL as _
            ));
        };

        let token = unsafe { token.assume_init() };

        Ok(GdiPlus {
            token,
            input: Some(input),
            output,
        })
    }

    pub fn shutdown(&self) {
        unsafe {
            GdiplusShutdown(self.token);
        }
    }
}
impl Drop for GdiPlus {
    fn drop(&mut self) {
        self.shutdown();
    }
}
