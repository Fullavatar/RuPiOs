use core::arch::asm;
use crate::drivers::uart;

#[unsafe(no_mangle)]
pub extern "C" fn exception_handler(esr: u64, elr: u64, far: u64) -> ! {
    uart::write_str("\r\n--- RuPiOs Exception ---\r\n");

    uart::write_fmt(
        format_args!("ESR_EL1 : {:#018x}\r\n", esr)
    );

    uart::write_fmt(
        format_args!("ELR_EL1 : {:#018x}\r\n", elr)
    );

    uart::write_fmt(
        format_args!("FAR_EL1 : {:#018x}\r\n", far)
    );

    uart::write_str("System halted.\r\n");

    loop {
        core::hint::spin_loop();
    }
}

pub fn trigger_undefined_instruction() -> ! {
    unsafe {
        core::arch::asm!(
            "udf #0",
            options(noreturn)
        );
    }
}

pub fn current_level() -> u8 {
    let value: u64;

    unsafe {
        asm!(
            "mrs {}, CurrentEL",
            out(reg) value
        );
    }

    ((value >> 2) & 0b11) as u8
}