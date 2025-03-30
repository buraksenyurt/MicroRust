#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    hal::uarte::{Baudrate, Parity, Uarte},
    Board,
};
use rtt_target::rtt_init_print;
use rtt_target::rprintln;

use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    // Message gönderimi için gerekli olan bileşenleri başlatıyoruz
    // ve UART haberleşmesini başlatıyoruz.
    rtt_init_print!();
    rprintln!("UART Comm is starting...");

    let board = Board::take().unwrap(); // Micro:bit kartının sahipliğini alıyoruz

    let mut uart = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    ); // UART modülü hazırlanıyor. write metodu ile mesaj gönderimi yapacak.
       // Parity::EXCLUDED ile parity kontrolü devre dışı bırakılır.
       // Baudrate::BAUD115200 değeri ile de 115200 baud hızında bir haberleşme ayarlanır.
       // UARTE0 kart üzerindeki UART modülüdür.

    let message = b"Hello from Micro:bit!\r\n"; // Gönderilecek ifadeyi byte array olarak tanımlıyoruz.

    loop {
        // Her bir byte için döngü başlatıyoruz.
        for byte in message.iter() {
            // UART modülü üzerinden her bir byte'ı gönderiyoruz.
            uart.write(&[*byte]).unwrap();
        }
        // delay değerini hesaplamak için mikrodenetleyicinin saat hızını ele almak yöntemlerden birisi.
        // delay = clock_speed * time
        // Micro:bit V2.2 kartı spesifikasyonlarına göre ARM Cortex işlemcisi (nRF52833)
        // 64 MHz hızında çalışmakta 
        // Detaylar için; (https://docs.nordicsemi.com/bundle/ps_nrf52833/page/keyfeatures_html5.html)

        // 64 MHz hızında çalışan bir sistemi 2 saniye beklemek için
        // 64_000_000 * 2 = 128_000_000 değeri kullanılabilir
        cortex_m::asm::delay(128_000_000); // Yaklaşık 2 saniye bekleme süresi

        // Bu örnekte kullanılan asm::delay düşük seviyeli bir çözümdür ve oldukça hızlıdır.
        // Ancak dikkat etmek gerekir zira, işlemciyi tamamen beklemeye alır 
        // ve diğer görevlerin çalışmasını engeller.
        // Dolayısıyla daha karmaşık uygulamalarda bir zamanlayıcı kullanmak daha iyi bir seçenek olabilir.
    }
}

/*
    Bu örnek Microbit kartından UART protokolü ile veri gönderimi yapmaktadır.
    UARTE modülünü kullanarak, belirli bir baudrate ve parity ayarları ile UART haberleşmesi başlatılır.
    Ardından, bir mesaj dizisi tanımlanır ve bu mesaj sürekli olarak UART üzerinden gönderilir.
*/
