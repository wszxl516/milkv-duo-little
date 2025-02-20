const FMUX_GPIO_FUNCSEL_IIC0_SCL: usize = 0x70;
const FMUX_GPIO_FUNCSEL_IIC0_SCL_MASK: u32 = 0x7;
const FMUX_GPIO_FUNCSEL_IIC0_SCL_OFFSET: u32 = 0;

const FMUX_GPIO_FUNCSEL_IIC0_SDA: usize = 0x74;
const FMUX_GPIO_FUNCSEL_IIC0_SDA_MASK: u32 = 0x7;
const FMUX_GPIO_FUNCSEL_IIC0_SDA_OFFSET: u32 = 0;

const FMUX_GPIO_0: usize = 0x80;
const FMUX_GPIO_1: usize = 0x84;


const PINMUX_BASE: usize = 0x03001000;
use super::super::common::mmio::MMIO;
pub fn pinmux(addr: usize, clear: u32, set: u32) {
    let pinmux = MMIO::new(addr);
    let value = pinmux.read::<u32>() & !clear | set;
    pinmux.write(value);
}

pub fn fix_uart1() {
    pinmux(
        PINMUX_BASE + FMUX_GPIO_FUNCSEL_IIC0_SCL,
        FMUX_GPIO_FUNCSEL_IIC0_SCL_MASK << FMUX_GPIO_FUNCSEL_IIC0_SCL_OFFSET,
        0x7,
    );
    pinmux(
        PINMUX_BASE + FMUX_GPIO_FUNCSEL_IIC0_SDA,
        FMUX_GPIO_FUNCSEL_IIC0_SDA_MASK << FMUX_GPIO_FUNCSEL_IIC0_SDA_OFFSET,
        0x7,
    );
    pinmux(
        PINMUX_BASE + FMUX_GPIO_0,
        0,
        1,
    );
    pinmux(
        PINMUX_BASE + FMUX_GPIO_1,
        0,
        1,
    );
}
