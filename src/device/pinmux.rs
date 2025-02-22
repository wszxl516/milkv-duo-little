const FMUX_GPIO_0: usize = 0x4c;
const FMUX_GPIO_1: usize = 0x50;

const PINMUX_BASE: usize = 0x03001000;
use super::super::common::mmio::MMIO;
pub fn pinmux(addr: usize, set: u32) {
    let pinmux = MMIO::new(addr);
    pinmux.write(set);
}

pub fn fix_uart1() {
    pinmux(
        PINMUX_BASE + FMUX_GPIO_0,
        1,
    );
    pinmux(
        PINMUX_BASE + FMUX_GPIO_1,
        1,
    );
}
