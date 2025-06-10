/// Ideal when you want to easily style a dynamic string and terminate it and allocate a String for it at runtime.
/// This is not a fully compiletime or no_std option as it allows for dynamic strings as text. If you want fully compile time, refer to the ansi! macro.
/// ## Usage:
/// ```rust
/// styled!("myString", br.red bold underline)
/// ```
///
/// In place of the "myString" literal can be anything that implemements Display.
#[macro_export]
macro_rules! styled {
    ($text:expr, $($style:tt)+) => {{
        format!("{}{}{}", ansi!($($style)+), $text, ansi!(reset))
    }}
}
