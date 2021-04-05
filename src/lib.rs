//! A utility crate to make it easier to work with the
//! [`Formatter`](core::fmt::Formatter)
//!
//! # Examples
//!
//! Nested debug help struct:
//!
//! ```
//! use core::fmt::{Debug, Formatter, Result};
//! use format::lazy_format;
//! use std::format;
//!
//! struct Foo {
//!     bar: [u32; 10],
//! }
//!
//! impl Debug for Foo {
//!     fn fmt(&self, f: &mut Formatter) -> Result {
//!         let bar = lazy_format!(|f| f.debug_list().entries(&self.bar).finish());
//!         f.debug_struct("Foo")
//!             .field("bar", &format_args!("{}", bar))
//!             .finish()
//!     }
//! }
//!
//! assert_eq!(
//!     "Foo { bar: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] }",
//!     format!("{:?}", Foo { bar: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] })
//! );
//! ```
//!
//! Control flow:
//!
//! ```
//! use core::fmt::{Debug, Formatter, Result};
//! use format::lazy_format;
//! use std::format;
//!
//! struct Foo(usize);
//!
//! impl Debug for Foo {
//!     fn fmt(&self, f: &mut Formatter) -> Result {
//!         let alternate = f.alternate();
//!         let bar = lazy_format!(|f| if alternate {
//!             write!(f, "{:#x}", self.0)
//!         } else {
//!             write!(f, "{:x}", self.0)
//!         });
//!         f.debug_tuple("Foo")
//!             .field(&format_args!("{}", bar))
//!             .finish()
//!     }
//! }
//!
//! assert_eq!("Foo(75bcd15)", format!("{:?}", Foo(0123456789)));
//! assert_eq!("Foo(\n    0x75bcd15,\n)", format!("{:#?}", Foo(0123456789)));
//! ```

#![no_std]

pub use format_core::{
    Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex,
};
#[cfg(feature = "macro")]
pub use format_macro::lazy_format;

#[cfg(feature = "ext")]
pub mod ext;
