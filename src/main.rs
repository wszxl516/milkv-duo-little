#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(const_trait_impl)]
#![allow(static_mut_refs)]
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
    println!("duo256 little core started!");
    plic::register_handler(61, mail_box_handler);
    loop {
        match mail_box_fetch() {
            None => {}
            Some(cmd) => {
                if cmd.param_ptr == 2 {
                    duo_led_control(true);
                    println!("led on!\n");
                }
                else {
                    duo_led_control(false);
                    println!("led off!\n");
                }
            }
        }
        common::sleep::sleep_ns(1000);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pr_err!("panic: {:?}", info);
    loop {
        duo_led_control(true);
        common::sleep::sleep_ns(100);
        duo_led_control(false);
        common::sleep::sleep_ns(100);
    }
}

