use gdiplus_sys2::{Color_AlphaShift, Color_BlueShift, Color_GreenShift, Color_RedShift};
use winapi::shared::windef::COLORREF;

const RGB_MASK: u32 = 0x00ffffff;
const ALPHA_MASK: u32 = 0xff000000;
const RED_MASK: u32 = 0x00ff0000;
const GREEN_MASK: u32 = 0x0000ff00;
const BLUE_MASK: u32 = 0x000000ff;

type AlphaColorTuple = (u8, u8, u8, u8);
type ColorTuple = (u8, u8, u8);

pub struct Color {
    color: u32,
}
impl Color {
    pub fn alpha(&self) -> u8 {
        ((self.color & ALPHA_MASK) >> Color_AlphaShift) as _
    }

    pub fn red(&self) -> u8 {
        ((self.color & RED_MASK) >> Color_RedShift) as _
    }

    pub fn green(&self) -> u8 {
        ((self.color & GREEN_MASK) >> Color_GreenShift) as _
    }

    pub fn blue(&self) -> u8 {
        ((self.color & BLUE_MASK) >> Color_BlueShift) as _
    }

    pub fn argb(&self) -> u32 {
        self.color
    }

    pub fn rgb(&self) -> u32 {
        self.color & RGB_MASK
    }

    pub fn colorref(&self) -> COLORREF {
        self.rgb()
    }
}
impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self { color }
    }
}
impl From<AlphaColorTuple> for Color {
    fn from(val: AlphaColorTuple) -> Self {
        Self {
            color: (val.0 as u32) << Color_AlphaShift
                | (val.1 as u32) << Color_RedShift
                | (val.2 as u32) << Color_GreenShift
                | val.3 as u32,
        }
    }
}
impl From<ColorTuple> for Color {
    fn from(val: ColorTuple) -> Self {
        Self {
            color: ALPHA_MASK
                | (val.0 as u32) << Color_RedShift
                | (val.1 as u32) << Color_GreenShift
                | val.2 as u32,
        }
    }
}

pub const ALICE_BLUE: u32 = 0xfff0f8ff;
pub const ANTIQUE_WHITE: u32 = 0xfffaebd7;
pub const AQUA: u32 = 0xff00ffff;
pub const AQUAMARINE: u32 = 0xff7fffd4;
pub const AZURE: u32 = 0xfff0ffff;
pub const BEIGE: u32 = 0xfff5f5dc;
pub const BISQUE: u32 = 0xffffe4c4;
pub const BLACK: u32 = 0xff000000;
pub const BLANCHED_ALMOND: u32 = 0xffffebcd;
pub const BLUE: u32 = 0xff0000ff;
pub const BLUE_VIOLET: u32 = 0xff8a2be2;
pub const BROWN: u32 = 0xffa52a2a;
pub const BURLY_WOOD: u32 = 0xffdeb887;
pub const CADET_BLUE: u32 = 0xff5f9ea0;
pub const CHARTREUSE: u32 = 0xff7fff00;
pub const CHOCOLATE: u32 = 0xffd2691e;
pub const CORAL: u32 = 0xffff7f50;
pub const CORNFLOWER_BLUE: u32 = 0xff6495ed;
pub const CORNSILK: u32 = 0xfffff8dc;
pub const CRIMSON: u32 = 0xffdc143c;
pub const CYAN: u32 = 0xff00ffff;
pub const DARK_BLUE: u32 = 0xff00008b;
pub const DARK_CYAN: u32 = 0xff008b8b;
pub const DARK_GOLDENROD: u32 = 0xffb8860b;
pub const DARK_GRAY: u32 = 0xffa9a9a9;
pub const DARK_GREEN: u32 = 0xff006400;
pub const DARK_KHAKI: u32 = 0xffbdb76b;
pub const DARK_MAGENTA: u32 = 0xff8b008b;
pub const DARK_OLIVE_GREEN: u32 = 0xff556b2f;
pub const DARK_ORANGE: u32 = 0xffff8c00;
pub const DARK_ORCHID: u32 = 0xff9932cc;
pub const DARK_RED: u32 = 0xff8b0000;
pub const DARK_SALMON: u32 = 0xffe9967a;
pub const DARK_SEA_GREEN: u32 = 0xff8fbc8b;
pub const DARK_SLATE_BLUE: u32 = 0xff483d8b;
pub const DARK_SLATE_GRAY: u32 = 0xff2f4f4f;
pub const DARK_TURQUOISE: u32 = 0xff00ced1;
pub const DARK_VIOLET: u32 = 0xff9400d3;
pub const DEEP_PINK: u32 = 0xffff1493;
pub const DEEP_SKY_BLUE: u32 = 0xff00bfff;
pub const DIM_GRAY: u32 = 0xff696969;
pub const DODGER_BLUE: u32 = 0xff1e90ff;
pub const FIREBRICK: u32 = 0xffb22222;
pub const FLORAL_WHITE: u32 = 0xfffffaf0;
pub const FOREST_GREEN: u32 = 0xff228b22;
pub const FUCHSIA: u32 = 0xffff00ff;
pub const GAINSBORO: u32 = 0xffdcdcdc;
pub const GHOST_WHITE: u32 = 0xfff8f8ff;
pub const GOLD: u32 = 0xffffd700;
pub const GOLDENROD: u32 = 0xffdaa520;
pub const GRAY: u32 = 0xff808080;
pub const GREEN: u32 = 0xff008000;
pub const GREEN_YELLOW: u32 = 0xffadff2f;
pub const HONEYDEW: u32 = 0xfff0fff0;
pub const HOT_PINK: u32 = 0xffff69b4;
pub const INDIAN_RED: u32 = 0xffcd5c5c;
pub const INDIGO: u32 = 0xff4b0082;
pub const IVORY: u32 = 0xfffffff0;
pub const KHAKI: u32 = 0xfff0e68c;
pub const LAVENDER: u32 = 0xffe6e6fa;
pub const LAVENDER_BLUSH: u32 = 0xfffff0f5;
pub const LAWN_GREEN: u32 = 0xff7cfc00;
pub const LEMON_CHIFFON: u32 = 0xfffffacd;
pub const LIGHT_BLUE: u32 = 0xffadd8e6;
pub const LIGHT_CORAL: u32 = 0xfff08080;
pub const LIGHT_CYAN: u32 = 0xffe0ffff;
pub const LIGHT_GOLDENROD_YELLOW: u32 = 0xfffafad2;
pub const LIGHT_GRAY: u32 = 0xffd3d3d3;
pub const LIGHT_GREEN: u32 = 0xff90ee90;
pub const LIGHT_PINK: u32 = 0xffffb6c1;
pub const LIGHT_SALMON: u32 = 0xffffa07a;
pub const LIGHT_SEA_GREEN: u32 = 0xff20b2aa;
pub const LIGHT_SKY_BLUE: u32 = 0xff87cefa;
pub const LIGHT_SLATE_GRAY: u32 = 0xff778899;
pub const LIGHT_STEEL_BLUE: u32 = 0xffb0c4de;
pub const LIGHT_YELLOW: u32 = 0xffffffe0;
pub const LIME: u32 = 0xff00ff00;
pub const LIME_GREEN: u32 = 0xff32cd32;
pub const LINEN: u32 = 0xfffaf0e6;
pub const MAGENTA: u32 = 0xffff00ff;
pub const MAROON: u32 = 0xff800000;
pub const MEDIUM_AQUAMARINE: u32 = 0xff66cdaa;
pub const MEDIUM_BLUE: u32 = 0xff0000cd;
pub const MEDIUM_ORCHID: u32 = 0xffba55d3;
pub const MEDIUM_PURPLE: u32 = 0xff9370db;
pub const MEDIUM_SEA_GREEN: u32 = 0xff3cb371;
pub const MEDIUM_SLATE_BLUE: u32 = 0xff7b68ee;
pub const MEDIUM_SPRING_GREEN: u32 = 0xff00fa9a;
pub const MEDIUM_TURQUOISE: u32 = 0xff48d1cc;
pub const MEDIUM_VIOLET_RED: u32 = 0xffc71585;
pub const MIDNIGHT_BLUE: u32 = 0xff191970;
pub const MINT_CREAM: u32 = 0xfff5fffa;
pub const MISTY_ROSE: u32 = 0xffffe4e1;
pub const MOCCASIN: u32 = 0xffffe4b5;
pub const NAVAJO_WHITE: u32 = 0xffffdead;
pub const NAVY: u32 = 0xff000080;
pub const OLD_LACE: u32 = 0xfffdf5e6;
pub const OLIVE: u32 = 0xff808000;
pub const OLIVE_DRAB: u32 = 0xff6b8e23;
pub const ORANGE: u32 = 0xffffa500;
pub const ORANGE_RED: u32 = 0xffff4500;
pub const ORCHID: u32 = 0xffda70d6;
pub const PALE_GOLDENROD: u32 = 0xffeee8aa;
pub const PALE_GREEN: u32 = 0xff98fb98;
pub const PALE_TURQUOISE: u32 = 0xffafeeee;
pub const PALE_VIOLET_RED: u32 = 0xffdb7093;
pub const PAPAYA_WHIP: u32 = 0xffffefd5;
pub const PEACH_PUFF: u32 = 0xffffdab9;
pub const PERU: u32 = 0xffcd853f;
pub const PINK: u32 = 0xffffc0cb;
pub const PLUM: u32 = 0xffdda0dd;
pub const POWDER_BLUE: u32 = 0xffb0e0e6;
pub const PURPLE: u32 = 0xff800080;
pub const RED: u32 = 0xffff0000;
pub const ROSY_BROWN: u32 = 0xffbc8f8f;
pub const ROYAL_BLUE: u32 = 0xff4169e1;
pub const SADDLE_BROWN: u32 = 0xff8b4513;
pub const SALMON: u32 = 0xfffa8072;
pub const SANDY_BROWN: u32 = 0xfff4a460;
pub const SEA_GREEN: u32 = 0xff2e8b57;
pub const SEA_SHELL: u32 = 0xfffff5ee;
pub const SIENNA: u32 = 0xffa0522d;
pub const SILVER: u32 = 0xffc0c0c0;
pub const SKY_BLUE: u32 = 0xff87ceeb;
pub const SLATE_BLUE: u32 = 0xff6a5acd;
pub const SLATE_GRAY: u32 = 0xff708090;
pub const SNOW: u32 = 0xfffffafa;
pub const SPRING_GREEN: u32 = 0xff00ff7f;
pub const STEEL_BLUE: u32 = 0xff4682b4;
pub const TAN: u32 = 0xffd2b48c;
pub const TEAL: u32 = 0xff008080;
pub const THISTLE: u32 = 0xffd8bfd8;
pub const TOMATO: u32 = 0xffff6347;
pub const TRANSPARENT: u32 = 0x00ffffff;
pub const TURQUOISE: u32 = 0xff40e0d0;
pub const VIOLET: u32 = 0xffee82ee;
pub const WHEAT: u32 = 0xfff5deb3;
pub const WHITE: u32 = 0xffffffff;
pub const WHITE_SMOKE: u32 = 0xfff5f5f5;
pub const YELLOW: u32 = 0xffffff00;
pub const YELLOW_GREEN: u32 = 0xff9acd32;
