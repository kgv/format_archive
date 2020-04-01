#![feature(proc_macro_hygiene)]

use core::cell::Cell;

struct Generator {
    index: Cell<usize>,
}

impl Generator {
    fn new() -> Self {
        Self {
            index: Cell::new(0),
        }
    }

    fn next(&self) -> usize {
        self.index.set(self.index.get() + 1);
        self.index.get()
    }

    fn count(&self) -> usize {
        self.index.get()
    }
}

mod closure {
    use format::lazy_format;

    #[test]
    fn r#if() {
        let alternate = true;
        assert_eq!(
            "a",
            lazy_format!(|f| if alternate {
                write!(f, "{}", "a")
            } else {
                write!(f, "{}", "b")
            })
            .to_string()
        );
    }

    #[test]
    fn r#match() {
        let alternate = false;
        assert_eq!(
            "b",
            lazy_format!(|f| match alternate {
                true => write!(f, "{}", "a"),
                false => write!(f, "{}", "b"),
            })
            .to_string()
        );
    }

    #[test]
    fn iter() {
        let vec = vec![0, 1, 2, 3];
        assert_eq!(
            "#0#1#2#3",
            lazy_format!(|f| vec.iter().try_for_each(|item| write!(f, "#{}", item))).to_string()
        );
    }

    #[test]
    fn r#move() {
        let a = format!("a");
        lazy_format!(|f| write!(f, "{}", a));
        assert_eq!("a", a);

        let a = format!("one");
        lazy_format!(move |f| write!(f, "{}", a));
        // assert_eq!("a", a); // compile time error
    }
}

mod format {
    use crate::Generator;
    use format::lazy_format;
    use std::fmt::Display;

    #[test]
    fn without_args() {
        assert_eq!("none", lazy_format!("none").to_string());
    }

    #[test]
    fn with_unnamed_args() {
        assert_eq!("a, b", lazy_format!("{}, {}", "a", "b").to_string());
    }

    #[test]
    fn with_named_args() {
        assert_eq!(
            "a, b",
            lazy_format!("{a}, {b}", a = "a", b = "b").to_string()
        );
        assert_eq!(
            "a, b",
            lazy_format!("{a}, {b}", b = "b", a = "a").to_string()
        );
    }

    #[test]
    fn with_shadowed_named_args() {
        let a = 1;
        let b = 2;
        assert_eq!(
            "3, 2",
            lazy_format!("{a}, {b}", a = a + b, b = a * b).to_string()
        );
    }

    #[test]
    fn with_mixed_args() {
        assert_eq!("a, b", lazy_format!("{}, {b}", "a", b = "b").to_string());
    }

    #[test]
    fn with_trailing_comma() {
        let _ = lazy_format!("none",);
        let _ = lazy_format!("{}, {}", "a", "b",);
        let _ = lazy_format!("{a}, {b}", a = "a", b = "b",);
        let _ = lazy_format!("{}, {b}", "a", b = "b",);
    }

    #[test]
    fn without_trailing_comma() {
        let _ = lazy_format!("none");
        let _ = lazy_format!("{}, {}", "a", "b");
        let _ = lazy_format!("{a}, {b}", a = "one", b = "b");
        let _ = lazy_format!("{}, {b}", "a", b = "b");
    }

    #[test]
    fn lazy() {
        let generator = &Generator::new();
        let a = lazy_format!("{}, {}, {}", "a", generator.next(), generator.next());
        let b = lazy_format!("{}, {}", generator.next(), generator.next());
        assert_eq!(0, generator.count());
        assert_eq!("a, 1, 2; 3, 4", format!("{}; {}", a, b));
        assert_eq!(4, generator.count());
        assert_eq!("5, 6; a, 7, 8", format!("{}; {}", b, a));
        assert_eq!(8, generator.count());
    }

    #[test]
    fn lifetime() {
        fn temp<'a>(a: &'a str, b: &'a str) -> impl Display + 'a {
            lazy_format!("{}, {}", a, b = b)
        }

        let a = "a".to_string();
        let b = "b".to_string();
        assert_eq!("a, b", temp(a.as_str(), b.as_str()).to_string());
    }

    #[test]
    fn recursion() {
        let generator = &Generator::new();
        let a = lazy_format!("{}, {}", generator.next(), generator.next());
        let b = lazy_format!("({c}), ({c})", c = a);
        assert_eq!(0, generator.count());
        assert_eq!("(1, 2), (3, 4)", b.to_string(),);
        assert_eq!(4, generator.count());
        assert_eq!("(5, 6), (7, 8)", b.to_string(),);
        assert_eq!(8, generator.count());
    }
}

mod kind {
    use format::lazy_format;
    use format_core::{
        Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex,
    };

    #[test]
    fn debug() {
        let _: Debug<_> = lazy_format!("{:?}", 0);
        let _: Debug<_> = lazy_format!("{:?}{:?}", 0, 1);
    }

    #[test]
    fn display() {
        let _: Display<_> = lazy_format!("");
        let _: Display<_> = lazy_format!("{}", 0);
        let _: Display<_> = lazy_format!("{}, {:?}", 0, 1);
        let _: Display<_> = lazy_format!("{:?}, {}", 0, 1);
        let _: Display<_> = lazy_format!("{:b}, {:e}", 0, 1);
    }

    #[test]
    fn binary() {
        let _: Binary<_> = lazy_format!("{:?}, {:b}", 0, 1);
        let _: Binary<_> = lazy_format!("{:b}, {:?}", 0, 1);
        let _: Binary<_> = lazy_format!("{}, {:b}", 0, 1);
        let _: Binary<_> = lazy_format!("{:b}, {}", 0, 1);
    }

    #[test]
    fn lower_exp() {
        let _: LowerExp<_> = lazy_format!("{:?}, {:e}", 0, 1);
        let _: LowerExp<_> = lazy_format!("{:e}, {:?}", 0, 1);
        let _: LowerExp<_> = lazy_format!("{}, {:e}", 0, 1);
        let _: LowerExp<_> = lazy_format!("{:e}, {}", 0, 1);
    }

    #[test]
    fn lower_hex() {
        let _: LowerHex<_> = lazy_format!("{:?}, {:x}", 0, 1);
        let _: LowerHex<_> = lazy_format!("{:x}, {:?}", 0, 1);
        let _: LowerHex<_> = lazy_format!("{}, {:x}", 0, 1);
        let _: LowerHex<_> = lazy_format!("{:x}, {}", 0, 1);
    }

    #[test]
    fn octal() {
        let _: Octal<_> = lazy_format!("{:?}, {:o}", 0, 1);
        let _: Octal<_> = lazy_format!("{:o}, {:?}", 0, 1);
        let _: Octal<_> = lazy_format!("{}, {:o}", 0, 1);
        let _: Octal<_> = lazy_format!("{:o}, {}", 0, 1);
    }

    #[test]
    fn pointer() {
        let _: Pointer<_> = lazy_format!("{:?}, {:p}", 0, 1 as *const ());
        let _: Pointer<_> = lazy_format!("{:p}, {:?}", 0 as *const (), 1);
        let _: Pointer<_> = lazy_format!("{}, {:p}", 0, 1 as *const ());
        let _: Pointer<_> = lazy_format!("{:p}, {}", 0 as *const (), 1);
    }

    #[test]
    fn upper_exp() {
        let _: UpperExp<_> = lazy_format!("{:?}, {:E}", 0, 1);
        let _: UpperExp<_> = lazy_format!("{:E}, {:?}", 0, 1);
        let _: UpperExp<_> = lazy_format!("{}, {:E}", 0, 1);
        let _: UpperExp<_> = lazy_format!("{:E}, {}", 0, 1);
    }

    #[test]
    fn upper_hex() {
        let _: UpperHex<_> = lazy_format!("{:?}, {:X}", 0, 1);
        let _: UpperHex<_> = lazy_format!("{:X}, {:?}", 0, 1);
        let _: UpperHex<_> = lazy_format!("{}, {:X}", 0, 1);
        let _: UpperHex<_> = lazy_format!("{:X}, {}", 0, 1);
    }
}
