#[derive(Eq, PartialEq, Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Red = 91,
    Green = 92,
    Orange = 93,
    Blue = 94,
    Magenta = 95,
    Cyan = 96,
    White = 97,
}

impl Color {
    pub const fn value(self) -> u8 {
        match self {
            Color::Red => 91,
            Color::Green => 92,
            Color::Orange => 93,
            Color::Blue => 94,
            Color::Magenta => 95,
            Color::Cyan => 96,
            Color::White => 97,
        }
    }
}

#[macro_export]
macro_rules! print {
    () => {};
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::device::uart::puts(format_args!($fmt $(, $($arg)+)?))
    };
}

#[macro_export]
macro_rules! println {
    () => { print!("\n") };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::device::uart::puts(format_args!(concat!($fmt, "\n\r") $(, $($arg)+)?))
    };
}

#[macro_export]
macro_rules! pr_color {
    ($fmt: literal ,$color: expr $(, $($arg: tt)+)?) =>{
        $crate::device::uart::puts(
            format_args!(concat!("\x1b[{}m", $fmt, "\x1b[0m"),
            $color
            $(, $($arg)+)?)
        )
    };
}

#[macro_export]
macro_rules! pr_info {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::common::print::Color::Green.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_notice {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::common::print::Color::Blue.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_warn {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
        $crate::pr_color!($fmt,
            $crate::common::print::Color::Orange.value()
            $(, $($arg)+)?
            )
    };
}

#[macro_export]
macro_rules! pr_err {
    ($fmt: literal $(, $($arg: tt)+)?) =>{
       $crate::pr_color!($fmt,
            $crate::common::print::Color::Red.value()
            $(, $($arg)+)?
            )
    };
}
