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

