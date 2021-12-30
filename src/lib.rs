#![cfg(windows)]
pub mod brush;
pub mod color;
pub mod enums;
pub mod graphics;
pub mod init;
mod macros;
pub mod pen;
pub mod types;

pub use crate::brush::SolidBrush;
pub use crate::color::Color;
pub use crate::graphics::Graphics;
pub use crate::init::GdiPlus;
pub use crate::pen::Pen;
pub use crate::types::{Error, Point, Result};
