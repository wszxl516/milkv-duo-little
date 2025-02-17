use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite};
use tock_registers::{register_structs};

register_structs! {
    Uart8250Reg {
        (0x0 => pub data_dll: ReadWrite<u8>),
        (0x1 => pub ier_dlh: ReadWrite<u8>),
        (0x2 => pub iir_fcr: ReadWrite<u8>),
        (0x3 => pub lcr: ReadWrite<u8>),
        (0x4 => pub mcr: ReadWrite<u8>),
        (0x5 => pub lsr: ReadOnly<u8>),
        (0x6 => pub msr: ReadOnly<u8>),
        (0x7 => pub sr: ReadWrite<u8>),
        (0x8 => @END),
    }
}
pub struct Uart8250(usize);

impl Uart8250 {
    pub const fn from_addr(addr: usize) -> Self {
        Self { 0: addr }
    }
    #[inline]
    fn reg(&self) -> &Uart8250Reg {
        unsafe { &*(self.0 as *mut Uart8250Reg) }
    }
    pub fn init(&self, uart_clock: u64, baudrate: u64) {
        let divisor = uart_clock / (16 * baudrate);
        let reg = self.reg();
        reg.lcr.set(0x80 | 0x03);
        reg.data_dll.set((divisor & 0xff) as u8);
        reg.ier_dlh.set(((divisor >> 8) & 0xff) as u8);
        reg.lcr.set(reg.lcr.get() | &(!0x80));
        reg.ier_dlh.set(0);
        reg.mcr.set(0x1 | 0x2);
        reg.iir_fcr.set(0x1 | 0x2 | 0x4);
        reg.lcr.set(0x3);
    }
    pub fn write_char(&self, c: u8) {
        while (self.reg().lsr.get() & 0x20).ne(&0) {}
        self.reg().data_dll.set(c);
    }
}