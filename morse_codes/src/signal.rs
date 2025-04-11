use crate::constants::{DASH_DURATION, DOT_DURATION, ELEMENT_GAP};
use crate::timer::delay;
use nrf52833_pac::Peripherals;
use rtt_target::rprintln;

pub struct Signal;

impl Signal {
    pub fn send_dot(p: &Peripherals) {
        rprintln!(".");
        // Turn ON
        p.P0.outset.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 HIGH
        delay(DOT_DURATION);

        // Turn OFF
        p.P0.outclr.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 LOW
        delay(ELEMENT_GAP);
    }

    pub fn send_dash(p: &Peripherals) {
        rprintln!("-");
        // Turn ON
        p.P0.outset.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 HIGH
        delay(DASH_DURATION);

        // Turn OFF
        p.P0.outclr.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 LOW
        delay(ELEMENT_GAP);
    }
}
