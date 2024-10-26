use lazy_static::lazy_static;
use tock_registers::interfaces::Writeable;
use crate::{pr_notice, reg_clear_bit_p, reg_read_p};
use tock_registers::registers::ReadWrite;
use crate::config::{CLINT_BASE, SYS_CLOCK_HZ};

#[allow(non_snake_case)]
#[allow(dead_code)]
#[repr(C)]
pub struct ClintReg {
    MSIP0: ReadWrite<u32>,
    MSIP1: ReadWrite<u32>,
    MSIP2: ReadWrite<u32>,
    MSIP3: ReadWrite<u32>,
    RESERVED0: [u32; (0x4004000 - 0x400000C) / 4 - 1],
    MTIMECMPL0: ReadWrite<u32>,
    MTIMECMPH0: ReadWrite<u32>,
    MTIMECMPL1: ReadWrite<u32>,
    MTIMECMPH1: ReadWrite<u32>,
    MTIMECMPL2: ReadWrite<u32>,
    MTIMECMPH2: ReadWrite<u32>,
    MTIMECMPL3: ReadWrite<u32>,
    MTIMECMPH3: ReadWrite<u32>,
    RESERVED1: [u32; (0x400C000 - 0x400401C) / 4 - 1],
    SSIP0: ReadWrite<u32>,
    SSIP1: ReadWrite<u32>,
    SSIP2: ReadWrite<u32>,
    SSIP3: ReadWrite<u32>,
    RESERVED2: [u32; (0x400D000 - 0x400C00C) / 4 - 1],
    STIMECMPL0: ReadWrite<u32>,
    STIMECMPH0: ReadWrite<u32>,
    STIMECMPL1: ReadWrite<u32>,
    STIMECMPH1: ReadWrite<u32>,
    STIMECMPL2: ReadWrite<u32>,
    STIMECMPH2: ReadWrite<u32>,
    STIMECMPL3: ReadWrite<u32>,
    STIMECMPH3: ReadWrite<u32>,
}

pub struct Timer(usize);
impl Timer {
    pub const fn new(base: usize) -> Timer {
        Self(base)
    }
    #[inline]
    fn reg(&self) -> &ClintReg{
        unsafe { &*(self.0 as *mut ClintReg) }
    }
    pub fn setup_timer(&self, ticks: u64) {
        pr_notice!("T\n");
        let current = reg_read_p!(time) as u64;
        let next = current + ticks;
        self.reg().MTIMECMPL0.set(next as u32);
        self.reg().MTIMECMPH0.set((next >> 32) as u32);
        reg_clear_bit_p!(mip, 1<<7)
    }

    pub fn disable_timer(&self) {
        self.setup_timer(u64::MAX)
    }

    pub fn enable_timer(&self) {
        self.setup_timer(SYS_CLOCK_HZ)

    }
}

lazy_static!{
    pub static ref TIMER: Timer = Timer::new(CLINT_BASE);
}