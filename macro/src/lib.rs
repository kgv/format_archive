use proc_macro::TokenStream;

/// Lazy format macro
///
/// The first form receives closure as the only one argument.
///
/// ```
/// let a = "a";
/// lazy_format!(|f| write!(f, "{}", a));
/// ```
///
/// The second form is syntax sugar of the first form which is a closure with
/// the only one [write](std::write) macro. Its syntax is identical to the
/// [format](std::fmt::format) syntax.
///
/// ```
/// let a = "a";
/// lazy_format!("{}", a);
/// ```
///
/// The second form inferences the output format.
///
/// Format inference rules:
///
/// - has only [Debug](core::fmt::Debug) formats - Debug
/// - has formats of only one kind from: [Binary](core::fmt::Binary),
///   [LowerExp](core::fmt::LowerExp), [LowerHex](core::fmt::LowerHex),
///   [Octal](core::fmt::Octal), [Pointer](core::fmt::Pointer),
///   [UpperExp](core::fmt::UpperExp) or [UpperHex](core::fmt::UpperHex) -
///   Binary, LowerExp, LowerHex, Octal, Pointer, UpperExp or UpperHex
///   respectively
/// - otherwise - Display
///
/// # Examples
///
/// Nested Debug help struct example
///
/// ```
/// #![feature(proc_macro_hygiene)]
///
/// # use format_macro as format;
/// use format::lazy_format;
/// use std::fmt;
///
/// struct Foo {
///     bar: [u32; 9],
/// }
///
/// impl fmt::Debug for Foo {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         let bar = lazy_format!(|f| f.debug_list().entries(&self.bar).finish());
///         f.debug_struct("Foo").field("bar", &bar).finish()
///     }
/// }
/// ```
///
/// Control flow example
///
/// ```
/// #![feature(proc_macro_hygiene)]
///
/// # use format_macro as format;
/// use format::lazy_format;
/// use std::fmt;
///
/// struct Foo(usize);
///
/// impl fmt::Display for Foo {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         let bar = lazy_format!(|f| if f.alternate() {
///             write!(f, "{:#x}", self.0)
///         } else {
///             write!(f, "{:x}", self.0)
///         });
///         f.debug_tuple("Foo").field(&bar).finish()
///     }
/// }
/// ```
#[proc_macro]
pub fn lazy_format(input: TokenStream) -> TokenStream {
    lazy_format::proc_macro(input)
}

mod lazy_format;
