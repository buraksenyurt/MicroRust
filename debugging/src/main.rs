#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn run() -> ! {
    let mut counter = 0;
    loop {
        counter += 1;
        for _ in 0..400_000 {
            nop(); // no operation
        }
    }
}
