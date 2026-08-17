use crate::{timer, uart};

const BUFFER_SIZE: usize = 64;

pub fn run() -> ! {
    loop {
        uart::write_str("RuPiOs> ");

        let mut buffer = [0u8; BUFFER_SIZE];
        let len = read_line(&mut buffer);

        let input = &buffer[..len];

        execute(input);
    }
}

fn read_line(buffer: &mut [u8]) -> usize {
    let mut len = 0;

    loop {
        let byte = uart::read_byte();

        match byte {
            b'\r' | b'\n' => {
                uart::write_str("\r\n");
                return len;
            }

            8 | 127 => {
                if len > 0 {
                    len -= 1;
                    uart::write_str("\x08 \x08");
                }
            }

            _ => {
                if len < buffer.len() {
                    buffer[len] = byte;
                    len += 1;

                    uart::write_byte(byte);
                }
            }
        }
    }
}

fn execute(input: &[u8]) {
    if input == b"help" {
        help();
    } else if input == b"about" {
        about();
    } else if input == b"uptime" {
        uptime();
    } else if input == b"timerfreq" {
        timer_frequency();
    } else if input == b"sleep" {
        sleep();
    } else if input.is_empty() {
        // Do nothing
    } else {
        unknown_command();
    }
}

fn help() {
    uart::write_str("Available commands:\r\n");
    uart::write_str("  help      - Show this help\r\n");
    uart::write_str("  about     - Show RuPiOs information\r\n");
    uart::write_str("  uptime    - Show system uptime\r\n");
    uart::write_str("  timerfreq - Show system timer frequency\r\n");
    uart::write_str("  sleep     - Wait for 1 second\r\n");
}

fn about() {
    uart::write_str("RuPiOs v");
    uart::write_str(env!("CARGO_PKG_VERSION"));
    uart::write_str(" - Rust Raspberry Pi OS\r\n");
}

fn uptime() {
    let seconds = timer::uptime_seconds();

    uart::write_fmt(format_args!("Uptime : {} seconds\r\n", seconds));
}

fn timer_frequency() {
    let hertz = timer::frequency();

    uart::write_fmt(format_args!("Timer frequency : {} Hz\r\n", hertz));
}

fn sleep() {
    uart::write_str("Sleeping for 1 second...\r\n");
    timer::sleep_ms(1000);
    uart::write_str("Awake!\r\n");
}

fn unknown_command() {
    uart::write_str("Unknown command\r\n");
}