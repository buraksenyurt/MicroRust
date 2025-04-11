#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Lighthouse project started!");

    let p = Peripherals::take().unwrap();

    /*
        Python örnek kodu ışık ayarı için PIN0'ı kullanır ancak asıl map farklıdır.

        micro:bit v2 pin mapping:
        pin0 -> P0.02
        pin1 -> P0.03
        pin2 -> P0.04
    */
    p.P0.pin_cnf[2].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled()
    });

    loop {
        rprintln!("Relay ON");
        p.P0.outset.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 HIGH

        delay(400_000);

        rprintln!("Relay OFF");
        p.P0.outclr.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 LOW

        delay(800_000);
    }
}

fn delay(count: u32) {
    for _ in 0..count {
        nop();
    }
}
