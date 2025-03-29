#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use microbit::{board::Board, hal::timer::Timer};
use panic_halt as _;

#[entry]
fn start() -> ! { // "!" işareti, fonksiyonun sonsuz döngüde çalışacağını belirtir

    let mut board = Board::take().unwrap(); // Board'un sahipliğini alıyoruz
    let _ = board.display_pins.col3.set_low(); // Işığı kapatıyoruz
    let mut row3 = board.display_pins.row3; // row3 pinini alıyoruz
    let mut timer = Timer::new(board.TIMER0); // Timer'ı başlatıyoruz

    loop {
        let _ = row3.set_low(); // row3 pinini LOW yapıyoruz (Işığı kapatır)
        timer.delay_ms(1_500); // 1.5 saniye bekliyoruz
        let _ = row3.set_high(); // row3 pinini HIGH yapıyoruz (Işığı yakar)
        timer.delay_ms(1_500);  // 1.5 saniye bekliyoruz
    }
}
/*
    Program mikrodenetleyici üzerinde bir LED'in yanıp sönmesini sağlamak için kullanılır.
    İşleri kolaylaştırmak için Hal (Hardware Abstraction Layer) ve embedded_hal kütüphanelerini kullanır.

    - `#![no_std]` direktifi standart kütüphaneyi kullanmadan çalışacak bir uygulama yazdığımızı belirtir.
    - `cortex_m_rt::entry` makrosu, uygulamanın başlangıç noktasını belirtir.
    - `embedded_hal` kütüphanesi, donanım arayüzleri için soyutlamalar sağlar.
    - `microbit` kütüphanesi, mikrodenetleyici üzerinde çalışmak için gerekli fonksiyonları ve yapıları içerir.
    - `panic_halt` kütüphanesi, bir hata durumunda programın durmasını sağlar.
    - `set_low()` fonksiyonu LED'i kapatır, `set_high()` fonksiyonu ise LED'i açar.
    - `Timer::new()` fonksiyonu, zamanlayıcıyı başlatır.
    - `delay_ms(1_500)` fonksiyonu, LED'in yanıp sönmesi için 1.5 saniye bekler.
*/