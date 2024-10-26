pub const UART_BASE: usize = 0x04140000;
pub const UART_BASE1: usize = 0x04150000;
pub const SYS_CLOCK_HZ: u64 = 25 * 1000 * 1000;
pub const UART_BAUD_RATE: u64= 115200;

pub const  MAILBOX_REG_BASE: usize = 0x01900000;
pub const MAILBOX_REG_BUFF: usize = MAILBOX_REG_BASE + 0x0400;
pub const SPINLOCK_REG_BASE: usize = MAILBOX_REG_BASE + 0x00c0;
pub const CLINT_BASE: usize = 0x74000000;
pub const PLIC_BASE: usize =  0x70000000;

//2MB
pub const MEM_SIZE: usize = 0x200000;

pub const MEM_START: usize = 0x8fe00000;
