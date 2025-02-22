#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(const_trait_impl)]
#![allow(static_mut_refs)]
extern crate alloc;
use crate::device::led::duo_led_control;
use crate::device::mailbox::{mail_box_register, Mailboxmsg, Opration};
use core::panic::PanicInfo;

pub mod arch;
mod common;
pub mod config;
pub mod device;
mod entry;
mod res_table;

#[no_mangle]
fn kernel_main() {
    println!("duo256 little core started!");

    mail_box_register(0, led_task);
    loop {
        common::sleep::sleep_ms(1000);
        println!("test!\n");
    }
}

pub fn led_task(msg: &Mailboxmsg) {
    let cmd = unsafe { (msg.data as *const Opration).read_volatile() };
    if cmd.cmd_id == 0x93 {
        if cmd.param == 2 {
            duo_led_control(true);
        } else {
            duo_led_control(false);
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pr_err!("panic: {:?}", info);
    loop {
        duo_led_control(true);
        common::sleep::sleep_ms(100);
        duo_led_control(false);
        common::sleep::sleep_ms(100);
    }
}
