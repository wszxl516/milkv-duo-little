#[repr(C, packed(8))]
#[derive(Copy, Clone, Debug, Default)]
pub struct ValidT {
	linux_valid: u8,
	rtos_valid: u8
}

#[repr(C, packed(8))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Opration {
    pub ip_id: u8,
    pub cmd_id: u8,
    pub valid: ValidT,
    pub param: i32,
}