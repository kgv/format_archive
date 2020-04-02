use format::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};

#[test]
fn binary() {
    let lazy_format = Binary(|f| write!(f, "{:#b}", 0123456789));
    assert_eq!(
        "0b111010110111100110100010101",
        format!("{:b}", lazy_format)
    );
    assert_eq!("0b111010110111100110100010101", format!("{}", lazy_format));
    assert_eq!(
        "Binary(0b111010110111100110100010101)",
        format!("{:?}", lazy_format)
    );
}

#[test]
fn debug() {
    let lazy_format = Debug(|f| write!(f, "{}", 0123456789));
    assert_eq!("123456789", format!("{:?}", lazy_format));
}

#[test]
fn display() {
    let lazy_format = Display(|f| write!(f, "{}", 0123456789));
    assert_eq!("123456789", format!("{}", lazy_format));
    assert_eq!("Display(123456789)", format!("{:?}", lazy_format));
}

#[test]
fn lower_exp() {
    let lazy_format = LowerExp(|f| write!(f, "{:#e}", 0123456789f64));
    assert_eq!("1.23456789e8", format!("{:e}", lazy_format));
    assert_eq!("1.23456789e8", format!("{}", lazy_format));
    assert_eq!("LowerExp(1.23456789e8)", format!("{:?}", lazy_format));
}

#[test]
fn lower_hex() {
    let lazy_format = LowerHex(|f| write!(f, "{:#x}", 0123456789));
    assert_eq!("0x75bcd15", format!("{:x}", lazy_format));
    assert_eq!("0x75bcd15", format!("{}", lazy_format));
    assert_eq!("LowerHex(0x75bcd15)", format!("{:?}", lazy_format));
}

#[test]
fn octal() {
    let lazy_format = Octal(|f| write!(f, "{:#o}", 0123456789));
    assert_eq!("0o726746425", format!("{:o}", lazy_format));
    assert_eq!("0o726746425", format!("{}", lazy_format));
    assert_eq!("Octal(0o726746425)", format!("{:?}", lazy_format));
}

#[test]
fn pointer() {
    let lazy_format = Pointer(|f| write!(f, "{:#p}", 0123456789 as *const ()));
    assert_eq!("0x00000000075bcd15", format!("{:p}", lazy_format));
    assert_eq!("0x00000000075bcd15", format!("{}", lazy_format));
    assert_eq!("Pointer(0x00000000075bcd15)", format!("{:?}", lazy_format));
}

#[test]
fn upper_exp() {
    let lazy_format = UpperExp(|f| write!(f, "{:#E}", 0123456789f64));
    println!("{:E}", lazy_format);
    assert_eq!("1.23456789E8", format!("{:E}", lazy_format));
    assert_eq!("1.23456789E8", format!("{}", lazy_format));
    assert_eq!("UpperExp(1.23456789E8)", format!("{:?}", lazy_format));
}

#[test]
fn upper_hex() {
    let lazy_format = UpperHex(|f| write!(f, "{:#X}", 0123456789));
    assert_eq!("0x75BCD15", format!("{:X}", lazy_format));
    assert_eq!("0x75BCD15", format!("{}", lazy_format));
    assert_eq!("UpperHex(0x75BCD15)", format!("{:?}", lazy_format));
}
