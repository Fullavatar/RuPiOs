use crate::{drivers::{gic, uart}, system::{exceptions, interrupts, timer}};

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
    } else if input == b"el" {
        exception_level();
    } else if input == b"fault" {
        fault();
    } else if input == b"gic" {
        gic_info();
    } else if input == b"timerirq" {
        timer_irq();
    } else if input == b"timerstatus" {
        timer_status();
    } else if input == b"irqon" {
        enable_irq();
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
    uart::write_str("  el        - Show current ARM exception level\r\n");
    uart::write_str("  fault     - Trigger a test exception\r\n");
    uart::write_str("  gic       - Show gic info\r\n");
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

fn exception_level() {
    let level = exceptions::current_level();

    uart::write_fmt(
        format_args!("Exception level : EL{}\r\n", level)
    );
}

fn fault() {
    uart::write_str("Triggering undefined instruction...\r\n");

    exceptions::trigger_undefined_instruction()
}

fn gic_info() {
    let typer = crate::drivers::gic::typer();
    let dist = crate::drivers::gic::distributor_control();
    let cpu = crate::drivers::gic::cpu_control();
    let timer_group1 = crate::drivers::gic::timer_ppi_is_group1();
    let timer_enabled = crate::drivers::gic::timer_ppi_enabled();

    uart::write_fmt(format_args!("GICD_TYPER : {:#010x}\r\n", typer));

    uart::write_fmt(format_args!("GICD_CTLR  : {:#010x}\r\n", dist));

    uart::write_fmt(format_args!("GICC_CTLR  : {:#010x}\r\n", cpu));

    uart::write_fmt(format_args!("Timer PPI group 1 : {}\r\n", timer_group1));

    uart::write_fmt(format_args!("Timer PPI enabled : {}\r\n", timer_enabled));

}

fn timer_irq() {
    timer::schedule_interrupt_ms(1000);

    uart::write_str(
        "Timer IRQ scheduled in 1 second (CPU IRQ still masked)\r\n"
    );
}

fn timer_status() {
    let control = timer::interrupt_control();
    let timer_pending = timer::interrupt_pending();
    let gic_pending = gic::timer_ppi_pending();

    uart::write_fmt(format_args!(
        "CNTP_CTL_EL0       : {:#010x}\r\n",
        control
    ));

    uart::write_fmt(format_args!(
        "Timer condition    : {}\r\n",
        timer_pending
    ));

    uart::write_fmt(format_args!(
        "GIC PPI 30 pending : {}\r\n",
        gic_pending
    ));
}

fn enable_irq() {
    uart::write_str("Enabling CPU interrupts...\r\n");
    interrupts::enable_irq();
}

fn unknown_command() {
    uart::write_str("Unknown command\r\n");
}