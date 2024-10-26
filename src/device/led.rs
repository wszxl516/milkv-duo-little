use crate::common::mmio::MMIO;

pub const GPIO_LED: usize = 0x05021000;
pub const GPIO_LED_NR: u32 = 2;

pub const GPIO_SWPORTA_DR: usize = 0x000;
pub const GPIO_SWPORTA_DDR: usize = 0x004;
pub fn duo_led_control(enable: bool) {
    MMIO::new(GPIO_LED | GPIO_SWPORTA_DDR).write(1u32 << GPIO_LED_NR);
    if enable {
        MMIO::new(GPIO_LED | GPIO_SWPORTA_DR).write(1u32 << GPIO_LED_NR);
    } else {
        MMIO::new(GPIO_LED | GPIO_SWPORTA_DR).write(0u32 << GPIO_LED_NR);
    }
}
