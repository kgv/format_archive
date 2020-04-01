use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ExprClosure, Ident, LitStr, Result, Token,
};

pub(super) fn proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lazy_format = parse_macro_input!(input as LazyFormat);
    lazy_format.into_token_stream().into()
}

enum LazyFormat {
    Closure(Closure),
    Format(Format),
}

impl Parse for LazyFormat {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            Ok(Self::Format(input.parse()?))
        } else {
            // TODO: More right error handling?
            Ok(Self::Closure(input.parse().map_err(|mut err| {
                err.combine(lookahead.error());
                err
            })?))
        }
    }
}

impl ToTokens for LazyFormat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Closure(closure) => {
                closure.to_tokens(tokens);
            }
            Self::Format(format) => {
                format.to_tokens(tokens);
            }
        }
    }
}

struct Closure(ExprClosure);

impl Parse for Closure {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Closure(input.parse()?))
    }
}

impl ToTokens for Closure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr_closure = &self.0;
        let closure_tokens = quote! {
            format_core::Display(#expr_closure)
        };
        closure_tokens.to_tokens(tokens);
    }
}

struct Format {
    format: LitStr,
    comma_token: Option<Token![,]>,
    args: Punctuated<Arg, Token![,]>,
}

impl Parse for Format {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Format {
            format: input.parse()?,
            comma_token: input.parse()?,
            args: input.parse_terminated(Arg::parse)?,
        })
    }
}

impl ToTokens for Format {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let kind: Kind = self.format.value().as_str().into();
        let format = &self.format;
        let comma_token = self.comma_token;
        let args = &self.args;
        let lazy_format_tokens = quote! {
            format_core::#kind(move |f| -> core::fmt::Result {
                core::write!(f, #format #comma_token #args)
            })
        };
        lazy_format_tokens.to_tokens(tokens);
    }
}

#[derive(Clone)]
enum Arg {
    Named {
        name: Ident,
        eq_token: Token![=],
        value: Expr,
    },
    Unnamed {
        value: Expr,
    },
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) && input.peek2(Token![=]) {
            Ok(Self::Named {
                name: input.parse()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Ok(Self::Unnamed {
                value: input.parse()?,
            })
        }
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Named {
                name,
                eq_token,
                value,
            } => {
                name.to_tokens(tokens);
                eq_token.to_tokens(tokens);
                value.to_tokens(tokens);
            }
            Self::Unnamed { value } => {
                value.to_tokens(tokens);
            }
        }
    }
}

enum Kind {
    Debug,
    Display,
    Binary,
    LowerExp,
    LowerHex,
    Octal,
    Pointer,
    UpperExp,
    UpperHex,
}

impl From<&str> for Kind {
    fn from(from: &str) -> Self {
        lazy_static! {
            static ref DEBUG: Regex = Regex::new(r#"\{.*:#?.*\?\}"#).unwrap();
            static ref DISPLAY: Regex = Regex::new(r#"\{\}"#).unwrap();
            static ref BINARY: Regex = Regex::new(r#"\{.*:#?.*b\}"#).unwrap();
            static ref LOWER_EXP: Regex = Regex::new(r#"\{.*:#?.*e\}"#).unwrap();
            static ref LOWER_HEX: Regex = Regex::new(r#"\{.*:#?.*x\}"#).unwrap();
            static ref OCTAL: Regex = Regex::new(r#"\{.*:#?.*o\}"#).unwrap();
            static ref POINTER: Regex = Regex::new(r#"\{.*:#?.*p\}"#).unwrap();
            static ref UPPER_EXP: Regex = Regex::new(r#"\{.*:#?.*E\}"#).unwrap();
            static ref UPPER_HEX: Regex = Regex::new(r#"\{.*:#?.*X\}"#).unwrap();
        }

        let debug = DEBUG.is_match(from);
        let display = DISPLAY.is_match(from);
        let binary = BINARY.is_match(from);
        let lower_exp = LOWER_EXP.is_match(from);
        let lower_hex = LOWER_HEX.is_match(from);
        let octal = OCTAL.is_match(from);
        let pointer = POINTER.is_match(from);
        let upper_exp = UPPER_EXP.is_match(from);
        let upper_hex = UPPER_HEX.is_match(from);
        match (
            debug, display, binary, lower_exp, lower_hex, octal, pointer, upper_exp, upper_hex,
        ) {
            (true, false, false, false, false, false, false, false, false) => Self::Debug,
            (_, _, true, false, false, false, false, false, false) => Self::Binary,
            (_, _, false, true, false, false, false, false, false) => Self::LowerExp,
            (_, _, false, false, true, false, false, false, false) => Self::LowerHex,
            (_, _, false, false, false, true, false, false, false) => Self::Octal,
            (_, _, false, false, false, false, true, false, false) => Self::Pointer,
            (_, _, false, false, false, false, false, true, false) => Self::UpperExp,
            (_, _, false, false, false, false, false, false, true) => Self::UpperHex,
            (_, _, _, _, _, _, _, _, _) => Self::Display,
        }
    }
}

impl ToTokens for Kind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Debug => quote!(Debug).to_tokens(tokens),
            Self::Display => quote!(Display).to_tokens(tokens),
            Self::Binary => quote!(Binary).to_tokens(tokens),
            Self::LowerExp => quote!(LowerExp).to_tokens(tokens),
            Self::LowerHex => quote!(LowerHex).to_tokens(tokens),
            Self::Octal => quote!(Octal).to_tokens(tokens),
            Self::Pointer => quote!(Pointer).to_tokens(tokens),
            Self::UpperExp => quote!(UpperExp).to_tokens(tokens),
            Self::UpperHex => quote!(UpperHex).to_tokens(tokens),
        }
    }
}
