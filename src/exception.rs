use core::arch::asm;

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