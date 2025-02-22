#![allow(dead_code)]
use crate::config::PLIC_BASE;
use crate::pr_err;
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::ReadWrite,
};
use core::ptr::NonNull;
use core::usize;

const NUM_IRQ: usize = 128;
//https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc
static mut PLIC: Plic = Plic::new(PLIC_BASE as _);




/// See §1.
const SOURCE_NUM: usize = 1024;
/// See §1.
const CONTEXT_NUM: usize = 15872;

const U32_BITS: usize = u32::BITS as usize;

register_structs! {
  #[allow(non_snake_case)]
  ContextLocal {
    /// Priority Threshold
    /// - The base address of Priority Thresholds register block is located at 4K alignment starts from offset 0x200000.
    (0x0000 => PriorityThreshold: ReadWrite<u32>),
    /// Interrupt Claim/complete Process
    /// - The Interrupt Claim Process register is context based and is located at (4K alignment + 4) starts from offset 0x200000.
    (0x0004 => InterruptClaimComplete: ReadWrite<u32>),
    (0x0008 => _reserved_0),
    (0x1000 => @END),
  }
}

register_structs! {
  #[allow(non_snake_case)]
  InterruptEnableCtxX {
    /// Priority Threshold
    /// - The base address of Priority Thresholds register block is located at 4K alignment starts from offset 0x200000.
    (0x00 => InterruptSources: [ReadWrite<u32>; SOURCE_NUM / U32_BITS]),
    (0x80 => @END),
  }
}

register_structs! {
  #[allow(non_snake_case)]
  PLICRegs {
    /// Interrupt Source Priority #0 to #1023
    (0x000000 => InterruptPriority: [ReadWrite<u32>; SOURCE_NUM]),
    /// Interrupt Pending Bit of Interrupt Source #0 to #N
    /// 0x001000: Interrupt Source #0 to #31 Pending Bits
    /// ...
    /// 0x00107C: Interrupt Source #992 to #1023 Pending Bits
    (0x001000 => InterruptPending: [ReadWrite<u32>; 0x20]),
    (0x001080 => _reserved_0),
    /// Interrupt Enable Bit of Interrupt Source #0 to #1023 for 15872 contexts
    (0x002000 => InterruptEnableCtxX: [InterruptEnableCtxX; CONTEXT_NUM]),
    (0x1F2000 => _reserved_1),
    /// 4096 * 15872 = 65011712(0x3e000 00) bytes
    /// Priority Threshold for 15872 contexts
    /// - The base address of Priority Thresholds register block is located at 4K alignment starts from offset 0x200000.
    /// Interrupt Claim Process for 15872 contexts
    /// - The Interrupt Claim Process register is context based and is located at (4K alignment + 4) starts from offset 0x200000.
    /// - The Interrupt Completion registers are context based and located at the same address with Interrupt Claim Process register, which is at (4K alignment + 4) starts from offset 0x200000.
    (0x200000 => Contexts: [ContextLocal; CONTEXT_NUM]),
    (0x4000000 => @END),
  }
}


/// Platform-Level Interrupt Controller.
pub struct Plic {
    base: NonNull<PLICRegs>,
}

unsafe impl Send for Plic {}
unsafe impl Sync for Plic {}

impl Plic {
    /// Create a new instance of the PLIC from the base address.
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
        }
    }

    /// Initialize the PLIC by context, setting the priority threshold to 0.
    pub fn init_by_context(&mut self, context: usize)
    {
        self.regs().Contexts[context]
            .PriorityThreshold
            .set(0);
    }

    const fn regs(&self) -> &PLICRegs {
        unsafe { self.base.as_ref() }
    }

    /// Sets priority for interrupt `source` to `value`.
    ///
    /// Write `0` to priority `value` effectively disables this interrupt `source`, for the priority
    /// value 0 is reserved for "never interrupt" by the PLIC specification.
    ///
    /// The lowest active priority is priority `1`. The maximum priority depends on PLIC implementation
    /// and can be detected with [`Plic::probe_priority_bits`].
    ///
    /// See §4.
    #[inline]
    pub fn set_priority(&self, source: u32, value: u32)
    {
        self.regs().InterruptPriority[source as usize].set(value);
    }

    /// Gets priority for interrupt `source`.
    ///
    /// See §4.
    #[inline]
    pub fn get_priority(&self, source: u32) -> u32
    {
        self.regs().InterruptPriority[source as usize].get()
    }

    /// Probe maximum level of priority for interrupt `source`.
    ///
    /// See §4.
    #[inline]
    pub fn probe_priority_bits(&self, source: u32) -> u32
    {
        let source = source as usize;
        self.regs().InterruptPriority[source].set(!0);
        self.regs().InterruptPriority[source].get()
    }

    /// Check if interrupt `source` is pending.
    ///
    /// See §5.
    #[inline]
    pub fn is_pending(&self, source: u32) -> bool
    {
        let (group, index) = parse_group_and_index(source as usize);
        self.regs().InterruptPending[group].get() & (1 << index) != 0
    }

    #[inline]
    pub fn clear_pending(&self, source: u32)
    {
        let (group, index) = parse_group_and_index(source as usize);
        let value = self.regs().InterruptPending[group].get();
        self.regs().InterruptPending[group].set(value | (!(1 << index)));
    }
    /// Enable interrupt `source` in `context`.
    ///
    /// See §6.
    #[inline]
    pub fn enable(&self, source: u32, context: usize)
    {
        let (group, index) = parse_group_and_index(source as usize);

        let value = self.regs().InterruptEnableCtxX[context].InterruptSources[group].get();
        self.regs().InterruptEnableCtxX[context].InterruptSources[group].set(value | 1 << index);
    }

    /// Disable interrupt `source` in `context`.
    ///
    /// See §6.
    #[inline]
    pub fn disable(&self, source: u32, context: usize)
    {
        let (group, index) = parse_group_and_index(source as usize);

        let value = self.regs().InterruptEnableCtxX[context].InterruptSources[group].get();
        self.regs().InterruptEnableCtxX[context].InterruptSources[group].set(value & !(1 << index));
    }

    /// Check if interrupt `source` is enabled in `context`.
    ///
    /// See §6.
    #[inline]
    pub fn is_enabled(&self, source: u32, context: usize) -> bool
    {
        let (group, index) = parse_group_and_index(source as usize);

        self.regs().InterruptEnableCtxX[context].InterruptSources[group].get() & (1 << index) != 0
    }

    /// Get interrupt threshold in `context`.
    ///
    /// See §7.
    #[inline]
    pub fn get_threshold(&self, context: usize) -> u32
    {
        self.regs().Contexts[context]
            .PriorityThreshold
            .get()
    }

    /// Set interrupt threshold for `context` to `value`.
    ///
    /// See §7.
    #[inline]
    pub fn set_threshold(&self, context: usize, value: u32)
    {
        self.regs().Contexts[context]
            .PriorityThreshold
            .set(value);
    }

    /// Probe maximum supported threshold value the `context` supports.
    ///
    /// See §7.
    #[inline]
    pub fn probe_threshold_bits(&self, context: usize) -> u32
    {
        self.regs().Contexts[context].PriorityThreshold.set(!0);
        self.regs().Contexts[context].PriorityThreshold.get()
    }

    /// Claim an interrupt in `context`, returning its source.
    ///
    /// It is always legal for a hart to perform a claim even if `EIP` is not set.
    /// A hart could set threshold to maximum to disable interrupt notification, but it does not mean
    /// interrupt source has stopped to send interrupt signals. In this case, hart would instead
    /// poll for active interrupt by periodically calling the `claim` function.
    ///
    /// See §8.
    #[inline]
    pub fn claim(&self, context: usize) -> u32
    {
        self.regs().Contexts[context]
        .InterruptClaimComplete
        .get()
    }

    /// Mark that interrupt identified by `source` is completed in `context`.
    ///
    /// See §9.
    #[inline]
    pub fn complete(&self, context: usize, source: u32)
    {
        self.regs().Contexts[context]
            .InterruptClaimComplete
            .set(source);
    }
}

fn parse_group_and_index(source: usize) -> (usize, usize) {
    let group = source / U32_BITS;
    let index = source % U32_BITS;
    (group, index)
}

pub fn plic_init() {
    unsafe {
        PLIC.init_by_context(0);
        PLIC.set_threshold(0, 0);
    }
}
static mut PLIC_HANDLERS: [Option<fn()>; NUM_IRQ] = [None; NUM_IRQ];
pub fn register_handler(irq: u32, handler: fn()) {
    unsafe {
        PLIC.clear_pending(irq);
        PLIC_HANDLERS[irq as usize] = Some(handler);
        PLIC.enable(irq, 0);
        PLIC.set_priority(irq as u32, 7)
    }
}

#[no_mangle]
pub fn platform_irq() {
    let irq = unsafe { PLIC.claim(0)};
    if 0 == irq{
        pr_err!("no irq found!");
        return;
    }
    let handlers = unsafe { PLIC_HANDLERS.get(irq as usize) }.unwrap();
    match handlers {
        Some(handler) => handler(),
        None => pr_err!("can not found irq {:#x} handler!", irq),
    }
    unsafe { PLIC.complete(0,irq)};
}

