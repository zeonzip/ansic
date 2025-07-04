# Ansic
*The most efficient and easy ansi styling crate ever!*

Ansic is a crate which adds the ansi! proc macro, which allows for easy, simple and zero cost styling which happens at compiletime, in addition to other ansi utilities. (with default and 100% support for `#[no_std]`)

## Features:
- Convenient and easy DSL syntax
- Compiletime styling with proc macro
- Zero cost at runtime
- Encoded in static str's
- #[no_std]

## Usage:
To make a red foreground, bold and underline ansi
```rust
ansi!(red bold underline)
```

All color arguments can take these arguments:
- "br" - bright
- "bg" - background
(rgb can only take bg)

You can chain them like this if we want a red background:

```rust
ansi!(bg.red bold underline)
```

Or if you want a red bright background you can do this:
```rust
ansi!(bg.br.red bold underline)
```

In addition we can add a foreground color now that we have a background one. Let's add green!
```rust
ansi!(bg.br.red green bold underline)
```

Idiomatic ansic syntax is also storing styles in constants and using them to style in a much less verbose way:

```rust
const ERROR: &str = ansi!(br.red bold underline italic);
const R: &str = ansi!(reset);

fn main() {
    println!("{ERROR}[ERROR]: Hello, world!{R}");
}
```

Ansic also supports full RGB styles with the color syntax "rgb(r, g, b)".

Ansic has alot more styles which you can find on our docs.rs page: [Ansic on Docs.rs](https://docs.rs/ansic);
Ansic also has util macros and functions for more convenient use (listed under Comparisons)

## Comparisons

| Feature                     | Ansic ✅           | owo-colors ⚠️/✅     | ansi_term ❌/⚠️      |
|-----------------------------|--------------------|------------------------|------------------------|
| FULLY Compile Time Generation | Yes ✅          | No ❌                 | No ❌                 |
| Zero Runtime Cost           | Yes ✅             | No ❌                | No ❌                |
| Minimal Binary Size         | Zero extra ✅         | Very Low ⚠️               | Medium ⚠️            |
| Supports RGB Styles         | Yes ✅             | Yes ✅               | No ❌                |
| No-Std support              | Yes ✅             | Yes ✅               | Yes ✅               |
| Reusable Style Constants    | Yes ✅             | Awkward ⚠️          | Partial ⚠️           |
| Simple Macro DSL            | Yes ✅             | No ❌                | No ❌                |
| ANSI Reset Handling         | Via `styled!` from `utils` ⚠️       | Automatic ✅           | Manual ⚠️            |
| Text Injection              | Yes ✅             | Yes ✅               | Yes ✅               |
| Extensibility / Custom DSL  | Yes ✅             | No ❌                | No ❌                |
| Windows Compatibility       | Can be enabled ✅ (via vt_mode!()) | Yes ✅               | Yes ✅               |
| Well-maintained             | New, active 🚧    | Yes ✅               | Mostly deprecated ⚠️ |


## Why ansic?
There are tons of other ansi styling crates out there, so why `ansic`?
Ansic is for people who need a ANSI styling crate, which more efficient, easier for maintainability or `#[no_std]`.
Ansic solves this with a simple and very readable reuseability pattern, proc macro DSL, and being fully compile time letting you live without the stress of bigger binary sizes.
