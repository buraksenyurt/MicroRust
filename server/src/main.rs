#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use microbit::{
    display::blocking::Display,
    hal::{
        Timer,
        uarte::{Baudrate, Parity, Uarte},
    },
};
use rtt_target::rprintln;
use rtt_target::rtt_init_print;

use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    rtt_init_print!(); // RTT hedefini başlatıyoruz
    rprintln!("Server is starting...");

    let board = microbit::Board::take().unwrap(); // Board nesnesinin sahipliği alınıyor
    let mut timer = Timer::new(board.TIMER0); // Zamanlayıcı başlatılıyor
    let mut display = Display::new(board.display_pins); // LED ekranı başlatılıyor
    let mut buffer = [0u8; 1]; // UART'tan gelen veriyi tutacak bir buffer tanımlanıyor. Bu örnekte tek byte bilgi okuyoruz.

    let mut uart = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    ); // UART modülü başlatılıyor
    // Parity kontrolünün devre dışı bırakıldığı ve baudrate değerinin 115200 hz olduğu UARTE0 modül nesnesi alınıyor.

    loop {
        // Eğer UART'tan veri okunabiliyorsa
        if uart.read(&mut buffer).is_ok() {
            let command = buffer[0]; // Okunan veriyi alıyoruz
            rprintln!(
                "[INFO] Received Command: ({}) - {}",
                command as char,
                command
            );
            let command = command as char;

            // Pattern matching ile gelen komutları kontrol ediyoruz
            // ve uygun işlemleri gerçekleştiriyoruz.
            match command {
                'h' => {
                    write!(uart, "\r\nUsages\r\n").unwrap();
                    write!(uart, "h: Help\r\n").unwrap();
                    write!(uart, "r: Reset LEDs\r\n").unwrap();
                    write!(uart, "o: Open LEDs\r\n").unwrap();
                }
                'r' => {
                    rprintln!("[INFO] Resetting LEDs");
                    write!(uart, "\r\nResetting LEDs\r\n").unwrap();
                    display.clear();
                }
                'o' => {
                    rprintln!("[INFO] Opening LEDs");
                    write!(uart, "\r\nOpening LEDs\r\n").unwrap();

                    // Ekranda gülümseyen surat simgesi gösteriliyor
                    // Bunun için yine 5X5 boyutunda bir dizi tanımlanıyor
                    let plus_symbol = [
                        [0, 0, 0, 0, 0],
                        [0, 1, 0, 1, 0],
                        [0, 0, 0, 0, 0],
                        [1, 0, 0, 0, 1],
                        [0, 1, 1, 1, 0],
                    ];
                    display.show(&mut timer, plus_symbol, 1000u32); // 1 saniye boyunca LED matris yanıyor
                }
                _ => {
                    // Eğer tanınmayan bir komut gelirse hata mesajı gönderiliyor
                    rprintln!("[ERROR] Unknown command: {}", command);
                    write!(uart, "[ERROR] Unknown command: {}\r\n", command).unwrap();
                    write!(uart, "h: Help\r\n").unwrap();
                }
            }
        } else {
            rprintln!("Failed to read from UART");
        }
        timer.delay(1_000_000);
    }
}

/*
    Bu program koduna göre mikrodenetleyici UART üzerinden gelen komutları dinler.
    Gelen komutlar arasında 'h' (yardım), 'r' (LED'leri sıfırla) ve 'o' (LED'leri aç) bulunmaktadır.
    'h' komutu alındığında, kullanılabilir komutların listesi gönderilir.
    'r' komutu alındığında, LED'ler sıfırlanır ve ekran temizlenir.
    'o' komutu alındığında, LED'ler açılır ve ekranda  1 saniye kadar gülen surat gösterilir.

    İletişim UARTE0 modülü üzerinden yapılmaktadır. Mikrodenetleyici bir server gibi davranış sergilerken,
    bağlandığı bilgisayar veya başka bir cihazdan gelen komutlar aktarılabilir
*/
