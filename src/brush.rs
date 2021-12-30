use std::mem::MaybeUninit;

use gdiplus_sys2::{GdipCloneBrush, GdipDeleteBrush, GdipGetPenBrushFill, GpBrush};

use crate::color::Color;
use crate::pen::Pen;
use crate::types::Result;
use crate::{panic_iferror, return_iferror};

pub struct SolidBrush {
    brush: *mut GpBrush,
}
impl SolidBrush {
    pub(crate) fn brush(&self) -> *mut GpBrush {
        self.brush
    }

    pub fn new(color: &Color) -> Result<Self> {
        let pen = Pen::new(color, 0.0)?;
        let mut brush = MaybeUninit::uninit();

        return_iferror!(GdipGetPenBrushFill(pen.pen(), brush.as_mut_ptr()));

        Ok(Self {
            brush: unsafe { brush.assume_init() },
        })
    }

    pub fn try_clone(&self) -> Result<Self> {
        let mut brush = MaybeUninit::uninit();
        return_iferror!(GdipCloneBrush(self.brush, brush.as_mut_ptr()));
        Ok(Self {
            brush: unsafe { brush.assume_init() },
        })
    }
}
impl Drop for SolidBrush {
    fn drop(&mut self) {
        panic_iferror!(GdipDeleteBrush(self.brush));
    }
}
