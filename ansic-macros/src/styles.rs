// Parsing of styles and colours

use crate::{AnsiArg, detect::error};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BasicAnsiStyle {
    // Styles
    Reset = 0,
    Bold = 1,
    Dim = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    RapidBlink = 6,
    Invert = 7,
    Hidden = 8,
    Strikethrough = 9,

    // Foreground Colors
    FgBlack = 30,
    FgRed = 31,
    FgGreen = 32,
    FgYellow = 33,
    FgBlue = 34,
    FgMagenta = 35,
    FgCyan = 36,
    FgWhite = 37,

    // Bright Foreground
    FgBrightBlack = 90,
    FgBrightRed = 91,
    FgBrightGreen = 92,
    FgBrightYellow = 93,
    FgBrightBlue = 94,
    FgBrightMagenta = 95,
    FgBrightCyan = 96,
    FgBrightWhite = 97,

    // Background Colors
    BgBlack = 40,
    BgRed = 41,
    BgGreen = 42,
    BgYellow = 43,
    BgBlue = 44,
    BgMagenta = 45,
    BgCyan = 46,
    BgWhite = 47,

    // Bright Background
    BgBrightBlack = 100,
    BgBrightRed = 101,
    BgBrightGreen = 102,
    BgBrightYellow = 103,
    BgBrightBlue = 104,
    BgBrightMagenta = 105,
    BgBrightCyan = 106,
    BgBrightWhite = 107,
}

pub enum AnsiStyle {
    Basic(BasicAnsiStyle),
    FgRgb(u8, u8, u8),
    BgRgb(u8, u8, u8),
}

use AnsiStyle::*;
use BasicAnsiStyle::*;

impl AnsiStyle {
    pub fn to_color(bg: bool, bright: bool, name: &str, rgb: Option<(u8, u8, u8)>) -> Option<Self> {
        Some(Basic(match name.to_ascii_lowercase().as_str() {
            "black" => match (bg, bright) {
                (false, false) => FgBlack,
                (false, true) => FgBrightBlack,
                (true, false) => BgBlack,
                (true, true) => BgBrightBlack,
            },
            "red" => match (bg, bright) {
                (false, false) => FgRed,
                (false, true) => FgBrightRed,
                (true, false) => BgRed,
                (true, true) => BgBrightRed,
            },
            "green" => match (bg, bright) {
                (false, false) => FgGreen,
                (false, true) => FgBrightGreen,
                (true, false) => BgGreen,
                (true, true) => BgBrightGreen,
            },
            "yellow" => match (bg, bright) {
                (false, false) => FgYellow,
                (false, true) => FgBrightYellow,
                (true, false) => BgYellow,
                (true, true) => BgBrightYellow,
            },
            "blue" => match (bg, bright) {
                (false, false) => FgBlue,
                (false, true) => FgBrightBlue,
                (true, false) => BgBlue,
                (true, true) => BgBrightBlue,
            },
            "magenta" => match (bg, bright) {
                (false, false) => FgMagenta,
                (false, true) => FgBrightMagenta,
                (true, false) => BgMagenta,
                (true, true) => BgBrightMagenta,
            },
            "cyan" => match (bg, bright) {
                (false, false) => FgCyan,
                (false, true) => FgBrightCyan,
                (true, false) => BgCyan,
                (true, true) => BgBrightCyan,
            },
            "white" => match (bg, bright) {
                (false, false) => FgWhite,
                (false, true) => FgBrightWhite,
                (true, false) => BgWhite,
                (true, true) => BgBrightWhite,
            },
            "rgb" => {
                let (r, g, b) = rgb?;
                match bg {
                    true => return Some(BgRgb(r, g, b)),
                    false => return Some(FgRgb(r, g, b)),
                }
            }
            _ => return None,
        }))
    }

    pub fn to_style(name: &str) -> Option<Self> {
        Some(Basic(match name.to_ascii_lowercase().as_str() {
            "reset" => Reset,
            "bold" => Bold,
            "dim" => Dim,
            "italic" => Italic,
            "underline" => Underline,
            "blink" => Blink,
            "rapidblink" => RapidBlink,
            "invert" => Invert,
            "hidden" => Hidden,
            "strikethrough" => Strikethrough,
            "st" => Strikethrough,
            _ => return None,
        }))
    }

    pub fn code(self) -> String {
        match self {
            Basic(code) => (code as u8).to_string(),
            FgRgb(r, g, b) => format!("38;2;{r};{g};{b}"),
            BgRgb(r, g, b) => format!("48;2;{r};{g};{b}"),
        }
    }
}
