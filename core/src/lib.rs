#![no_std]

use core::fmt::{self, Formatter, Result};

// Generate lazy format macro
macro_rules! gen_lazy_format {
    (Debug) => {
        gen_lazy_format! { @base
            #[doc = "A lazy format type that implements [Debug](core::fmt::Debug) as base format trait"]
            Debug
        }
    };
    (Display) => {
        gen_lazy_format! { @base
            #[doc = "A lazy format type that implements [Display](core::fmt::Display) as base format trait, [Debug](core::fmt::Debug) as derivative from [Display](core::fmt::Display)"]
            Display
        }
        gen_lazy_format! { @debug Display }
    };
    ($self:ident) => {
        gen_lazy_format! { @base
            #[doc = concat!(
                "A lazy format type that implements [",
                stringify!($self),
                "](core::fmt::",
                stringify!($self),
                ") as base format trait, [Display](core::fmt::Display) and [Debug](core::fmt::Debug) as derivative from [",
                stringify!($self),
                "](core::fmt::",
                stringify!($self),
                ")"
            )]
            $self
        }
        gen_lazy_format! { @display $self }
        gen_lazy_format! { @debug $self }
    };
    (@base $(#[doc = $doc:expr])* $self:ident) => {
        $(#[doc = $doc])*
        #[derive(Clone, Copy)]
        pub struct $self<F: Fn(&mut Formatter) -> Result>(pub F);

        impl<F: Fn(&mut Formatter) -> Result> fmt::$self for $self<F> {
            fn fmt(&self, f: &mut Formatter) -> Result {
                (self.0)(f)
            }
        }
    };
    (@display $self:ident) => {
        impl<F: Fn(&mut Formatter) -> Result> fmt::Display for $self<F> {
            fn fmt(&self, f: &mut Formatter) -> Result {
                fmt::$self::fmt(self, f)
            }
        }
    };
    (@debug $self:ident) => {
        impl<F: Fn(&mut Formatter) -> Result> fmt::Debug for $self<F> {
            fn fmt(&self, f: &mut Formatter) -> Result {
                f.debug_tuple(stringify!($self))
                    .field(&Debug(&self.0))
                    .finish()
            }
        }
    };
}

gen_lazy_format! { Debug }
gen_lazy_format! { Display }
gen_lazy_format! { Binary }
gen_lazy_format! { LowerExp }
gen_lazy_format! { LowerHex }
gen_lazy_format! { Octal }
gen_lazy_format! { Pointer }
gen_lazy_format! { UpperExp }
gen_lazy_format! { UpperHex }
