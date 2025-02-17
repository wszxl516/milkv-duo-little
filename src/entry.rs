use core::arch::naked_asm;

#[allow(named_asm_labels)]
#[naked]
#[no_mangle]
#[link_section = ".boot"]
unsafe extern "C" fn _entry() -> ! {
    naked_asm!(
    r#"
    # enable fp
    li x3, 0x1 << 13
    csrs mstatus, x3
    
    # set mxstatus to init value
    li x3, 0xc0638000
    csrw 0x7C0, x3

    # set plic_ctrl = 0
    li x3, 0x701FFFFC # plic_base + 0x1FFFFC
    li x4, 0
    sw x4 , 0(x3)

    # invalid I-cache
    li x3, 0x33
    csrc 0x7C2, x3
    li x3, 0x11
    csrs 0x7C2, x3
    # enable I-cache
    li x3, 0x1
    csrs 0x7C1, x3
    # invalid D-cache
    li x3, 0x33
    csrc 0x7C2, x3
    li x3, 0x12
    csrs 0x7C2, x3
    # enable D-cache
    li x3, 0x2
    csrs 0x7C1, x3

    // Primary hart
    la sp, stack_top

    // Clear bss section
    la a0, bss_start
    la a1, bss_end
    bgeu a0, a1, start_main
clear_bss:
    sd zero, 0(a0)
    addi a0, a0, 8
    bltu a0, a1, clear_bss

start_main:
    // argc, argv, envp is 0
    li  a0, 0
    li  a1, 0
    li  a2, 0
    call {setup}
    call {main}

secondary:
    wfi
    j secondary
	"#,
        setup = sym crate::arch::setup,
        main = sym crate::kernel_main,
    )
}

