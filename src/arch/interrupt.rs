#![allow(dead_code)]

use crate::println;
use crate::config::SYS_CLOCK_HZ;
use crate::arch::timer::TIMER;
use crate::arch::trap::Context;
use super::plic::platform_irq;
#[repr(u32)]
#[derive(Debug)]
pub enum Interrupt {
    //1 0 RESERVED
    SupervisorSoftwareInterrupt = 1,
    //1 2 RESERVED
    MachineSoftwareInterrupt = 3,
    //1 4 RESERVED
    SupervisorTimerInterrupt = 5,
    //1 6 RESERVED
    MachineTimerInterrupt = 7,
    //1 8 RESERVED
    SupervisorExternalInterrupt = 9,
    //1 10 RESERVED
    MachineExternalInterrupt = 11,
    //1 12–15 RESERVED
    // 1 ≥16 DESIGNATED FOR PLATFORM USE
}

impl Interrupt {
    pub fn from_u32(value: u32) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

pub fn interrupt_handler(interrupt: Interrupt, _stack_addr: &mut Context) {
    match interrupt {
        Interrupt::MachineTimerInterrupt => {
            TIMER.setup_timer(SYS_CLOCK_HZ)
        },
        Interrupt::MachineExternalInterrupt => {
            platform_irq();
        },
        _ => println!("{:?}", interrupt)
    }
}
