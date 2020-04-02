use proc_macro::TokenStream;

/// Lazy format macro
///
/// [`lazy_format!`] is syntax sugar of `Display`.
///
/// The first form of [`lazy_format!`] receives closure as the only one
/// argument.
///
/// ```text
/// lazy_format!(|f| ...);
/// ```
///
/// it expands to:
///
/// ```text
/// Display(move |f| ...);
/// ```
///
/// The second form of [`lazy_format!`] has a syntax identical to the syntax of
/// [`format!`](std::fmt::format). See [`fmt`](std::fmt) for more information.
///
/// ```text
/// lazy_format!("...", arg0, arg1, ...);
/// ```
/// 
/// it expands to:
/// 
/// ```text
/// Display(move |f| {
///     write!(f, "...", arg0, arg1, ...)
/// });
/// ```
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
