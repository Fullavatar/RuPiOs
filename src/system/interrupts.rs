use core::arch::asm;

pub fn enable_irq() {
    unsafe {
        asm!(
            "msr daifclr, #2",
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub fn disable_irq() {
    unsafe {
        asm!(
            "msr daifset, #2",
            options(nomem, nostack, preserves_flags)
        );
    }
}