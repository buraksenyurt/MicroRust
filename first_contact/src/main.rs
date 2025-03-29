#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // RTT (Real-Time Transfer) başlatılıyor
    rtt_init_print!();

    // RTT konsoluna mesaj yazdırılıyor
    rprintln!("Starting up...");

    loop {
        rprintln!("Hi!");
        // Yaklaşık 1 saniye bekle
        for _ in 0..400_000 {
            nop(); // no operation
        }
    }
}
/*
    Ekrana belli aralıklarla (saniyede bir) "Hi!" yazdıran bir program kodudur.
    Çıktılar carbo embed komutunun işletildiği işletim sistemindeki terminale yazdırılır.

    Mikrodenetleyici üzerinden Windows 11 makinesine mesaj göndermek için RTT (Real-Time Transfer) kullanılır.
    RTT, mikrodenetleyici ile bilgisayar arasında gerçek zamanlı veri iletimi sağlar.

    Program, bir mikrodenetleyici üzerinde çalışacak şekilde tasarlanmıştır ve
    `#![no_std]` ve `#![no_main]` öznitelikleri ile başlar. Bu öznitelikler, programın
    rust standart kütüphanesini kullanmadığını ve kendi giriş noktasını (entry point) tanımladığını belirtir.

    `cortex_m_rt` ve `cortex_m` kütüphaneleri, Cortex-M işlemcileri için düşük seviyeli erişim sağlar.
    `panic_halt` kütüphanesi, programın bir hata ile karşılaştığında durmasını sağlar.
    `rtt_target` kütüphanesi, RTT konsolunu başlatmak ve mesajları yazdırmak için kullanılır.
    `rprintln!` makrosu, RTT konsoluna mesaj yazdırmak için kullanılır.
    `rtt_init_print!()` makrosu, RTT konsolunu yazdırma modunda başlatır.
    `nop()` fonksiyonu, her yazdırma ifadesi arasında bir gecikme oluşturmak için kullanılır. Bunun için
    400_000 kez döngüye girilir. Bu, yazdırma işlemi ile bir sonraki yazdırma işlemi arasında
    belirli bir süre beklemek için kullanılır.
*/