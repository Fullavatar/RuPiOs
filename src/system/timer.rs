use core::{arch::asm, sync::atomic::{AtomicU64, Ordering}};

static INTERRUPT_COUNT: AtomicU64 = AtomicU64::new(0);

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

pub fn schedule_interrupt_ms(milliseconds: u32) {
    let timer_ticks = frequency() * milliseconds as u64 / 1000;

    unsafe {
        core::arch::asm!(
            "msr cntp_tval_el0, {ticks}",
            "msr cntp_ctl_el0, {control}",
            "isb",
            ticks = in(reg) timer_ticks,
            control = in(reg) 1u64
        );
    }
}

pub fn interrupt_control() -> u64 {
    let value: u64;

    unsafe {
        core::arch::asm!(
            "mrs {}, cntp_ctl_el0",
            out(reg) value
        );
        value
    }
}

pub fn interrupt_pending() -> bool {
    interrupt_control() & (1 << 2) != 0
}

pub fn handle_interrupt() {
    INTERRUPT_COUNT.fetch_add(1, Ordering::Relaxed);

    schedule_interrupt_ms(1000);
}

pub fn interrupt_count() -> u64 {
    INTERRUPT_COUNT.load(Ordering::Relaxed)
}

pub fn disable_interrupt() {
    unsafe {
        core::arch::asm!(
            "msr cntp_ctl_el0, xzr",
            "isb",
        );
    }
}