use core::ptr::{read_volatile, write_volatile};
use core::fmt::{self, Write};

const UART0_BASE: usize = 0xFE20_1000;

const UART_DR: *mut u32 = UART0_BASE as *mut u32;
const UART_FR: *const u32 = (UART0_BASE + 0x18) as *const u32;
const UART_LCRH: *mut u32 = (UART0_BASE + 0x2C) as *mut u32;
const UART_CR: *mut u32 = (UART0_BASE + 0x30) as *mut u32;

const FR_TXFF: u32 = 1 << 5;
const FR_RXFE: u32 = 1 << 4;

const CR_UARTEN: u32 = 1 << 0;
const CR_TXE: u32 = 1 << 8;
const CR_RXE: u32 = 1 << 9;

pub fn init() {
	unsafe {
		// Disable UART
		write_volatile(UART_CR, 0);
		// C8 bits per character
		write_volatile(UART_LCRH, 0b11 << 5);
		// Enable UART, TX and RX
		write_volatile(
			UART_CR,
			CR_UARTEN | CR_TXE | CR_RXE,
		);
	}
}

pub fn write_byte(byte: u8) {
	unsafe {
		// Wait while TXE is full
		while read_volatile(UART_FR) & FR_TXFF != 0 {}

		// Write the octet
		write_volatile(UART_DR, byte as u32);
	}
}

pub fn write_str(text: &str) {
		for byte in text.bytes() {
		write_byte(byte);
	}
}

pub fn read_byte() -> u8 {
	unsafe {
		// Wait while RXE is empty
		while read_volatile(UART_FR) & FR_RXFE != 0 {}

		// Read the octet and return it
		read_volatile(UART_DR) as u8
	}
}

struct UartWriter;

impl Write for UartWriter {
	fn write_str(&mut self, text: &str) -> fmt::Result {
		crate::uart::write_str(text);
		Ok(())
	}
}

pub fn write_fmt(args: fmt::Arguments<'_>) {
	let mut writer = UartWriter;
	let _ = writer.write_fmt(args);
}