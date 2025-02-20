use core::fmt::Write;
use lazy_static::lazy_static;
use crate::device::pinmux::fix_uart1;

lazy_static!{
    static ref UART8250: super::uart8250::DW8250 = {
        fix_uart1();
        let mut u = super::uart8250::DW8250::new(super::super::config::UART_BASE);
        u.init(115200);
        u.set_ier(false);
        u
    };
}

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            let _ = self.write_char(*c as char);
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        UART8250.putchar(c as u8);
        Ok(())
    }
}


pub fn puts(args: core::fmt::Arguments) {
    let _ = Console.write_fmt(args);
}
