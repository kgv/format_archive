use proc_macro::TokenStream;

/// Lazy format macro
///
/// The first form receives closure as the only one argument.
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// # use format_core as format;
/// # use format_macro::lazy_format;
/// let a = "a";
/// lazy_format!(|f| write!(f, "{}", a));
/// ```
///
/// The second form is syntax sugar of the first form which is a closure with
/// the only one [`write!`](core::write). Its syntax is identical to the
/// [`format!`](std::fmt::format) syntax.
///
/// ```
/// # #![feature(proc_macro_hygiene)]
/// # use format_core as format;
/// # use format_macro::lazy_format;
/// let a = "a";
/// lazy_format!("{}", a);
/// ```
///
/// The second form inferences the output format.
///
/// Format inference rules:
///
/// - has only [`Debug`](core::fmt::Debug) formats - `Debug`
/// - has formats of only one kind from: [`Binary`](core::fmt::Binary),
///   [`LowerExp`](core::fmt::LowerExp), [`LowerHex`](core::fmt::LowerHex),
///   [`Octal`](core::fmt::Octal), [`Pointer`](core::fmt::Pointer),
///   [`UpperExp`](core::fmt::UpperExp) or [`UpperHex`](core::fmt::UpperHex) -
///   `Binary`, `LowerExp`, `LowerHex`, `Octal`, `Pointer`, `UpperExp` or
///   `UpperHex` respectively
/// - otherwise - `Display`
///
/// # Examples
///
/// Nested debug help struct
///
/// ```
/// #![feature(proc_macro_hygiene)]
///
/// # mod format {
/// #     pub use format_core::*;
/// #     pub use format_macro::lazy_format;
/// # }
/// # use std::format;
/// use core::fmt::{Debug, Formatter, Result};
/// use format::lazy_format;
///
/// struct Foo {
///     bar: [u32; 10],
/// }
///
/// impl Debug for Foo {
///     fn fmt(&self, f: &mut Formatter) -> Result {
///         let bar = lazy_format!(|f| f.debug_list().entries(&self.bar).finish());
///         f.debug_struct("Foo")
///             .field("bar", &format_args!("{}", bar))
///             .finish()
///     }
/// }
///
/// assert_eq!(
///     "Foo { bar: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] }",
///     format!("{:?}", Foo { bar: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] })
/// );
/// ```
///
/// Control flow
///
/// ```
/// #![feature(proc_macro_hygiene)]
///
/// # mod format {
/// #     pub use format_core::*;
/// #     pub use format_macro::lazy_format;
/// # }
/// # use std::format;
/// use core::fmt::{Debug, Formatter, Result};
/// use format::lazy_format;
///
/// struct Foo(usize);
///
/// impl Debug for Foo {
///     fn fmt(&self, f: &mut Formatter) -> Result {
///         let alternate = f.alternate();
///         let bar = lazy_format!(|f| if alternate {
///             write!(f, "{:#x}", self.0)
///         } else {
///             write!(f, "{:x}", self.0)
///         });
///         f.debug_tuple("Foo")
///             .field(&format_args!("{}", bar))
///             .finish()
///     }
/// }
///
/// assert_eq!("Foo(75bcd15)", format!("{:?}", Foo(0123456789)));
/// assert_eq!("Foo(\n    0x75bcd15,\n)", format!("{:#?}", Foo(0123456789)));
/// ```
#[proc_macro]
pub fn lazy_format(input: TokenStream) -> TokenStream {
    lazy_format::proc_macro(input)
}

mod lazy_format;
