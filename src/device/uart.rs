use core::fmt::Write;
use crate::device::pinmux::fix_uart1;

static mut UART: Option<super::uart_dev> = None;

pub fn init_uart(){
    fix_uart1();
    let u = super::uart_dev::from_addr(super::super::config::UART_BASE);
    u.init(115200);
    unsafe {
        UART.replace(u)
    };
}

#[inline]
fn putc(c: char){
    unsafe {
        match &UART {
            Some(u) => u.putchar(c as u8),
            None => {}
        }
    }
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
        putc(c);
        Ok(())
    }
}


pub fn puts(args: core::fmt::Arguments) {
    let _ = Console.write_fmt(args);
}
