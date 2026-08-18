use core::ptr::{read_volatile, write_volatile};

// Gic Distributor
const GICD_BASE: usize = 0xFF84_1000;
// Gic CPU Interface
const GICC_BASE: usize = 0xFF84_2000;

const GICD_CTLR: *mut u32 = GICD_BASE as *mut u32;
const GICD_TYPER: *const u32 = (GICD_BASE + 0x004) as *const u32;

const GICD_IGROUPR0: *const u32 = (GICD_BASE + 0x080) as *const u32;
const GICD_ISENABLER0: *mut u32 = (GICD_BASE + 0x100) as *mut u32;

const GICD_ISPENDR0: *const u32 = (GICD_BASE + 0x200) as *const u32;


const GICC_CTLR: *mut u32 = GICC_BASE as *mut u32;
const GICC_PMR: *mut u32 = (GICC_BASE + 0x004) as *mut u32;

const GICC_IAR: *const u32 = (GICC_BASE + 0x00C) as *const u32;

const GICC_EOIR: *mut u32 = (GICC_BASE + 0x010) as *mut u32;


const TIMER_PPI_ID: u32 = 30;
const TIMER_PPI_MASK: u32 = 1 << TIMER_PPI_ID;


pub fn init() {
    unsafe {
        // Allow all interrupt priorities
        write_volatile(GICC_PMR, 0xFF);

        // Enable CPU interface
        write_volatile(GICC_CTLR, 1);

        // Enable Distributor
        write_volatile(GICD_CTLR, 1);
    }

    enable_timer_ppi();
}

pub fn enable_timer_ppi() {
    unsafe {
        write_volatile(
        GICD_ISENABLER0, 
        TIMER_PPI_MASK
        );
    }
}

pub fn timer_ppi_pending() -> bool {
    unsafe {
        read_volatile(GICD_ISPENDR0) & TIMER_PPI_MASK != 0
    }
}

pub fn timer_ppi_enabled() -> bool {
    unsafe {
        read_volatile(GICD_ISENABLER0) & TIMER_PPI_MASK != 0
    }
}

pub fn timer_ppi_is_group1() -> bool {
    unsafe {
        read_volatile(GICD_IGROUPR0) & TIMER_PPI_MASK != 0
    }
}

pub fn distributor_control() -> u32 {
    unsafe {
        read_volatile(GICD_CTLR)
    }
}

pub fn cpu_control() -> u32 {
    unsafe {
        read_volatile(GICC_CTLR)
    }
}

pub fn acknowledge_interrupt() -> u32 {
    unsafe {
        read_volatile(GICC_IAR)
    }
}

pub fn interrupt_id(iar: u32) -> u32 {
    iar & 0x3FF
}

pub fn typer() -> u32 {
    unsafe {
        read_volatile(GICD_TYPER)
    }
}

pub fn end_interrupt(iar: u32) {
    unsafe {
        write_volatile(GICC_EOIR, iar);
    }
}