# Ansic
*The most efficient and easy ansi styling crate ever!*

Ansic is a crate which adds the ansi! proc macro, which allows for easy, simple and zero cost styling which happens at compiletime, in addition to other ansi utilities. (with default and 100% support for `#[no_std]`)

## Features:
- Compiletime styling with proc macro
- Zero cost at runtime
- Encoded in static str's

## Why ansic?
There are tons of other ansi styling crates out there so why `ansic`?
Ansic is supposed to be for people which are annoyed with the weird nesting and reuseability pattern of other crates and
which doesn't like that on every single print and use of the ANSI styles, it needs to be processed to get the final style.
`ansic` solves this is a clean, quick and easy way.

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
const RESET: &str = ansi!(reset);

fn main() {
    println!("{ERROR}[ERROR]: Hello, world!{RESET}");
}
```

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
| ANSI Reset Handling         | Automatic ✅       | Mostly ⚠️           | Manual ⚠️            |
| Text Injection              | Yes ✅             | Yes ✅               | Yes ✅               |
| Extensibility / Custom DSL  | Yes ✅             | No ❌                | No ❌                |
| Windows Compatibility       | Can be enabled ✅ (via vt_mode!()) | Yes ✅               | Yes ✅               |
| Well-maintained             | New, active 🚧    | Yes ✅               | Mostly deprecated ⚠️ |
