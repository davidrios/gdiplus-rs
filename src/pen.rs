use std::mem::MaybeUninit;

use gdiplus_sys2::{
    GdipClonePen, GdipCreatePen1, GdipDeletePen, GdipGetPenColor, GdipGetPenWidth, GdipSetPenColor,
    GdipSetPenWidth, GpPen, REAL,
};

use crate::color::Color;
use crate::types::Result;
use crate::{panic_iferror, return_iferror};

pub struct Pen {
    pen: *mut GpPen,
}
impl Pen {
    pub(crate) fn pen(&self) -> *mut GpPen {
        self.pen
    }

    pub fn new(color: &Color, width: REAL) -> Result<Self> {
        let mut pen = MaybeUninit::uninit();

        return_iferror!(GdipCreatePen1(color.argb(), width, 0, pen.as_mut_ptr()));

        Ok(Self {
            pen: unsafe { pen.assume_init() },
        })
    }

    pub fn try_clone(&self) -> Result<Self> {
        let mut pen = MaybeUninit::uninit();
        return_iferror!(GdipClonePen(self.pen, pen.as_mut_ptr()));
        Ok(Self {
            pen: unsafe { pen.assume_init() },
        })
    }

    pub fn color(&self) -> Result<Color> {
        let mut argb = 0;
        return_iferror!(GdipGetPenColor(self.pen, &mut argb));

        Ok(Color::from(argb))
    }

    pub fn set_color(&mut self, color: &Color) -> Result<&mut Self> {
        return_iferror!(GdipSetPenColor(self.pen, color.argb()));
        Ok(self)
    }

    pub fn width(&self) -> Result<REAL> {
        let mut width = 0.0;
        return_iferror!(GdipGetPenWidth(self.pen, &mut width));

        Ok(width)
    }

    pub fn set_width(&mut self, width: REAL) -> Result<&mut Self> {
        return_iferror!(GdipSetPenWidth(self.pen, width));
        Ok(self)
    }
}
impl Drop for Pen {
    fn drop(&mut self) {
        panic_iferror!(GdipDeletePen(self.pen));
    }
}
