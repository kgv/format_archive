use proc_macro::TokenStream;

/// Lazy format macro
///
/// Elision rules:
///
/// - has only [Debug](core::fmt::Debug) formats - [Debug]
/// - has formats of only one kind from: [Binary](core::fmt::Binary),
///   [LowerExp](core::fmt::LowerExp), [LowerHex](core::fmt::LowerHex),
///   [Octal](core::fmt::Octal), [Pointer](core::fmt::Pointer),
///   [UpperExp](core::fmt::UpperExp) or [UpperHex](core::fmt::UpperHex) -
///   Binary, LowerExp, LowerHex, Octal, Pointer, UpperExp or UpperHex
///   respectively
/// - otherwise - Display
#[proc_macro]
pub fn lazy_format(input: TokenStream) -> TokenStream {
    lazy_format::proc_macro(input)
}

mod lazy_format;
