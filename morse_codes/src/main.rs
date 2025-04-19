#![no_std]
#![no_main]

pub mod coder;
pub mod constants;
pub mod signal;
pub mod timer;

use crate::constants::WORD_GAP;
use crate::signal::Signal;
use crate::timer::delay;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Lighthouse Morse Code - 'Hello' started!");

    let p = Peripherals::take().unwrap();

    p.P0.pin_cnf[2].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled()
    });
    let word = "HELLO";
    let mut my_buf: [u8; 4] = [0; 4];

    loop {
        for letter in word.chars() {
            let l = letter.encode_utf8(&mut my_buf);
            Signal::send_letter(&p, l);
            // delay(LETTER_GAP);
        }

        delay(WORD_GAP);
    }
}
