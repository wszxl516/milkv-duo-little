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
pub fn setup(){
    uart::init_uart();
    #[cfg(not(feature = "virt"))]
    mailbox::mail_box_init();
    #[cfg(not(feature = "virt"))]
    super::arch::plic::register_handler(61, mailbox::mail_box_handler);
}