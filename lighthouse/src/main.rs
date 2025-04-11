#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{hal::gpio::Level, Board};
use embedded_hal::digital::OutputPin;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Lighthouse sample is starting...");

    if let Some(board) = Board::take() {
        let mut relay = board.pins.p0_01.into_push_pull_output(Level::Low);

        loop {
            relay.set_high().unwrap();
            delay(1_000_000);

            relay.set_low().unwrap();
            delay(2_000_000);
        }
    }

    loop {

    }
}

fn delay(count: u32) {
    for _ in 0..count {
        cortex_m::asm::nop();
    }
}
