use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, ExprClosure, Ident, LitStr, Result, Token,
};

pub(super) enum LazyFormat {
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

pub(super) struct Closure(ExprClosure);

impl Parse for Closure {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Closure(input.parse()?))
    }
}

impl ToTokens for Closure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr_closure = &self.0;
        let closure_tokens = quote! {
            format::Display(#expr_closure)
        };
        closure_tokens.to_tokens(tokens);
    }
}

pub(super) struct Format {
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
        let format = &self.format;
        let comma_token = self.comma_token;
        let args = &self.args;
        let lazy_format_tokens = quote! {
            format::Display(move |f| -> core::fmt::Result {
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
