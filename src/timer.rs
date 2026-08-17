use core::arch::asm;

pub fn frequency() -> u64 {
    let value: u64;

    unsafe {
        asm!(
            "mrs {}, cntfrq_el0",
            out(reg) value
        );
    }

    value
}

pub fn ticks() -> u64 {
    let value: u64;

    unsafe {
        asm!(
            "mrs {}, cntpct_el0",
            out(reg) value
        );
    }

    value
}

pub fn uptime_seconds() -> u64 {
    ticks() / frequency()
}

pub fn sleep_ms(milliseconds: u32) {
    let duration_ticks = frequency() * milliseconds as u64 / 1000;
    let start = ticks();

    while ticks().wrapping_sub(start) < duration_ticks {
        core::hint::spin_loop();
    }
}