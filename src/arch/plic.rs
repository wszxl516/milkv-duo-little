#![allow(dead_code)]
use crate::config::PLIC_BASE;
use crate::{pr_err, reg_clear_bit_p, reg_read_a, reg_write_a};

const NUM_IRQ: usize = 128;
//https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc
static mut PLIC: Plic = Plic::new(PLIC_BASE);


pub struct Plic {
    base_addr: usize,
    plic_handlers: [Option<fn()>; NUM_IRQ],
}

impl Plic {
    const PRIORITY_OFFSET: usize = 0;
    const PENDING_OFFSET: usize= 0x1000;
    const ENABLE_OFFSET: usize = 0x2000;
    const THRESHOLD_OFFSET: usize = 0x200000;
    const CLAIM_OFFSET: usize = 0x200004;
    pub const fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            plic_handlers: [None; NUM_IRQ],
        }
    }

    pub fn init(&self){
        for i in 0..NUM_IRQ {
            self.set_priority(i as u32, 0);
        }

        for i in 0..=(NUM_IRQ / 32) {
            reg_write_a!(self.base_addr + Self::PENDING_OFFSET + i * 4, 0, u32);
            reg_write_a!(self.base_addr + Self::ENABLE_OFFSET + i * 4, 0, u32);
        }
        self.set_threshold(0)
    }

    pub fn enable(&self, irq_num: usize) {
        assert!(irq_num <= NUM_IRQ);
        self.set_priority(irq_num as u32, 1);
        let ctrl_addr = self.base_addr
            + Self::ENABLE_OFFSET
            + (4 * (irq_num / 32));
        let old = reg_read_a!(ctrl_addr, u32);
        reg_write_a!(ctrl_addr, old | (1 << (irq_num % 32)), u32);
    }
    pub fn disable(&self, irq_num: usize) {
        assert!(irq_num <= NUM_IRQ);
        self.set_priority(irq_num as u32, 1);
        let ctrl_addr = self.base_addr
            + Self::ENABLE_OFFSET
            + (4 * ((irq_num) / 32));
        let old = reg_read_a!(ctrl_addr, u32);
        reg_write_a!(ctrl_addr, old & !(1 << (irq_num % 32)), u32);
        self.set_threshold(0)
    }
    fn set_threshold(&self, threshold: u32) {
        reg_write_a!(
            self.base_addr
                + Self::THRESHOLD_OFFSET,
            threshold,
            u32
        );
    }
    fn set_priority(&self, irq_num: u32, value: u32) {
        reg_write_a!(self.base_addr + Self::PRIORITY_OFFSET + (4 * irq_num as usize), value, u32);
    }

    pub fn fetch_irq(&self) -> u32 {
        reg_read_a!(
            self.base_addr + Self::CLAIM_OFFSET,
            u32
        )
    }
    pub fn complete_irq(&self, irq_num: u32) {
        reg_write_a!(
            self.base_addr + Self::CLAIM_OFFSET,
            irq_num,
            u32
        )
    }
}

pub fn plic_init() {
    unsafe {
        PLIC.init();
    }
}

pub fn register_handler(irq: usize, handler: fn()) {
    unsafe {
        PLIC.plic_handlers[irq] = Some(handler);
        PLIC.enable(irq);
        PLIC.set_priority(irq as u32, 7)
    }
}

#[no_mangle]
pub fn platform_irq() {
    let irq = unsafe { PLIC.fetch_irq()};
    if irq == 0 {
        pr_err!("irq {:#x} IntNoReschedule !", irq);
        return;
    }
    let handlers = unsafe { PLIC.plic_handlers.get(irq as usize) }.unwrap();
    match handlers {
        Some(handler) => handler(),
        None => pr_err!("can not found irq {:#x} handler!", irq),
    }
    unsafe { PLIC.complete_irq(irq)};
    reg_clear_bit_p!(mip, 1 << 11);
}

