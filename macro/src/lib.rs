use self::lazy_format::LazyFormat;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::ToTokens;
use syn::parse_macro_input;

/// Lazy format macro
///
/// [`lazy_format!`] is syntax sugar of `Display`.
///
/// The first form of [`lazy_format!`] receives closure as the only one
/// argument.
///
/// ```ignore
/// lazy_format!(|f| ...);
/// ```
///
/// it expands to:
///
/// ```ignore
/// Display(move |f| ...);
/// ```
///
/// The second form of [`lazy_format!`] has a syntax identical to the syntax of
/// [`format!`](std::fmt::format). See [`fmt`](std::fmt) for more information.
///
/// ```ignore
/// lazy_format!("...", arg0, arg1, ...);
/// ```
///
/// it expands to:
///
/// ```ignore
/// Display(move |f| write!(f, "...", arg0, arg1, ...));
/// ```
#[cfg_attr(feature = "nightly", proc_macro)]
#[cfg_attr(not(feature = "nightly"), proc_macro_hack)]
pub fn lazy_format(input: TokenStream) -> TokenStream {
    let lazy_format = parse_macro_input!(input as LazyFormat);
    lazy_format.into_token_stream().into()
}

mod lazy_format;
