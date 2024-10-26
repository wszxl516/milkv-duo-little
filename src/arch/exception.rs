#![allow(dead_code)]
use super::trap::Regs;
use crate::{println};

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
    println!("epc: {}", regs.epc);
    loop {}
}
