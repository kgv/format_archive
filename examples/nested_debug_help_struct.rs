#![cfg_attr(feature = "nightly", proc_macro_hygiene)]

use core::fmt::{Debug, Formatter, Result};
use format::lazy_format;

struct Foo {
    bar: [u32; 10],
}

impl Debug for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let bar = lazy_format!(|f| f.debug_list().entries(&self.bar).finish());
        f.debug_struct("Foo")
            .field("bar", &format_args!("{}", bar))
            .finish()
    }
}

fn main() {
    assert_eq!(
        "Foo { bar: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] }",
        format!(
            "{:?}",
            Foo {
                bar: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
            }
        )
    );
}
