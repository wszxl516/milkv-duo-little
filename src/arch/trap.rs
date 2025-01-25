#![allow(dead_code)]

use crate::{get_bits, reg_read_p};
use core::arch::{global_asm, asm};
use core::fmt::{Display, Formatter};
use crate::arch::exception::{Exception, exception_handler};
use crate::arch::interrupt::{Interrupt, interrupt_handler};
global_asm!(include_str!("macros.S"));

pub static mut M_TRAP_FRAMES: Context = Context::empty_new();
pub type Reg = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub ra: Reg,
    pub sp: Reg,
    gp: Reg,
    tp: Reg,
    t0: Reg,
    t1: Reg,
    t2: Reg,
    s0: Reg,
    s1: Reg,
    a0: Reg,
    a1: Reg,
    a2: Reg,
    a3: Reg,
    a4: Reg,
    a5: Reg,
    a6: Reg,
    a7: Reg,
    s2: Reg,
    s3: Reg,
    s4: Reg,
    s5: Reg,
    s6: Reg,
    s7: Reg,
    s8: Reg,
    s9: Reg,
    s10: Reg,
    s11: Reg,
    t3: Reg,
    t4: Reg,
    t5: Reg,
    t6: Reg,
    pub pc: Reg,
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let arr = self.array();
        for x in (0..30).step_by(3) {
            f.write_fmt(format_args!(
                "x{:0>2} = {:#018x} x{:0>2} = {:#018x} x{:0>2} = {:#018x}\n",
                x + 1,
                arr[x],
                x + 2,
                arr[x + 1],
                x + 3,
                arr[x + 2]
            ))
                .unwrap();
        }
        f.write_fmt(format_args!("pc  = {:#018x}", arr[31]))
    }
}

impl Context {
    pub const fn empty_new() -> Self {
        Self {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            pc: 0,
        }
    }
    pub fn addr(&self) -> usize {
        self as *const Self as usize
    }
    pub fn replace(&mut self, data: &Self) {
        *self = *data;
    }
    pub fn array(&self) -> [usize; 32] {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[derive(Debug)]
pub struct Regs {
    pub context: Context,
    pub ra: usize,
    pub epc: usize,
    pub tval: usize,
}
#[derive(Debug)]
pub struct Trap {
    regs: Regs,
    cause: usize,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ExceptionType {
    Exception = 0,
    Interrupt = 1,
}

impl ExceptionType {
    pub fn from_u32(value: u32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl Trap {
    pub fn new(sp: &mut Context, ra: usize) -> Self {
        Self {
            regs: Regs {
                context: *sp,
                ra,
                epc: reg_read_p!(mepc),
                tval: reg_read_p!(mtval),
            },
            cause: reg_read_p!(mcause),
        }
    }
    #[inline(always)]
    pub fn exception_type(&self) -> ExceptionType {
        ExceptionType::from_u32(get_bits!(self.cause, 63, 1) as u32)
    }
    #[inline(always)]
    pub fn exception_code(&self) -> u32 {
        get_bits!(self.cause, 0, 63) as u32
    }
}

#[allow(named_asm_labels)]
#[no_mangle]
#[link_section = ".vectors"]
pub unsafe extern "C" fn trap_handler() -> ! {
    asm!(
    r#"
    .align 8
    csrrw t6, mscratch, t6
    SAVE_G_REG t6
    csrr  t5, mscratch
    sd t5, (30)*REG_SIZE(t6)
    csrr t5, mepc
    sd t5, (31)*REG_SIZE(t6)
    mv a0, t6
    csrrw t6, mscratch, t6
    mv a1, ra
    jal {handle_trap}
    csrr t6, mscratch
    ld t5, (31)*REG_SIZE(t6)
    csrw mepc, t5
    REST_G_REG t6
    mret
	"#,
    handle_trap = sym handle_trap,
    options(noreturn),
    )
}
pub fn handle_trap(stack: &mut Context, ra: usize){
    let trap = Trap::new(stack, ra);
    match trap.exception_type() {
        ExceptionType::Interrupt => {
            interrupt_handler(Interrupt::from_u32(trap.exception_code()), stack);
        }

        ExceptionType::Exception => {
            exception_handler(Exception::from_u32(trap.exception_code()), &trap.regs);
        }
    }
}