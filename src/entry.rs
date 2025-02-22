use core::arch::naked_asm;

#[allow(named_asm_labels)]
#[naked]
#[no_mangle]
#[link_section = ".boot"]
unsafe extern "C" fn _entry() -> ! {
    naked_asm!(
    r#"
    .cfi_startproc
    .cfi_undefined ra
.option push
.option norelax
//    la  gp, __global_pointer$
.option pop

    // Continue primary hart
    csrr a0, mhartid
    bne  a0, x0, secondary

    li x1, 0
    li x2, 0
    li x3, 0
    li x4, 0
    li x5, 0
    li x6, 0
    li x7, 0
    li x8, 0
    li x9, 0
    li x10, 0
    li x11, 0
    li x12, 0
    li x13, 0
    li x14, 0
    li x15, 0
    li x16, 0
    li x17, 0
    li x18, 0
    li x19, 0
    li x20, 0
    li x21, 0
    li x22, 0
    li x23, 0
    li x24, 0
    li x25, 0
    li x26, 0
    li x27, 0
    li x28, 0
    li x29, 0
    li x30, 0
    li x31, 0
    csrw mie, x0
    csrw mstatus, x0
    // Primary hart
    la sp, stack_top

bss_clear:

    // Clear bss section
    la a0, bss_start
    la a1, bss_end
    bgeu a0, a1, bss_clear
    // argc, argv, envp is 0
    li  a0, 0
    li  a1, 0
    li  a2, 0
    call {setup}
    call {main}
loop:
    wfi
    j loop

secondary:
    // TODO: Multicore is not supported
    wfi
    j secondary
    .cfi_endproc
	"#,
        setup = sym crate::arch::setup,
        main = sym crate::kernel_main,
    )
}

