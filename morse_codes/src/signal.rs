//! # Signal Module
//! 
//! Bu modül, Morse kodunu göndermek için kullanılan sinyal gönderim fonksiyonlarını içerir.
//! Gönderim işlemi için NRF52833 mikrodenetleyicisinin GPIO pinlerini kullanır.
//! Doğrudan periferals üzerinden pinleri kontrol eder.
//! 
//! ## Example
//! 
//! Örnek bir kullanımı aşağıdaki gibidir.
//! 
//! ```rust
//! use morse_codes::signal::Signal;
//! use nrf52833_pac::Peripherals;
//! let p = Peripherals::take().unwrap();
//! Signal::send_letter(&p, "A");
//! ```

use crate::coder::Coder;
use crate::constants::{DASH_DURATION, DOT_DURATION, ELEMENT_GAP};
use crate::timer::delay;
use nrf52833_pac::Peripherals;
use rtt_target::rprintln;


/// Sinyal gönderim fonksiyonlarını içeren veri modeli.
pub struct Signal;

impl Signal {
    /// Kısa sinyal gönderimi (nokta) için kullanılan fonksiyon.
    ///
    /// ## Arguments
    /// * `p` - NRF52833 mikrodenetleyicisinin periferals yapısı.
    ///
    /// ## Example
    /// ```rust
    /// use morse_codes::signal::Signal;
    /// use nrf52833_pac::Peripherals;
    /// let p = Peripherals::take().unwrap();
    /// Signal::send_dot(&p);
    /// ```
    fn send_dot(p: &Peripherals) {
        rprintln!(".");
        // Turn ON
        p.P0.outset.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 HIGH
        delay(DOT_DURATION);

        // Turn OFF
        p.P0.outclr.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 LOW
        delay(ELEMENT_GAP);
    }

    /// Uzun sinyal gönderimi (çizgi) için kullanılan fonksiyon.
    ///
    /// ## Arguments
    /// * `p` - NRF52833 mikrodenetleyicisinin periferals yapısı.
    ///
    /// ## Example
    /// ```rust
    /// use morse_codes::signal::Signal;
    /// use nrf52833_pac::Peripherals;
    /// let p = Peripherals::take().unwrap();
    /// Signal::send_dash(&p);
    /// ```
    fn send_dash(p: &Peripherals) {
        rprintln!("-");
        // Turn ON
        p.P0.outset.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 HIGH
        delay(DASH_DURATION);

        // Turn OFF
        p.P0.outclr.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 LOW
        delay(ELEMENT_GAP);
    }

    /// Belirtilen harfi Morse kodu ile göndermek için kullanılan fonksiyon.
    ///
    /// ## Arguments
    /// * `p` - NRF52833 mikrodenetleyicisinin periferals yapısı.
    /// * `letter` - Gönderilecek harf.
    ///
    /// ## Example
    /// ```rust
    /// use morse_codes::signal::Signal;
    /// use nrf52833_pac::Peripherals;
    /// let p = Peripherals::take().unwrap();
    /// Signal::send_letter(&p, "A");
    /// ```
    pub fn send_letter(p: &Peripherals, letter: &str) {
        let code = Coder::get_letter_code(letter);
        rprintln!("{} = {}", letter, code);
        for c in code.chars() {
            if c == '.' {
                Self::send_dot(p)
            } else if c == '-' {
                Self::send_dash(p)
            }
        }
    }
}
