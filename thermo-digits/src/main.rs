#![no_std]
#![no_main]

mod constants;
mod led_matrix;
mod panel;
mod utility;

use crate::constants::*;
use crate::led_matrix::LedMatrix;
use crate::utility::*;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let peri = Peripherals::take().unwrap();

    let p0 = peri.P0;
    let p1 = peri.P1;
    let timer0 = peri.TIMER0;
    let mut matrix = LedMatrix::new(p0, p1, timer0);

    let saadc = &peri.SAADC;

    saadc.enable.write(|w| w.enable().enabled());

    saadc.ch[0].config.write(|w| {
        w.resp().bypass();
        w.gain().gain1_6();
        w.refsel().internal();
        w.tacq()._10us();
        w.mode().se();
        w.burst().disabled();
        w
    });

    saadc.ch[0].pselp.write(|w| w.pselp().analog_input0());
    static mut RESULT: i16 = 0;
    let result_ptr: *mut i16 = &raw mut RESULT;

    saadc
        .result
        .ptr
        .write(|w| unsafe { w.ptr().bits(result_ptr as u32) });

    unsafe {
        saadc.result.maxcnt.write(|w| w.bits(1));
    }
    let temp_reg = peri.TEMP;

    loop {
        saadc.tasks_start.write(|w| unsafe { w.bits(1) });
        while saadc.events_started.read().bits() == 0 {}
        saadc.events_started.reset();

        saadc.tasks_sample.write(|w| unsafe { w.bits(1) });
        while saadc.events_end.read().bits() == 0 {}
        saadc.events_end.reset();

        temp_reg.tasks_start.write(|w| unsafe { w.bits(1) });
        while temp_reg.events_datardy.read().bits() == 0 {}
        temp_reg.events_datardy.reset();

        let adc_raw = unsafe { RESULT };

        // Kalibrasyon formülü MonkMakes sitesinden alınmıştır.
        let calibrated_temp = (A / B) * (adc_raw as f32) + C;
        rprintln!("Calibrated: {}", calibrated_temp);

        /*
            Elimizde sıcaklık değeri var. Bu değeri 5x5 LED matrisinde göstermek için
            önce float türündeki sıcaklık değerini ASCII türünden karakter dizisine dönüştürüyoruz.
            Ardından söz konusu diziyi 5x5'lik bitmap dizisine dönüştürüyoruz.
            Nihayetinde elde edilen bitmap dizisini LED matrisinde gösteriyoruz.
        */
        let ascii = float_to_ascii(calibrated_temp);
        let text = core::str::from_utf8(&ascii[..4]).unwrap();
        rprintln!("Text value {}", text);

        let bitmap = text_to_bitmap(text);

        scroll_text(&mut matrix, &bitmap, SCROLL_WIDTH);

        for _ in 0..(DELAY_FACTOR * 2) {
            // Yaklaşık 2 saniyelik gecikleme süresi
            nop();
        }
    }
}
