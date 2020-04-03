#![cfg_attr(feature = "nightly", proc_macro_hygiene)]

use core::fmt::{Debug, Formatter, Result};
use format::lazy_format;

struct Foo(usize);

impl Debug for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let alternate = f.alternate();
        let bar = lazy_format!(|f| if alternate {
            write!(f, "{:#x}", self.0)
        } else {
            write!(f, "{:x}", self.0)
        });
        f.debug_tuple("Foo")
            .field(&format_args!("{}", bar))
            .finish()
    }
}

fn main() {
    assert_eq!("Foo(75bcd15)", format!("{:?}", Foo(0123456789)));
    assert_eq!("Foo(\n    0x75bcd15,\n)", format!("{:#?}", Foo(0123456789)));
}
