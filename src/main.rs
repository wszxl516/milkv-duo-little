#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(const_trait_impl)]
extern crate alloc;

use core::panic::PanicInfo;
use crate::device::led::duo_led_control;
use crate::device::mailbox::{mail_box_fetch, mail_box_handler};
use crate::arch::plic;

mod entry;
pub mod config;
pub mod device;
mod common;
mod res_table;
pub mod arch;

#[no_mangle]
fn kernel_main() {
    plic::register_handler(61, mail_box_handler);
    loop {
        match mail_box_fetch() {
            None => {}
            Some(cmd) => {
                print!("set led ");
                if cmd.param_ptr == 2 {
                    println!("on");
                    duo_led_control(true);
                }
                else {
                    println!("off");
                    duo_led_control(false);
                }
            }
        }
        common::sleep::sleep(1);
    }
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pr_err!("panic: {:?}", info);
    loop {}
}

