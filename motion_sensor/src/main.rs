#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = Peripherals::take().unwrap();
    p.P0.pin_cnf[2].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled()
    });

    loop {
        p.P0.outset.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 HIGH
        delay(200_000);
        p.P0.outclr.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 LOW
        delay(400_000);
    }
}

fn delay(count: u32) {
    for _ in 0..count {
        nop();
    }
}
