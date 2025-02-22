pub mod uart;
pub mod led;
pub mod mailbox;
// pub mod mail;
#[cfg(not(feature = "virt"))]
mod uart8250;
#[cfg(not(feature = "virt"))]
pub use uart8250::DW8250 as uart_dev;
#[cfg(feature = "virt")]
mod ns16550a;
#[cfg(feature = "virt")]
pub use ns16550a::Ns16550a as uart_dev;
mod pinmux;
use super::arch::plic;
use mailbox::{mail_box_init, mail_box_handler};
pub fn setup(){
    uart::init_uart();
    mail_box_init();
    plic::register_handler(61, mail_box_handler);
}