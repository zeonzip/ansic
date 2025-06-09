use crate::detect::error;

pub enum AnsicMacroError<'a> {
    Unreachable,
    MultipleColorStyleArgs,
    InvalidRgbArg(i32, &'a str),
    MissingRgbArg(i32),
    InvalidStyleAndColor(&'a str),
    NoStyleOrColorTarget,
    RgbArgNotU8(&'a str),
    ExpectedRgbSyntax,
}

use AnsicMacroError::*;

impl<'a> Into<syn::Error> for AnsicMacroError<'a> {
    fn into(self) -> syn::Error {
        match self {
            Unreachable => error("Unreachable Error!"),
            MultipleColorStyleArgs => error("Gave multiple color/style arguments!"),
            InvalidRgbArg(index, content) => error(
                format!(
                    "Argument {} isn't a valid RGB argument! (content: {})",
                    index + 1,
                    content,
                )
                .as_str(),
            ),
            MissingRgbArg(index) => {
                error(format!("Missing the index {} argument for RGB colour.", index + 1).as_str())
            }
            InvalidStyleAndColor(name) => {
                error(format!("{name} isn't a valid style/colour.").as_str())
            }
            NoStyleOrColorTarget => error("No valid style or color target!"),
            RgbArgNotU8(arg) => {
                error(format!("RGB argument wasn't of u8. Found: {}", arg).as_str())
            }
            ExpectedRgbSyntax => error("Expected RGB syntax: rgb(r, g, b)."),
        }
    }
}
