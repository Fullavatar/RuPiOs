#![no_std]
#![no_main]

mod boot;
mod drivers;
mod system;
mod shell;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    drivers::uart::init();
    drivers::uart::write_str("Welcome to RuPiOs!\n");
    shell::run();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {
		core::hint::spin_loop();
	}
}