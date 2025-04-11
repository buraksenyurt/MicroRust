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
    rprintln!("Lighthouse project started!");

    // Doğrudan nrf52833_pac peripherals donanım bileşenlerine erişim sağlıyoruz.
    let p = Peripherals::take().unwrap();

    /*
        Python örnek kodu ışık ayarı için PIN0'ı kullanır ancak asıl eşleştirme farklıdır.

        micro:bit v2 pin eşleştirmesi:

        pin0 -> P0.02
        pin1 -> P0.03
        pin2 -> P0.04
    */

    // P0.02 için konfigürasyon ayarları yapılıyor.
    p.P0.pin_cnf[2].write(|w| {
        w.dir().output(); // P0.02 pinini çıkış olarak ayarlıyoruz. Sonuçta relay boardu kontrol etmek için sinyal gönderilecek
        w.input().disconnect(); // Giriş bağlantısını kesiyoruz.
        w.pull().disabled(); // Pull-up/pull-down dirençlerini devre dışı bırakılıyor.
        w.drive().s0s1(); // High-Current yerine standard drive ayarı yapılıyor. Bu standart GPIO operasyonları için daha elverişlidir.
        w.sense().disabled() // Giriş algılaması devre dışı bırakılıyor.
    });

    loop {
        rprintln!("Relay ON");
        // İlk önce P0.02 pinine HIGH yapıyoruz. 
        // Bunu yaparken unsafe blok açılıyor zira bu işlem donanım üzerinde doğrudan değişiklik gerçekleştirmekte.
        // 1 << 2 ifadesi ile P0.02 pinine HIGH sinyali gönderiyoruz.
        p.P0.outset.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 HIGH

        delay(400_000);

        rprintln!("Relay OFF");
        // Yaklaşık 1 saniyelik duraksamadan sonra ise yine unsafe kod bloğu
        // kullanarak, P0.02 pinine LOW sinyali gönderiyoruz.
        p.P0.outclr.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 LOW

        delay(800_000); // Yaklaşık 2 saniyelik gecikme
    }
}

fn delay(count: u32) {
    for _ in 0..count {
        nop();
    }
}

/*
    Bu örnekte micro:bit ile relay board üzerinden bir lamba(bulp) açılıp kapatılmakta.
    Bunun için micro:bit üzerindeki P0.02 pinini kullanıyoruz(Üzerinde 0 işareti olan pin).
    Relay board üzerindeki IN1 pinine bağlanıyor. Relay board üzerindeki GND pini de yine micro:bit üzerindeki GND pinine bağlanıyor.
    Relay board üzerindeki OUT pinleri ise lamba(bulp) ile bağlantı kurmak için kullanılıyor. 
    Arada da 1.5V pil var.
*/
