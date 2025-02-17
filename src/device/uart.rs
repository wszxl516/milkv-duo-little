use core::fmt::Write;
use lazy_static::lazy_static;
lazy_static!{
    static ref UART8250: super::uart8250::Uart8250 = {
        let u = super::uart8250::Uart8250::from_addr(super::super::config::UART_BASE);
        let baudrate = super::super::config::UART_BAUD_RATE;
	    let uart_clock = super::super::config::SYS_CLOCK_HZ;
        u.init(uart_clock, baudrate);
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
        UART8250.write_char(c as u8);
        Ok(())
    }
}

pub fn puts(args: core::fmt::Arguments) {
    let _ = Console.write_fmt(args);
}
