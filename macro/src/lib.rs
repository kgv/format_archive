use proc_macro::TokenStream;

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
#[proc_macro]
pub fn lazy_format(input: TokenStream) -> TokenStream {
    lazy_format::proc_macro(input)
}

mod lazy_format;
