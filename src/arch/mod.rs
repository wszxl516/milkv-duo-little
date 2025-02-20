use crate::{reg_update_p, reg_write_p};
use crate::arch::mm::init_heap;
use crate::arch::timer::{TIMER};
pub mod timer;
pub mod trap;
mod exception;
mod interrupt;
pub mod plic;
mod mm;

pub fn setup(){
    // set arch handler
    reg_write_p!(mscratch, 0);
    reg_write_p!(mie, 0);
    reg_write_p!(mtvec, trap::trap_handler as usize);
    // enable float
    reg_update_p!(mstatus, 0x1 << 13);
    // enable interrupt
    reg_update_p!(mie, 0x880);
    reg_write_p!(mscratch, trap::M_TRAP_FRAMES.addr() as usize);
    //enable machine mode timer
    reg_write_p!(mstatus, 1 << 3);
    // enable all interrupt and exception
    reg_update_p!(mie,1 << 11 | 1 << 9 | 1<< 7 | 1 << 5 | 1 << 1);
    TIMER.enable_timer();
    plic::plic_init();
    init_heap()
}
