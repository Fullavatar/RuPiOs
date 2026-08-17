#![no_std]
#![no_main]

mod uart;
mod shell;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
	r#"
    .section .text._start
    .global _start

_start:
    mrs x0, mpidr_el1
    and x0, x0, #0xff
    cbz x0, 2f

1:
    wfe
    b 1b

2:
    ldr x0, =__stack_top
    mov sp, x0

    bl kernel_main

3:
    wfe
    b 3b
"#
);

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    uart::init();
    uart::write_str("Welcome to RuPiOs!\n");
    shell::run();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {
		core::hint::spin_loop();
	}
}