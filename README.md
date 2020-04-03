# Format

[![Build Status](https://travis-ci.org/kgv/format.svg?branch=master)](https://travis-ci.org/kgv/format)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/kgv/format?svg=true)](https://ci.appveyor.com/project/kgv/format)
[![Crates](https://img.shields.io/crates/v/format.svg)](https://crates.io/crates/format)
[![Docs](https://docs.rs/format/badge.svg)](https://docs.rs/format)
[![License](https://img.shields.io/crates/l/format)](#license)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.32+-lightgray.svg)](https://github.com/kgv/format#rust-version-requirements)

A utility crate to make it easier to work with the formatter

## Usage

Add dependency to your `Cargo.toml`:

```toml
[dependencies]
format = "0.2"
```

and use `lazy_format` macro:

```rust
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
```

or one of format type:

```rust
struct Foo(usize);

impl Debug for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let alternate = f.alternate();
        let bar = LowerHex(|f| {
            if alternate {
                write!(f, "{:#x}", self.0)
            } else {
                write!(f, "{:x}", self.0)
            }
        });
        f.debug_tuple("Foo")
            .field(&format_args!("{:x}", bar))
            .finish()
    }
}
```
