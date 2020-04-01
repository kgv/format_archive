# Format

A utility crate to make it easier to work with the formatter

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
format = "0.1"
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
