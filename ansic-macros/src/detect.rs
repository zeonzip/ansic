// Argument parsing

use crate::styles::AnsiStyle::{self, *};
use crate::styles::BasicAnsiStyle::*;
use proc_macro2::Span;

pub fn error(error: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), error)
}

// Different arguments for colours
pub enum ArgumentType {
    Bright,
    Background,
}

// parses arguments to their respective enum
pub fn parse_arg(target: &str) -> Option<ArgumentType> {
    Some(match target {
        "bg" => ArgumentType::Background,
        "br" => ArgumentType::Bright,
        _ => return None,
    })
}
