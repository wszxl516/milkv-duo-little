#![allow(dead_code)]
use super::trap::Regs;
use crate::{pr_err, println, reg_read_a};

#[repr(u32)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    BREAKPOINT = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAmoAddressMisaligned = 6,
    StoreAmoAccessFault = 7,
    EnvironmentCallFromUMode = 8,
    EnvironmentCallFromSMode = 9,
    //10 RESERVED
    EnvironmentCallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    //14 RESERVED
    StoreAmoPageFault = 15,
    //16–23 RESERVED
    //24–31 DESIGNATED FOR CUSTOM USE
    //32–47 RESERVED
    //48–63 DESIGNATED FOR CUSTOM USE
    //≥64 RESERVED
}

impl Exception {
    pub fn from_u32(value: u32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
fn dump_stack(regs: &Regs) {
    pr_err!("\n");
    pr_err!(
        "call stack: \n\t#1: {:#x} \n\t#0: {:#x} \n",
        regs.epc,
        regs.ra
    );
    pr_err!("tval: {:#x}\n", regs.tval);
    if regs.epc >= 4 {
        pr_err!("code: ");

        for addr in regs.epc..regs.epc + 4 {
            let code = reg_read_a!(addr, u8);
            pr_err!("{:02x} ", code);
        }
    }

    pr_err!("\n");
    pr_err!("\n");
    pr_err!("{}", regs.context);
    pr_err!("\n");
    pr_err!("\n");
}
pub fn exception_handler(exception: Exception, regs: &Regs) {
    match exception {
        Exception::InstructionAddressMisaligned => println!("Instruction address misaligned."),
        Exception::InstructionAccessFault => println!("Instruction access fault."),
        Exception::IllegalInstruction => println!("Illegal instruction."),
        Exception::BREAKPOINT => println!("Breakpoint."),
        Exception::LoadAddressMisaligned => println!("Load address misaligned."),
        Exception::LoadAccessFault => println!("Load access fault."),
        Exception::StoreAmoAddressMisaligned => println!(" Store/AMO address misaligned."),
        Exception::StoreAmoAccessFault => println!("Store/AMO access fault."),
        Exception::EnvironmentCallFromUMode => println!("Environment call from U-mode."),
        Exception::EnvironmentCallFromSMode => println!("Environment call from S-mode."),
        Exception::EnvironmentCallFromMMode => println!("Environment call from M-mode."),
        Exception::InstructionPageFault => println!("Instruction page fault."),
        Exception::LoadPageFault => println!("Load page fault."),
        Exception::StoreAmoPageFault => println!("Store/AMO page fault."),
    }
    dump_stack(regs);
    loop {}
}
