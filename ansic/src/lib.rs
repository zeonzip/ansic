#![no_std]

/// Contains optional utilities like vt_mode!() and styled!()
#[cfg(feature = "utils")]
pub mod utils;

/// # The ansi! macro
///
/// The proc macro directly returns the harcoded string result of the DSL syntax and styling you provided fully at compile time.
///
/// Syntax:
///
/// Each styling argument is passed to the ansi macro separated by a space as such:
/// ```rust
/// ansi!(red bold);
/// ```
///
/// Each styling argument (which has a color target) may also have any combination of these arguments:
/// - "br" - sets the target as a bright color
/// - "bg" - sets the color as the background color
///
/// Example with a red bright background and a bold style:
/// ```rust
/// ansi!(bg.br.red bold);
/// ```
///
/// (foreground is the default of written colors so to specify a red foreground you can just write "red")
///
/// We also Support rgb with the rgb(r, g, b) syntax like this:
/// ```rust
/// ansi!(rgb(255, 34, 55));
/// ```
///
/// Idiomatic ansic syntax is also storing styles in constants and using them to style in a much less verbose way:
/// ```rust
/// const ERROR: &str = ansi!(br.red bold underline italic);
/// const RESET: &str = ansi!(reset);
///
/// fn main() {
///     println!("{ERROR}[ERROR]: Hello, world!{RESET}");
/// }
/// ```
///
/// Rgb colors may also take bg as an argument but br won't have an effect
/// ```rust
/// ansi!(bg.rgb(255, 34, 55))
/// ```
///
/// ## All styles (doesnt support any arguments):
/// - reset
/// - bold
/// - dim
/// - italic
/// - underline
/// - blink
/// - rapidblink (mostly deprecated)
/// - invert
/// - hidden
/// - strikethrough (has alias: st)
///
/// ## All colors (supports br and bg arguments):
/// - black
/// - red
/// - green
/// - yellow
/// - blue
/// - magenta
/// - cyan
/// - white
///
/// Rgb color 24 bits (supports bg argument):
/// - rgb(r, g, b)
pub use ansic_macros::ansi;
