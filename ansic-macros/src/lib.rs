/*
 * Hey! This code does need a cleanup and some comments, so if you come over this
 * and is looking to contribute, this is a good place to start!
*/

use detect::{ArgumentType, error, parse_arg};
use error::AnsicMacroError;
use proc_macro::{Delimiter, Literal, TokenStream, TokenTree};
use styles::AnsiStyle;

mod detect;
mod error;
mod styles;

const ANSI_PREFIX: &'static str = "\x1b[";

struct AnsiArg<'a> {
    pub name: &'a str,
    pub rgb: Option<(u8, u8, u8)>,
    pub bright: bool,
    pub bg: bool,
}

impl<'a> AnsiArg<'a> {
    fn generate_ansi(self) -> Option<String> {
        if let Some(style) = AnsiStyle::to_style(self.name) {
            return Some(style.code());
        } else if let Some(color) = AnsiStyle::to_color(self.bg, self.bright, self.name, self.rgb) {
            return Some(color.code());
        }

        None
    }
}

fn generate_ansiarg(args: Vec<ArgumentType>, name: &str, rgb: Option<(u8, u8, u8)>) -> AnsiArg {
    let mut profile = AnsiArg {
        name,
        bright: false,
        bg: false,
        rgb,
    };

    for arg in args {
        match arg {
            ArgumentType::Background => profile.bg = true,
            ArgumentType::Bright => profile.bright = true,
        }
    }

    profile
}

fn generate(args: Vec<String>) -> Result<String, syn::Error> {
    let mut args_iter = args.iter().peekable();

    let mut name: Option<&str> = None;
    let mut rgb: Option<(u8, u8, u8)> = None;
    let mut args: Vec<ArgumentType> = Vec::new();

    while let Some(arg) = args_iter.next() {
        if arg == "rgb" {
            if let Some(_) = name {
                return Err(AnsicMacroError::MultipleColorStyleArgs.into());
            }

            name = Some(arg);
            let (mut r, mut g, mut b): (u8, u8, u8) = (0, 0, 0);

            for i in 0..3 {
                if let Some(res) = args_iter.next() {
                    if let Ok(num) = res.parse::<u8>() {
                        match i {
                            0 => r = num,
                            1 => g = num,
                            2 => b = num,
                            _ => return Err(AnsicMacroError::Unreachable.into()),
                        }
                    } else {
                        return Err(AnsicMacroError::InvalidRgbArg(i, res).into());
                    }
                } else {
                    return Err(AnsicMacroError::MissingRgbArg(i).into());
                }
            }

            rgb = Some((r, g, b));
        } else if let Some(color) = AnsiStyle::to_color(false, false, arg, None) {
            if let Some(_) = name {
                return Err(AnsicMacroError::MultipleColorStyleArgs.into());
            }

            name = Some(arg)
        } else if let Some(style) = AnsiStyle::to_style(arg) {
            if let Some(_) = name {
                return Err(AnsicMacroError::MultipleColorStyleArgs.into());
            }

            name = Some(arg)
        } else if let Some(targ) = parse_arg(arg) {
            args.push(targ);
        } else {
            return Err(AnsicMacroError::InvalidStyleAndColor(arg).into());
        }
    }

    if let Some(fname) = name {
        let arg = generate_ansiarg(args, fname, rgb);

        if let Some(data) = arg.generate_ansi() {
            return Ok(data);
        }

        return Err(AnsicMacroError::Unreachable.into());
    }

    Err(AnsicMacroError::NoStyleOrColorTarget.into())
}

fn generate_ansi(args: Vec<Vec<String>>) -> Result<String, syn::Error> {
    let mut ansi_code = ANSI_PREFIX.to_string();

    let argslen = args.len();

    for (i, arg) in args.into_iter().enumerate() {
        let parsed = generate(arg)?;

        if argslen - 1 == i {
            ansi_code.push_str(&format!("{}m", parsed));
        } else {
            ansi_code.push_str(&format!("{};", parsed));
        }
    }

    Ok(ansi_code)
}

fn generate_tokens(tokens: TokenStream) -> Result<Vec<Vec<String>>, syn::Error> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    let mut token_iter = tokens.into_iter().peekable();

    while let Some(token) = token_iter.next() {
        match &token {
            TokenTree::Ident(ident) => {
                current.push(ident.to_string());

                if ident.to_string() == "rgb" {
                    match token_iter.peek() {
                        Some(TokenTree::Group(parens)) => {
                            if parens.delimiter() == Delimiter::Parenthesis {
                                // pstream is the tokenstream inside  theparenthesis
                                let pstream = parens.stream();
                                let mut stream_iter = pstream.into_iter().peekable();

                                while let Some(token) = stream_iter.next() {
                                    match token {
                                        TokenTree::Literal(lit) => {
                                            let s = lit.to_string();

                                            if let Ok(num) = s.parse::<u8>() {
                                                current.push(s);

                                                match stream_iter.peek() {
                                                    Some(TokenTree::Punct(p))
                                                        if p.as_char() == ',' =>
                                                    {
                                                        stream_iter.next();
                                                    }
                                                    _ => {
                                                        result.push(current);
                                                        current = Vec::new();
                                                    }
                                                }
                                            } else {
                                                return Err(AnsicMacroError::RgbArgNotU8(&s).into());
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(AnsicMacroError::ExpectedRgbSyntax.into());
                        }
                    }

                    continue;
                }

                match token_iter.peek() {
                    Some(TokenTree::Punct(p)) if p.as_char() == '.' => {
                        token_iter.next();
                    }
                    _ => {
                        result.push(current);
                        current = Vec::new();
                    }
                }
            }
            _ => {}
        }
    }

    Ok(result)
}

#[proc_macro]
pub fn ansi(input: TokenStream) -> TokenStream {
    let tokens = generate_tokens(input);

    if let Ok(args) = tokens {
        let result = generate_ansi(args);

        if let Ok(ansi_code) = result {
            let lit = TokenTree::Literal(Literal::string(&ansi_code));
            return TokenStream::from(lit);
        } else if let Err(err) = result {
            return err.to_compile_error().into();
        } else {
            let err: syn::Error = AnsicMacroError::Unreachable.into();

            return err.to_compile_error().into();
        }
    } else if let Err(err) = tokens {
        return err.to_compile_error().into();
    } else {
        let err: syn::Error = AnsicMacroError::Unreachable.into();

        return err.to_compile_error().into();
    }
}
