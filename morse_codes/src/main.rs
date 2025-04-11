#![no_std]
#![no_main]

mod coder;
mod constants;
mod signal;
mod timer;

use cortex_m_rt::entry;
use panic_rtt_target as _;
#[entry]
fn main() -> ! {
    loop {}
}
