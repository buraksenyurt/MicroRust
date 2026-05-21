# MicroRust

Bu repoda mikrodenetleyiciler üzerinde Rust ile kodlama pratiklerine yer verilmesi planlanmaktadır.

- [İçerik](#microrust)
  - [Microbit Üzerinde Rust ile Program Geliştirme](#microbit-üzerinde-rust-ile-program-geliştirme)
    - [Cihaz Hakkında](#cihaz-hakkında)
    - [Gerekli Kurulumlar](#gerekli-kurulumlar)
    - [Örnekler](#örnekler)
      - [Fist Contact](#first-contact)
      - [Debugging](#debugging)
      - [Blinking Led](#blinking-led)
      - [Blinking Led v2](#blinking-led-v2)
      - [Blinking Rust](#blinking-rust)
      - [Beep](#beep)
      - [Uart](#uart)
      - [Server](#server)
      - [Accelerometer](#accelerometer)
      - [Thermometer v1](#thermometer-v1)
      - [Thermo v2](#thermo2)
      - [Funny Led](#funny-led)
      - [Thermo Digits](#thermo-digits)
      - [Lighthouse](#lighthouse)
      - [Morse Codes](#morse-codes)
      - [Motion Sensor](#motion-sensor)
    - [Mini Sözlük](#mini-sözlük)
    - [Kaynaklar](#kaynaklar)

## Microbit Üzerinde Rust ile Program Geliştirme

Mikrodenetleyiciler genel olarak sınırlı kapasiteye sahip, çoğunlukla işletim sistemi ile birlikte gelmeyen, çeşitli sensörler yardımıyla çevresel ortamlardan veri toplanması gibi işlerde sıklıkla kullanılan entegre kartlardır. Portatif ve ekonomik olmaları birçok düzeneğe dahil edilmelerini mümkün kılar. Mikrodenetleyiciler üzerine geliştirme yapmak için farklı programlama dilleri kullanılabilir ancak **RTOS _(real-time operating system-RTOS)_** ile birlikte gelmedikleri durumlarda bare-metal programming pratiklerini uygulamak gerekir.

Bu çalışmaya konu olan [BBC micro:bit](https://microbit.org/) üzerinde **Python**, **Scratch**, **Microsoft Make** ile programlama yapılabileceği gibi **C** ve **Rust** gibi dillerle de geliştirme yapmak mümkündür.

## Destekleyici Videolar

Buradaki örneklerle ilgili yardımcı birkaç video da paylaşmak isterim.

- [Embedded Rust - Introduction (Hello LEDs)](https://youtu.be/j2TuYY7M_nM?si=jnI2LuHrHbsuQ5Ve)
- [Micro:Bit Üzerinde Rust Maceraları - Mors Kodları ile HELLO](https://youtu.be/VRweOQozLxo?si=DG_u5MzFomaVnHnm)
- [Micro:Bit Üzerinde Rust Maceraları - Mors Kodları ile HELLO Kodlama Safhası](https://youtu.be/W7sG9U7amCA?si=xIEXAGjkg8SwRiT5)

## Cihaz Hakkında

Bu repodaki örnekler **BBC Micro:bit v2.2** üzerinde geliştirilmektedir. **ARM** tabanlı **Cortex** işlemciye _(nRF52833, Nordic Semiconductor)_ sahip olan cihaz **512 Kb Flash** ve **128 Kb Ram** belleğe sahiptir.

![Micro:bit 00](./images/MicroBit_00.jpg)

![Micro:bit 01](./images/MicroBit_01.jpg)

- Doğrudan Microcontroller Unit üzerinde programlama yaparken kartın donanım şema bilgilerine ihtiyaç duyulacaktır. [Kaynak](https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF)

Mikrodenetleyici USB üzerinden bilgisayara bağlanabilir. Bilgisayara bağlandıktan sonra **COM3** portundan bağlı bir cihaz gibi de algılanır.

```bash
# Kontrol için
mode
```

![COM3 Status](./images/MicroBit_02.png)

## Gerekli Kurulumlar

Örnekler **Windows 11** işletim sistemi üzerinde geliştirilmektedir.

```bash
# Sistemde rust'ın yüklü olduğu varsayılmıştır

rustup component add llvm-tools
cargo install cargo-binutils
cargo install cargo-embed
cargo binstall probe-rs-tools

# Micro:bit v2.2 sürümü için gerekli target enstrümanlarını ekleyelim
rustup target add thumbv7em-none-eabihf

# arm-none-eabi-gdb kurulum içinse 
# https://developer.arm.com/downloads/-/gnu-rm
```

## Örnekler

### First Contact

Bu örnek düzenli aralıklarla Windows makinedeki terminal ekranına mesaj göndermektedir. Uygulamada aşağıdaki toml dosyası kullanılmakta. Görüldüğü üzere belli başlı bağımlılıklar söz konusu.

Cargo.toml

```toml
[package]
name = "first_contact"
description = "First contact with the BBC Micro:bit V2.2 microcontroller"
authors = ["Burak Selim Şenyurt"]
version = "0.1.0"
edition = "2024"

[dependencies]
cortex-m = { version = "0.7.7", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7.5"
panic-halt = "1.0.0"
rtt-target = "0.6.1"
```

Diğer iki kaynaksa **Embed.toml** ve **memory.x** isimli aşağıdaki içeriğe sahip dosyalar. Genel olarak diğer örneklerde de benzer ekipmanlar kullanılmakta.

```toml
[default.general]
chip = "nrf52833_xxAA"

[default.reset]
halt_afterwards = false

[default.rtt]
enabled = true

[default.gdb]
enabled = false
```

ve **memory.x** dosyası

```text
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
```

Rust kodları ise şöyle.

```rust
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
```

Kod kontrolü için **check** komutu kullanılabilir. Ancak cihaz üzerine dağıtım yapılmadan asıl sonuçları göremeyiz. Bu genellikle **embed** komutu ile yapılır. Bu komut cihazın flash belleğine kodu yükler ve çalıştırır.

```bash
# Kod kontrolü
cargo check

# Flashing (Cihaza dağıtım)
cargo embed
```

Beklenen çıktı.

![First Contact Runtime](./images/MicroBit_03.png)

### Debugging

Terminal bazlı debug operasyonlarında **GDB** kullanılır. Dikkat edilmesi gereken noktalardan birisi Embed.toml dosyasındaki default.gdb ayarının **enabled = true** olarak ayarlanmış olmasıdır. Çalışma sırasındaki denemelerim ise şöyle;

```bash
# Klasik kontroller
cargo check
cargo build

# İlk terminalde aşağıdaki komut çalıştırılır ve flashing yapılır
cargo embed

# İkinci bir terminalde debug server'a bağlanılarak ilerlenir
arm-none-eabi-gdb .\target\thumbv7em-none-eabihf\debug\debugging

# gdb terminali açıldıktan sonra debug server'a bağlanılır
target remote :1337

# main.rs içerisinde bir satıra breakpoint eklemek için
break main.rs:12

# breakpoint noktasına gitmek için
continue

# local değişkenlerin durumunu görmek için
info locals

# Değişken değerini yazdırmak için
print counter
# Adresini öğrenmek için
print &counter
# Değer set etmek için
set var counter=0

# breakpoint'leri görmek için
info breakpoints

# ilk eklenen 1 numaralı breakpoint'i silmek için
delete 1

# Microdenetleyici register adreslerini görmek için
info registers

# Mikrodenetleyiciyi resetlemek için
monitor reset

# debugger'dan çıkmak için
quit
```

![Debugging](./images/MicroBit_04.png)

Denemeye konu olan kod parçamız oldukça basit. Aslında hiçbir şey yapmıyor. Sadece bir sayaç değişkeni tanımlanıyor ve bu sayaç sürekli olarak artırılıyor. Sayaç artırıldıktan sonra yaklaşık 1 saniyelik bir bekleme süresi var. Bekleme süresi boyunca mikrodenetleyici hiçbir işlem yapmaz. Bu sırada debugger ile sayaç değerini gözlemlemek mümkün olur. Upps, peki counter taşma hatası verirse ne olur. O kadar uzun sürede debug yapılmaz. Bu deneysel bir örnek sonuçta. Nasıl debug yaparız görmek için.  

```rust
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
```

### Blinking Led

Bu örnekte 5x5 Led matrisinin tam ortasındaki led ampülünün saniyede bir defa yanıp sönmesi sağlanıyor. Doğrudan mikrodenetleyicinin **GPIO _(General Purpose Input/Output)_** adresleri üzerinde işlem yapılarak ilerleniyor. Bu biraz zorlayıcı zira cihazın donanım şemasına hakim olmak gerekiyor. Ancak bu örnek sayesinde mikrodenetleyicinin temel çalışma prensiplerine dair fikir sahibi olmak pekala mümkün. İşte örnek kodlar;

```rust
#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn start() -> ! {
    
    const GPIO0_BASE: u32 = 0x5000_0000; // GPIO0 modülünün başlangıç adresi
    const PIN_CNF_OFFSET: u32 = 0x700; // GPIO0_PIN_CNF_21 ve GPIO0_PIN_CNF_28 pinlerinin konfigürasyon adresinin başlangıç adresi
    const P0_21: usize = 21; // GPIO0_PIN_CNF_21 pininin numarası
    const P0_28: usize = 28; // GPIO0_PIN_CNF_28 pininin numarası
    const GPIO0_PIN_CNF_21_ROW_1_ADDR: *mut u32 =
        (GPIO0_BASE + PIN_CNF_OFFSET + (P0_21 * 4) as u32) as *mut u32; // GPIO0_PIN_CNF_21 pininin konfigürasyon adresi
    const GPIO0_PIN_CNF_28_COL_1_ADDR: *mut u32 =
        (GPIO0_BASE + PIN_CNF_OFFSET + (P0_28 * 4) as u32) as *mut u32; // GPIO0_PIN_CNF_28 pininin konfigürasyon adresi
    const DIRECTION_OUTPUT_POS: u32 = 0;
    const PIN_CNF_DRIVE_LED: u32 = 1 << DIRECTION_OUTPUT_POS; // GPIO0_PIN_CNF_21 ve GPIO0_PIN_CNF_28 pinlerinin çıkış yönünü belirtir
    // GPIO0_PIN_CNF_21 ve GPIO0_PIN_CNF_28 pinlerini çıkış olarak ayarlar

    unsafe { // Güvenli olmayan kod bloğu
        write_volatile(GPIO0_PIN_CNF_21_ROW_1_ADDR, PIN_CNF_DRIVE_LED); // GPIO0_PIN_CNF_21 pinini çıkış olarak ayarlar
        write_volatile(GPIO0_PIN_CNF_28_COL_1_ADDR, PIN_CNF_DRIVE_LED); // GPIO0_PIN_CNF_28 pinini çıkış olarak ayarlar
    }
    const GPIO0_OUTPUT_ADDRESS: *mut u32 = (GPIO0_BASE + 4) as *mut u32; // GPIO0 modülünün çıkış adresi
    const GPIO0_OUTPUT_ROW_1_POS: u32 = 21; // GPIO0_OUTPUT_ADDRESS adresinde hangi bitin LED'i kontrol ettiğini belirtir
    let mut light_is_on: bool = false;
    loop { 
        unsafe { // Güvenli olmayan kod bloğu
            write_volatile(
                GPIO0_OUTPUT_ADDRESS,
                (light_is_on as u32) << GPIO0_OUTPUT_ROW_1_POS,
            ); // GPIO0_OUTPUT_ADDRESS adresine yazma işlemi yaparak LED'i yakıp söndürür
            for _ in 0..400_000 { // Yaklaşık 1 saniyelik gecikleme süresi
                nop(); // No-operation döngüsü
            }
            light_is_on = !light_is_on;
        }
    }
}
/*
    Bu program doğrudan GPIO0 modülünün bellek adreslerine erişerek ortadaki LED'i kontrol eder ve onu saniyede bir yakıp söndürür.
*/
```

Uygulamayı doğrudan cihaza dağıtmak içinse aşağıdaki komutu kullanmak yeterli.

```bash
cargo embed
```

Beklenen çıktı ortadaki led ışığının saniyede bir yanıp sönmesidir. Her ne kadar burada bir resmini gösteremesem de sözüme güvenebilirsiniz.

### Blinking Led v2

Bir önceki **Blinking Led** örneğindekinden farklı olarak bu örnekte **Hardware Abstraction Layer** kütüphaneleri kullanılmıştır. Bu şekilde soyutlamaları kullanmak pek tabii çok daha pratiktir. İşte bu örneğe ait kodlar.

```rust
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
```

ve dağıtım için,

```bash
cargo embed
```

Beklenen çıktı ortadaki led ışığının yaklaşık 1.5 saniyelik sürelerde yanıp sönmesidir.

### Blinking Rust

Bu örnekte ise **A** düğmesine basıldığında LED matriste sırasyıla **R, U , S ve T** harfleri görünür. Hani eskiden bazı bakkallarda elektrikli panolar olurdu. Üzerlerinde yanıp sönen harflerle reklamlar yapılırdı. Tam olarak o olmasa da bu ufak matris alanında yeterli bir deneme. Tabii burada 5X5 matrisi iyi kodlamak lazım. Matris üzerindeki işleyişi kod tarafında kolaylaştırmak için yardımcı bir veri yapısı kullanmak çok mantıklı olacaktır. Aynen aşağıdaki gibi.

```rust
#[derive(Debug, Clone, Copy)]
pub enum Letter {
    Clear,
    R,
    U,
    S,
    T,
}

pub const fn get_letter(letter: Letter) -> [[u8; 5]; 5] {
    match letter {
        Letter::Clear => [[0; 5]; 5],
        Letter::R => [
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 1, 0, 0],
            [1, 0, 0, 1, 0],
        ],
        Letter::U => [
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 1, 0],
        ],
        Letter::S => [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 1, 1, 0],
        ],
        Letter::T => [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ],
    }
}
```

Burada tüm harfler için hangi led ampüllerinin yanması gerektiği tanımlanıyor. **R** harfi için örneğin ilk satırda ilk üç ampülün yanması gerekirken dördüncü ve beşinci ampüllerin sönük kalması gerekmektedir. Diğer harfler için de benzer şekilde tanımlar yapılmıştır. Şimdi asıl kodlara gelelim.

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod letter_pipe;

use crate::letter_pipe::{Letter, get_letter};
use cortex_m_rt::entry;
use microbit::display::blocking::Display;
use microbit::{board::Board, hal::timer::Timer};
use embedded_hal::digital::InputPin;
use panic_halt as _;

#[entry]
fn start() -> ! {
    let mut board = Board::take().unwrap();

    // Matrisleri kullanabilecek bir Display nesnesi
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    const WAIT: u32 = 500;
    loop {
        // A düğmesine bastığımız sürece RUST yazısını çalıştırıyoruz.
        if let Ok(true) = board.buttons.button_a.is_low() {
            // 500 milisaniye boyunca clear matrisine göre led'ler konumlanacak (Hepsi Kapalı)
            display.show(&mut timer, get_letter(Letter::Clear), WAIT);
            // 500 saniye boyunca R matrisindeki Led'ler yanacak
            display.show(&mut timer, get_letter(Letter::R), WAIT);
            display.show(&mut timer, get_letter(Letter::U), WAIT);
            display.show(&mut timer, get_letter(Letter::S), WAIT);
            display.show(&mut timer, get_letter(Letter::T), WAIT);
        }
        // ve bu böyle sürüp gidecek (Blinky effect)
    }
}
/*
    Program çalıştığında, A düğmesine basılı tutulduğunda RUST yazısı yanıp söner. 
    Aslında LED matriste sırasıyla R, U, S ve T harfleri yanıp söner. 
    Bu örnekte de HAL (Hardware Abstraction Layer) kullanarak LED'leri kontrol ediyoruz.
    `letter_pipe` modülünde harflerin LED matrislerini temsil eden sabit diziler tanımlanmıştır.
*/
```

Artık uygulamayı cihaza dağıtma zamanı.

```bash
cargo embed
```

Beklenen çıktı A düğmesine basıldığında LED ışıklarında RUST kelimesinin harflerinin sıralı bir şekilde görünmesidir.

### Beep

Bu örnekte ise B düğmesine basıldığında denetleyici üzerindeki hoparlörden **beep** benzeri bir ses çıkartılması sağlanmaktadır. İşte rust tarafındaki kodlar.

```rust
#![no_main]
#![no_std]

use panic_halt as _;

use crate::gpio::{p0::P0_00, Output, PushPull};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    hal::{
        gpio,
        prelude::*,
        pwm::{self, Pwm},
        Timer,
    },
    Board,
};

fn play_beep(
    board_pwm: microbit::pac::PWM0,
    speaker_pin: P0_00<Output<PushPull>>,
    board_timer: microbit::pac::TIMER0,
) {
    let pwm = Pwm::new(board_pwm);
    pwm.set_output_pin(pwm::Channel::C0, speaker_pin.degrade());
    pwm.set_prescaler(pwm::Prescaler::Div1);
    pwm.set_counter_mode(pwm::CounterMode::UpAndDown);
    pwm.set_max_duty(32767);
    pwm.set_period(432u32.hz());
    pwm.set_duty_on_common(32767 / 2);
    pwm.enable();

    let mut timer = Timer::new(board_timer);
    timer.delay_ms(500u32);

    pwm.disable();
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::High);
    
    play_beep(board.PWM0, speaker_pin, board.TIMER0);

    loop {
        cortex_m::asm::wfi();
    }
}

/*
    Bu program Micro:bit V2 hoparlöründen 432 Hz frekansında ses çalmak için
    bir PWM sinyali oluşturur (Beep sesine benzer bir ses).
    Hoparlör, belirli bir süre boyunca (500 ms) aktif kalır ve ardından
    PWM devre dışı bırakılır.

    Kabaca aşağıdaki gibi bir kare sinyal oluşturur:

       HIGH ____      ____      ____
        |   |    |   |    |   |
        |   |    |   |    |   |
   LOW  |___|____|___|____|___|____
          <----> <----> <---->
           periyot aralığı 1/432 saniye


    PWM, Pulse Width Modulation anlamına gelir ve genellikle
    analog sinyalleri dijital sinyallere dönüştürmek için
    kullanılır. Bir sinyalin belirli bir süre boyunca
    açık kalma süresini (duty cycle) kontrol ederek
    ortalama bir voltaj değeri oluşturur. Bu değer hoparlör
    gibi cihazların ses çıkışını kontrol etmek için
    kullanılabilir. Hatta bir LED parlaklığını kontrol etmek için
    de kullanılabilir. PWM, genellikle bir mikrodenetleyici
    üzerinde bir zamanlayıcı (timer) kullanılarak
    gerçekleştirilir. Bu zamanlayıcı, belirli bir frekansta
    (örneğin 432 Hz) bir sinyal oluşturur ve bu sinyalin
    açık kalma süresini (duty cycle) ayarlayarak
    ortalama bir voltaj değeri oluşturur. Bu voltaj değeri,
    hoparlör gibi bir yük üzerinde uygulandığında, yükün
    ortalama bir voltaj değeri almasını sağlar. Bu, hoparlörün
    ses çıkarmasını sağlar. Örneğin, %50 duty cycle
    kullanıldığında, sinyalin yarısı açık ve yarısı kapalıdır.
    Bu, hoparlörün ortalama bir voltaj değeri alarak ses çıkarmasını
    sağlar.
*/
```

Her zaman olduğu gibi bu uygulama kodlarını da cihaza deploye etemiz lazım ki kullanabilelim.

```bash
cargo embed
```

Beklenen çıktı, B düğmesine basıldığında beep sesi duyulmasıdır.

### Uart

**UART _(Universal Asynchronous Receiver/Transmitter)_** mikrodenetleyici üzerinde yer alan bir çevresel iletişim birimidir _(Peripheral)_. Bu arabirimi kullanarak mikrodenetleyici ve bilgisayar arasında haberleşme sağlanabilir. Modül **Transmitter** ve **Receiver** için pin'lere sahiptir. Örneğin bilgisayarın **COM** portuna seri haberleşme protokolü üzerinden mesaj gönderilebilir veya bilgisayardan dönen mesaj okunabilir. İşte örnek kodlarımız;

```rust
#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use microbit::{
    hal::uarte::{Baudrate, Parity, Uarte},
    Board,
};
use rtt_target::rprintln;
use rtt_target::rtt_init_print;

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

    // let message = "Hello from Micro:bit!\r\n"; // Gönderilecek ifadeyi byte array olarak tanımlıyoruz.

    loop {
        // Her bir byte için döngü başlatıyoruz.
        // for byte in message.iter() {
        //     // UART modülü üzerinden her bir byte'ı gönderiyoruz.
        //     uart.write(&[*byte]).unwrap();
        // }
        write!(uart, "Hello there!\r\n").unwrap();

        // delay değerini hesaplamak için mikrodenetleyicinin saat hızını ele almak yöntemlerden birisi.
        // delay = clock_speed * time
        // Micro:bit V2.2 kartı spesifikasyonlarına göre ARM Cortex işlemcisi (nRF52833)
        // 64 MHz hızında çalışmakta ama varsayılan olarak 16 Mhz değerini kullanmakta.
        // Detaylar için; (https://docs.nordicsemi.com/bundle/ps_nrf52833/page/keyfeatures_html5.html)

        // 16 MHz hızında çalışan bir sistemi 5 saniye beklemek için
        // 16_000_000 * 2 = 32_000_000 değeri kullanılabilir
        cortex_m::asm::delay(32_000_000); // Yaklaşık 2 saniye bekleme süresi

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
```

Gerekli terminal komutları,

```bash
# Microbit bilgisayar USB kablosu ile bağlandıktan sonra
# mode komutu ile COM port bilgileri alınabilir.
mode

# Flashing için
cargo embed
```

COM portuna gelen mesajları görmek için **PuTTY** uygulamasından yararlanılabilir. Uygulama ayarları aşağıdaki gibi yapılandırılmalıdır.

![PuTTY Session](./images/MicroBit_06.png)

![PuTTY](./images/MicroBit_05.png)

![UART Runtime](./images/MicroBit_07.png)

### Server

Bu örnekte mikrodenetleyici bir server rolünü üstlenir. Bilgisayar terminalinden gönderilen komutlara göre farklı işlemler yapar. Komut gönderimi için **uart** isimli örnekte olduğu gibi **PuTTY** ile **COM3** portuna bağlanılan bir terminal açılır. Terminalde **h**, **o** ve **r** gibi tek karakterler gönderildiğinde mikrodenetleyici bu komutlara karşılık bazı işlemler yapar. Örneğin **o** harfi tuşlandığında **LED matrix**'te gülen surat yanar, **r** harfine basılırsa LED resetlenir, **h** harfi ile yardım menüsü okunur. Sunucu tarafındaki kodları şöyle geliştirebiliriz.

```rust
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
```

Uygulamayı cihaza dağıtmak içinse aşağıdaki komutu kullanmak yeterli.

```bash
cargo embed
```

Beklenen çalışma zamanı çıktısı;

![Server runtime](./images/MicroBit_08.png)

### Accelerometer

Bu örnekte microdenetleyici üzerinde yer alan ivme ölçer'den _(Accelerometer)_ anlık **x,y,z** değerlerinin okunması ve bu bilgilerden yararlanarak hareket hızının ölçülmesi deneniyor. Bu örnekte işin zorlu kısmı kalibrasyon. Gelen donanım üzerindeki ivme ölçer fabrika ayarlarına göre çıkar. Bu durumda cihazın hareketi sırasında ölçülen ivme değerleri gerçek hareketi tam olarak yansıtmayabilir. Bu nedenle kalibrasyon yaparak cihazın hareketini daha doğru bir şekilde ölçmek mümkün olabilir. Kalibrasyon işlemi için de bazı yardımcı fonksiyonlar gerekiyor. Örneğin karekök bulma. Normalde std kütüphanede yer alıyor elbette ama bare-metal programlama yaparken standart kütüphane kullanmak mümkün değil. O yüzden kendi karekök alma fonksiyonumuzu yazacağız. Ben bunun için **Newton Rapson** metodunu tercih ettim.

```rust
// Aşağıdaki fonksiyon Newton-Raphson yöntemini kullanarak karekök hesaplaması yapar
// Normalde standart kütüphanelerde bulunan sqrt fonksiyonu kullanılabilir
// Ancak bu örnekte standart kütüphane kullanılmadığından sqrt, powi gibi fonksiyonları kullanamıyoruz.
pub fn sqrt(value: f32) -> f32 {
    if value <= 0.0 {
        return 0.0;
    }
    let mut guess = value;
    for _ in 0..10 {
        guess = 0.5 * (guess + value / guess);
    }
    guess
}
```

Kalibrasyon işlemini başlatacak bir diğer fonksiyon da aşağıdaki gibi yazılabilir. 100 ölçüm değerine baz olarak bir Bias değeri bulmaya çalışıyoruz. Bias bir nevi standart sapmayı gösteriyor.

```rust
use lsm303agr::{Lsm303agr, interface::I2cInterface, mode::MagOneShot};
use microbit::{hal::twim::Twim, pac::TWIM0};
use rtt_target::rprintln;

pub struct Bias {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Bias {
    pub fn init(sensor: &mut Lsm303agr<I2cInterface<Twim<TWIM0>>, MagOneShot>) -> Self {
        /*
            Buradan itibaren kalibrasyon yapmaya çalışıyoruz.
            Öncelikle 100 adet ivmeölçer verisi topluyoruz.
            Bu verilerden x, y, z eksenlerindeki ivme değerlerinin ortalamasını alıyoruz ve bias ile başlayan değişkenlerde topluyoruz.
            Amacımız birazdan ivmeölçer verilerini filtrelemek ve gürültüden arındırmak.
            Zira durduğu yerde dahi yerçekimi değerine bağlı olarak veri üretilecektir.
        */
        let calib_samples: usize = 100; // Ortalama hesaplamaları için alınacak örnek sayısı
        let mut sum_x = 0;
        let mut sum_y = 0;
        let mut sum_z = 0;
        let mut collected = 0;
        rprintln!("Calibrating...");
        while collected < calib_samples {
            if sensor.accel_status().unwrap().xyz_new_data() {
                let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
                sum_x += x;
                sum_y += y;
                sum_z += z;
                collected += 1;
            }
        }

        rprintln!("Calibration done!");
        rprintln!("Bias: x: {}, y: {}, z: {}", sum_x, sum_y, sum_z);
        Bias {
            x: sum_x / calib_samples as i32,
            y: sum_y / calib_samples as i32,
            z: sum_z / calib_samples as i32,
        }
    }
}
```

Ana program kodumuz ise şöyle,

```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use math::sqrt;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embedded_hal::digital::InputPin;
use microbit::{
    hal::{Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

mod bias;
mod math;

use bias::Bias;

/*
    Bu örnekte micro:bit kartı üzerinde bulunan LSM303AGR ivmeölçer (Accelerometer) kullanılarak hız hesaplaması yapılmaktadır.
    LSM303AGR bileşeni x,y,z eksenlerinde ivme ölçümü yapabilen bir sensördür.
    Örnekte ivmeölçeri 50Hz örnekleme hızı ile çalıştırıyoruz ki bu değerin büyüklüğü ivmeölçerin hassasiyetini etkiler.
    Örnekleme hızı arttıkça ivmeölçerin hassasiyeti artar ancak işlemci üzerindeki yük de buna bağlı olarak artar.
    İvmeölçer eğer A düğmesine basıldıysa her 20ms'de bir ivme verilerini okur ve bu verileri kullanarak hız hesaplaması yapar.
    Hız hesaplaması, eksen bazlı ivme değerlerinin zamanla çarpımına bakılmak suretiyle yapılır.
    Olası sapmaların önüne geçmek için belli sayıda ivme verisi ölçülür ortalamaları alınır ve bu ortalamalar
    ivmeölçer verilerinden çıkarılır. Bu sayede ivmeölçer verileri filtrelenmiş olur.
    Ayrıca gürültü filtrelemesi de yapılır. Eğer ivme değerleri belirli bir eşiğin altındaysa bu değerler sıfırlanır.
*/

const NOISE_THRESHOLD: f32 = 0.05; // Gürültü eşiği
const MILL_G: f32 = 9.80665; // 1g = 9.80665 m/s^2, 1g = 1000mg (Yerçekimi ivmesinin milli-g cinsinden değeri)
const DELTA_TIME: f32 = 0.02; // 50Hz örnekleme → 20ms

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = microbit::Board::take().unwrap(); // Micro:bit kartının sahipliğini alıyoruz

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) }; // i2c haberleşmesi için gerekli olan bileşenleri başlatıyoruz.
    let mut timer0 = Timer::new(board.TIMER0); // Ölçümler için bir zamanlayıcı gerekiyor
    let mut sensor = Lsm303agr::new_with_i2c(i2c); // LSM303AGR ivmeölçer sensörünü hazırlıyoruz.

    sensor.init().unwrap(); // Sensörü başlatıyoruz.

    rprintln!("Sensor initialized successfully!");

    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap(); // Sensörü 50Hz örnekleme hızı ile çalışacak şekilde ayarlıyoruz
    let bias = Bias::init(&mut sensor);
    let (mut vx, mut vy, mut vz) = (0.0_f32, 0.0_f32, 0.0_f32);

    loop {
        if let Ok(true) = board.buttons.button_a.is_low() {
            // Eğer A butonuna basılırsa
            if sensor.accel_status().unwrap().xyz_new_data() {
                // ve ivmeölçer yeni veri ürettiyse
                let (x, y, z) = sensor.acceleration().unwrap().xyz_mg(); // bu verileri oku

                let (raw_x, raw_y, raw_z) = (x - bias.x, y - bias.y, z - bias.z); // sapma değerlerine göre ham x,y,z değerlerini hesapla

                let (mut ax, mut ay, mut az) = (
                    raw_x as f32 * MILL_G / 1000.0,
                    raw_y as f32 * MILL_G / 1000.0,
                    raw_z as f32 * MILL_G / 1000.0,
                ); // yerçekimi ivmesini de hesaba katarak ivme değerlerini hesapla

                // Gürültü filtreleme yapılan yer.
                if ax.abs() < NOISE_THRESHOLD {
                    ax = 0.0;
                }
                if ay.abs() < NOISE_THRESHOLD {
                    ay = 0.0;
                }
                if az.abs() < NOISE_THRESHOLD {
                    az = 0.0;
                }
                // İvme değerlerini zamanla çarparak x,y,z eksenlerindeki hızları hesaplanıyor
                vx += ax * DELTA_TIME;
                vy += ay * DELTA_TIME;
                vz += az * DELTA_TIME;

                let speed = sqrt(vx * vx + vy * vy + vz * vz); // Scalar hız değerini buluyoruz

                rprintln!("Current Speed : {:.2} m/s", speed);
                rprintln!("Hız (m/s): x: {:.2}, y: {:.2}, z: {:.2}", vx, vy, vz);
            }
        }
    }
}
```

ve pek tabii uygulamayı cihaza dağıtmamız lazım.

```bash
cargo embed
```

Beklenen çıktıya göre A düğmesine basılı iken terminal ekranına ivme artışlarının ve yaklaşık bir hız değerinin yazılması beklenir.

![Speed](./images/MicroBit_09.png)

### Thermometer V1

Bu örnek micro:bit denetleyicisi üzerindeki **LSM303AGR** çipinin sıcaklığını ölçümler. Rust kodlarını aşağıdaki gibi geliştirebiliriz.

```rust
#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;
use lsm303agr::Lsm303agr;
use microbit::{
    board::Board,
    hal::{twim, Timer},
    pac::twim0::frequency::FREQUENCY_A,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap(); // Board nesnesinin sahipliği alınır
    let i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut timer = Timer::new(board.TIMER0); // Zamanlayıcı modülü başlatılır

    /*
        İletişim I2C protokolü ile yapılır
        TWIM0, micro:bit kartındaki I2C modülüdür.
        I2C modülü, mikrodenetleyici ile diğer bileşenler arasında veri iletimi sağlar.
        Frekans değeri 100 kHz olarak ayarlanmıştır.
        Bu değer, I2C iletişim hızını belirler.
    */

    let mut lsm303 = Lsm303agr::new_with_i2c(i2c); // LSM303 çipi I2C protokolünü kullanacak şekilde örneklenir

    lsm303.init().unwrap();
    lsm303
        .set_accel_mode_and_odr(
            &mut timer,
            lsm303agr::AccelMode::Normal,
            lsm303agr::AccelOutputDataRate::Hz1,
        )
        .unwrap();

    loop {
        if let Ok(true) = board.buttons.button_a.is_low() {
            // Eğer A butonuna basılmışsa
            // Sıcaklık ölçümü başlatılır
            let status = lsm303.temperature_status().unwrap(); // Sıcaklık durumu kontrol edilir
            let celcius = lsm303.temperature().unwrap().degrees_celsius(); // Celsius cinsinden sıcaklık değeri alınır
            let fahrenheit = celcius * 9.0 / 5.0 + 32.0; // F = C * 9/5 + 32 formülü ile hesaplanır.

            if status.overrun() {
                rprintln!("Overrun...");
            }
            if status.new_data() {
                rprintln!(
                    "Temperature of LSM303AGR Chip: {}°C, {} F",
                    celcius,
                    fahrenheit
                );
            }
            timer.delay_ms(1_000);
        }
    }
}

/*
    Bu örnek kod parçası ile micro:bit denetleyicisi üzerindeki
    LSM303AGR çipinin ısı verilerini okuma işlemi gerçekleştirilmektedir.
    Sıcaklık ölçümü için LSM303 çipinin sıcaklık sensörü kullanılmaktadır.
    İşlem kullanıcı A butonuna bastığında başlatılmaktadır.
    Kullanıcı A butonuna basıldığında, LSM303 çipinin sıcaklık durumu kontrol edilir.
    Eğer sıcaklık durumu "overrun" ise, bu durum ekrana yazdırılır.
    Overrun, ölçüm verilerinin kaybolduğunu gösterir.
    Sonrasında sıcaklık değeri Celsius ve Fahrenheit cinsinden hesaplanır ve ekrana yazdırılır.
*/
```

ve uygulamayı cihaza dağıtmak için aşağıdaki komutu kullanmak yeterli.

```bash
cargo embed
```

Beklenen çıktı mikrodenetleyici üzerindeki A butonuna basıldığında sıcaklık değerinin santigrat cinsidinden bilgisayar terminalinde görülmesidir.

![Thermometer v1 runtime](./images/MicroBit_11.png)

### Thermo2

Bu örnekte ise bir öncekinden farklı olarak ortam sıcaklığını ölçmeye çalışıyoruz. Bunun için üzerinde ısı sensörü bulunan **MonkMakes** marka board kullanılıyor. Her iki denetleyiciyi birbirlerine aşağıdaki şekilde görüldüğü gibi bağlamamız gerekiyor.

![Sensor Board](./images/MicroBit_10.jpg)

Board ve micro:bit arasındaki bağlantıları aşağıdaki tablo ile özetleyebiliriz.

| Timsah Klips | Micro:bit | Sensor |
|--------------|-----------|--------|
| Siyah        | GND       | GND    |
| Kırmızı      | 3V        | 3V     |
| Sarı         | GPIO 1    | Isı    |

```text
        [ MonkMakes Sensor ]
                 |
      +----------+----------+
      |          |          |
     GND        3V        Isı (OUT)
      |          |          |
      |          |          |
      |          |          |
    [GND]       [3V]     [GPIO1]
      |          |          |
      +----------+----------+
      [    micro:bit v2.2   ]
      |          |          |
    Siyah     Kırmızı      Sarı
```

Uygulama kodlarımız şöyle

```rust
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

/*
    MonkMakes sıcaklık sensörünün https://monkmakes.com/mb_2a adresinde
    yayınlanan teknik dokümanda kalibrasyon formülü verilmiştir.
    Aşağıdaki sabit değerler buradan alınmıştır.
*/
const A: f32 = 18.0;
const B: f32 = 115.0;
const C: f32 = -54.0;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Thermo V2 is starting...");

    let peri = Peripherals::take().unwrap(); // Tüm donanım bileşenlerini kontrol edebileceğim PAC nesnesi oluşturuluyor
    let saadc = &peri.SAADC; // SAADC modülünün referansına erişim sağlanıyor

    // SAADC modülünü başlatıyoruz
    // write metodu ile enable bitini 1 yapıyoruz
    // enable bitinin 1 olması SAADC modülünün aktif olduğunu gösterir
    saadc.enable.write(|w| w.enable().enabled());

    // SAADC modülünün çalışması için gerekli olan yapılandırma ayarları
    saadc.ch[0].config.write(|w| {
        w.resp().bypass(); // Direnç bölücü bypass ediliyor
        w.gain().gain1_6(); // 1/6 kazanç. Bunun anlamı, ADC'nin giriş voltajını 6 kat artırmasıdır.
        w.refsel().internal(); // Dahili referans voltajı kullanılacağı belirtiliyor
        w.tacq()._10us(); // 10 mikro saniye örnekleme süresi belirleniyor
        w.mode().se(); // GND referanslı tek uçlu ölçüm modu seçiliyor
        w.burst().disabled(); // Burst mod devre dışı bırakılıyor. Burst modu, ardışık ölçümler için kullanılır.
        w // Oluşan nesnei döndürüyoruz
    });

    saadc.ch[0].pselp.write(|w| w.pselp().analog_input0());
    // GND referanslı tek uçlu ölçüm için analog giriş 0 seçiliyor
    // Dolayısıyla, sıcaklık sensörünün OUT çıkışı AIN0 pinine bağlanmış olmalı.

    // Ölçüm sonucu için RAM'de bir adres ayırıyoruz
    static mut RESULT: i16 = 0;
    let result_ptr: *mut i16 = &raw mut RESULT;
    /*
        RAM'de ayırdığımız adresi SAADC modülüne tanıtıyoruz
        result_ptr adresini 32 bitlik bir işaretçi olarak yazıyoruz
        ptr() metodu ile işaretçi adresini alıyoruz
        bits() metodu ile 32 bitlik bir değer yazıyoruz
        Bu sayede SAADC modülü, ölçüm sonuçlarını bu adrese yazıyor.
    */
    saadc
        .result
        .ptr
        .write(|w| unsafe { w.ptr().bits(result_ptr as u32) });

    // Ölçüm sonuçlarının 1 adet olacağını belirtiyoruz
    unsafe {
        saadc.result.maxcnt.write(|w| w.bits(1));
    }
    let temp_reg = peri.TEMP; // TEMP modülünün referansına erişim sağlanıyor

    loop {
        saadc.tasks_start.write(|w| unsafe { w.bits(1) });
        while saadc.events_started.read().bits() == 0 {}
        saadc.events_started.reset();

        saadc.tasks_sample.write(|w| unsafe { w.bits(1) });
        while saadc.events_end.read().bits() == 0 {}
        saadc.events_end.reset();

        /*
            Aşağıdaki kısımda TEMP modülünden sıcaklık ölçümü yapılıyor
            TEMP modülü doğrudan mikrodenetleyicinin sıcaklığını ölçer. Çok doğru sonuçlar vermeyebilir.
            Ancak, sıcaklık sensörünün voltajını ölçmek için kullanılabilir.
            Bu sayede kalibrasyon için gerekli regresyon analizine kaynak veriler toplanabilir
        */
        temp_reg.tasks_start.write(|w| unsafe { w.bits(1) });
        while temp_reg.events_datardy.read().bits() == 0 {}
        temp_reg.events_datardy.reset();
        let chip_temp = temp_reg.temp.read().bits() / 4;

        let adc_raw = unsafe { RESULT };

        /*
            ADC değerinden voltaj (mV) ve sıcaklık (°C) hesaplama
            Formül : V = ADC * (Vref / ADCmax)
            Vref = 0.6V (Dahili referans voltajı)
            ADCmax = 1024 (10 bit çözünürlük)
        */
        let voltage_mv = adc_raw as f32 * 0.6 * 6.0 * 1000.0 / 1024.0;
        let temp_c_default = voltage_mv / 10.0;

        // Kalibrasyon formülü MonkMakes sitesinden alınmıştır.
        let calibrated_temp = (A / B) * (adc_raw as f32) + C;

        rprintln!(
            "Chip Temp: {} °C | Sensor Voltage: {:.2} mV | Raw Temp: {:.1} °C | Calibrated Temp: {:.1} °C",
            chip_temp,
            voltage_mv,
            temp_c_default,
            calibrated_temp
        );

        for _ in 0..800_000 {
            // Yaklaşık 2 saniyelik gecikleme süresi
            nop(); // No-operation döngüsü
        }
    }
}

/*
    Bu örnek, NRF52833 mikrodenetleyicisi üzerindeki SAADC (Successive Approximation Analog-to-Digital Converter) modülünü kullanarak
    analog bir sıcaklık sensöründen veri okuma işlemini ele almaktadır. Harici sensör olarak MonkMakes kullanılır.
    Sensör verileri OUT çıkışından analog voltaj olarak alınır ve bu voltaj değeri mikro:bit üstündeki ADC modülü tarafından dijital verilere dönüştürülür.
    SAADC, analog sinyalleri dijital verilere dönüştürmek için kullanılır ve bu örnekte sıcaklık ölçümü için yapılandırılmıştır.
    Ölçüm sonuçları, belirli bir formatta (mV ve °C) hesaplanır ve RTT (Real-Time Transfer) kullanılarak bilgisayar konsoluna yazdırılır.

    Uygulamada PAC (Peripheral Access Crate) kullanılarak mikrodenetleyicinin donanım bileşenlerine doğrudan erişim sağlanır.
*/
```

test etmek için cihaza dağıtmak yeterli.

```bash
cargo embed
```

Beklenen çıktı aşağıdakine benzerdir. Test şöyle yapılmıştır. Kartlar bilgisayarın fanına yakın bir yere yerleştirilmiştir. Fana yakın olan MonkMakes sensörünün sıcaklık değerleri zamanla artış göstermiştir.

![External Sensor Temperature](./images/MicroBit_12.png)

Sensörler oda ortamında aynı noktaya yerleştirildiklerinde ise sıcaklık değerleri yakınsamıştır.

![External and Micro Sensor Temperatures](./images/MicroBit_13.png)

### Funny Led

Bu örnekte amaç kare, kalp, yukarı ok ve aşağı ok şekillerini LED matriste HAL kütüphanelerini kullanarak göstermektir. LED matis üzerindeki ampüllerle şekilleri daha kolay eşleştirmek için aşağıdaki veri yapısı kullanılır.

```rust
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Square,
    Hearth,
    UpArrow,
    DownArrow,
    X,
    Checkmark,
    Forbidden,
}

pub const fn get(shape: Shape) -> [[u8; 5]; 5] {
    match shape {
        Shape::Square => [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
        Shape::Hearth => [
            [0, 1, 0, 1, 0],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
        ],
        Shape::UpArrow => [
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
            [1, 0, 1, 0, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ],
        Shape::DownArrow => [
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 0, 1, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
        ],
        Shape::X => [
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ],
        Shape::Checkmark => [
            [0, 0, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [1, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
        Shape::Forbidden => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 1, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ],
    }
}
```

Diğer yandan LED matrisini kontrol etmek içinde aşağıdaki kodları kullanabiliriz.

```rust
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{
    gpio::{Level, Output, Pin, PushPull, p0, p1},
    pac,
    timer::Timer,
};
/*
    Aşağıdaki modülü LED matrisini kontrol etmek için kullanıyoruz.
    Amaçımız, LED matrisini kontrol etmek ve belirli şekilleri göstermek.
*/

pub struct LedMatrix {
    pub rows: [Pin<Output<PushPull>>; 5], // 5 satır pini
    pub cols: [Pin<Output<PushPull>>; 5], // 5 sütun pini
    pub timer: Timer<pac::TIMER0>,        // zamanlayıcı
}

impl LedMatrix {
    pub fn new() -> Self {
        // NRF52833 mikrodenetleyicisinin çevresel birimlerini alıyoruz.
        let p = pac::Peripherals::take().unwrap();

        let p0_parts = p0::Parts::new(p.P0); // P0 portunu alıyoruz.
        let p1_parts = p1::Parts::new(p.P1); // P1 portunu alıyoruz.

        let row_pins: [Pin<Output<PushPull>>; 5] = [
            p0_parts.p0_21.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_22.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_15.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_24.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_19.into_push_pull_output(Level::Low).degrade(),
        ]; // 5 satır pini oluşturuyoruz.

        let col_pins: [Pin<Output<PushPull>>; 5] = [
            p0_parts.p0_28.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_11.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_31.into_push_pull_output(Level::High).degrade(),
            p1_parts.p1_05.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_30.into_push_pull_output(Level::High).degrade(),
        ]; // 5 sütun pini oluşturuyoruz.

        LedMatrix {
            timer: Timer::new(p.TIMER0),
            rows: row_pins,
            cols: col_pins,
        }
    }

    // Bu fonksiyon, LED matrisinin tüm LED'lerini kapatır.
    // Bunun için her satır pini LOW, her sütun pini HIGH olacak şekilde ayarlanır.
    pub fn clear_all(&mut self) {
        for r in self.rows.iter_mut() {
            r.set_low().ok();
        }
        for c in self.cols.iter_mut() {
            c.set_high().ok();
        }
    }

    /*
        Aşağıdaki fonksiyon, LED matrisine bir şekil çizer.
        Bu şekil, 5x5 boyutunda bir dizi olarak temsil edilir.
        Her bir eleman 0 veya 1 değerini alabilir. 1 değeri LED'in yanmasını, 0 değeri ise sönmesini temsil eder.
        Fonksiyon, şekli çizmek için her bir satır ve sütun pinini sırayla LOW ve HIGH yapar.

        Işıkların göze doğru görünmesi için frame_count kadar döngü yapılır.
    */

    pub fn draw(&mut self, shape: [[u8; 5]; 5], duration_ms: u32) {
        let frame_count = duration_ms / 5; // Her bir çerçeve için 5 ms bekleyeceğiz.
        // frame_count kadar döngü yapıyoruz.

        for _ in 0..frame_count {
            // Satır bazında döngü başlatıyoruz.
            for row in 0..5 {
                // Öncelikle tüm satır pinlerini LOW yapıyoruz.
                // Bu, tüm LED'lerin sönmesini sağlar.
                for r in self.rows.iter_mut() {
                    r.set_low().ok();
                }

                // Şimdi, sadece şu anki satır pinini HIGH yapıyoruz.
                // Bu, o satırdaki LED'lerin yanmasını sağlar.
                self.rows[row].set_high().ok();

                // Ardından sütun pinlerini dolaşıyoruz
                for col in 0..5 {
                    // Eğer şekil dizisindeki değer 1 ise, o sütun pinini LOW yapıyoruz.
                    // Bu, o sütundaki LED'in yanık kalmasını sağlar.
                    if shape[row][col] == 1 {
                        self.cols[col].set_low().ok();
                    } else {
                        self.cols[col].set_high().ok();
                    }
                }

                // Burada mikrosaniye zamanlayıcısını kullanarak 500 mikro saniye bekliyoruz.
                // Bu, LED'lerin yanık kalma süresini kontrol eder.
                // ve böylece parlaklık hataları oluşmaz.
                self.timer.delay_us(500);

                // Geri kalan tüm satır pinlerini LOW yapıyoruz ve böylece LED'ler sönüyor.
                for c in self.cols.iter_mut() {
                    c.set_high().ok();
                }
            }
        }

        self.clear_all(); // Döngü dışına geldiğimizde ise tüm LED'leri kapatıyoruz.
    }
}
```

Nihayetinde main içeriğini de aşağıdaki gibi oluşturabiliriz.

```rust
#![no_main]
#![no_std]

mod led;
mod shape;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use led::LedMatrix;
use panic_halt as _;
use shape::*;

#[entry]
fn main() -> ! {
    let mut led_matrix = LedMatrix::new();
    let shapes = [
        get(Shape::Square),
        get(Shape::Hearth),
        get(Shape::UpArrow),
        get(Shape::DownArrow),
        get(Shape::X),
        get(Shape::Checkmark),
        get(Shape::Forbidden)
    ];
    let mut current_shape = 0;

    loop {
        led_matrix.clear_all();
        led_matrix.draw(shapes[current_shape], 5000);
        led_matrix.timer.delay_ms(500);
        current_shape = (current_shape + 1) % shapes.len();
    }
}
```

Buraya kadar gelebildiysek hemen sonuçları deneyebiliriz.

```bash
cargo embed
```

Beklenen çıktı, LED matris ekranında sırayla kare, kalp, yukarı ok, aşağı ok, X, onay işareti ve yasak işareti şekillerinin görünmesidir. Her şekil 5 saniye boyunca ekranda kalır ve ardından diğer şekle geçilir.

### Thermo Digits

Thermo2 uygulamasının farklı bir versiyonunu deneyebiliriz. Isı değerleri LED matris ekranında kayan sayılar şeklinde gösterilir. Bu sefer biraz daha fazla efora ihtiyacımı olacak. Kullanacağımız sabit değerler ile başlayalım.

```rust
pub const A: f32 = 18.0;
pub const B: f32 = 115.0;
pub const C: f32 = -54.0;
pub const SCROLL_WIDTH: usize = 4 * 4;
pub const DELAY_FACTOR: usize = 400_000;
pub const LED_ROW_LENGTH: usize = 5;
pub const LED_COL_LENGTH: usize = 5;
pub const DRAW_DELAY: u32 = 500;
pub const LIGHT_ON: u8 = 1;
pub const FRAME_FACTOR: u32 = 5;
pub const TOTAL_TEXT_COLUMN: usize = 64;
```

Pek tabii LED matrisinde sayıları ve nokta karakterini göstermek için de yardımcı bir veri yapısı iyi olur.

```rust
/*
    5X5 Led matrisinde 0-9 rakamlarını ve nokta karakterini temsil eden sabit diziler.
    Her rakam ve nokta karakteri, 5 satır ve 3 sütundan oluşan bir dizi ile temsil edilir.
    Her satırda 1, 0 değerleri ile LED'lerin açık (1) veya kapalı (0) olduğu belirtilir.
    1: LED açık
    0: LED kapalı

    Ayrıca virgüllü sayılar için nokta karakteri de kullanılır.
*/
pub const DIGITS: [[[u8; 3]; 5]; 10] = [
    // 0
    [[1, 1, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 1, 1]],
    // 1
    [[0, 1, 0], [1, 1, 0], [0, 1, 0], [0, 1, 0], [1, 1, 1]],
    // 2
    [[1, 1, 1], [0, 0, 1], [1, 1, 1], [1, 0, 0], [1, 1, 1]],
    // 3
    [[1, 1, 1], [0, 0, 1], [0, 1, 1], [0, 0, 1], [1, 1, 1]],
    // 4
    [[1, 0, 1], [1, 0, 1], [1, 1, 1], [0, 0, 1], [0, 0, 1]],
    // 5
    [[1, 1, 1], [1, 0, 0], [1, 1, 1], [0, 0, 1], [1, 1, 1]],
    // 6
    [[1, 1, 1], [1, 0, 0], [1, 1, 1], [1, 0, 1], [1, 1, 1]],
    // 7
    [[1, 1, 1], [0, 0, 1], [0, 1, 0], [1, 0, 0], [1, 0, 0]],
    // 8
    [[1, 1, 1], [1, 0, 1], [1, 1, 1], [1, 0, 1], [1, 1, 1]],
    // 9
    [[1, 1, 1], [1, 0, 1], [1, 1, 1], [0, 0, 1], [1, 1, 1]],
];

pub const DOT: [[u8; 1]; 5] = [[0], [0], [0], [0], [1]];
```

Bu enstrümanları LED matrisini yöneteceğimiz aşağıdaki veri yapısından ele alabiliriz.

```rust
use crate::constants::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{
    gpio::{Level, Output, Pin, PushPull, p0, p1},
    pac,
    timer::Timer,
};

pub struct LedMatrix {
    pub rows: [Pin<Output<PushPull>>; LED_ROW_LENGTH],
    pub cols: [Pin<Output<PushPull>>; LED_COL_LENGTH],
    pub timer: Timer<pac::TIMER0>,
}

impl LedMatrix {
    pub fn new(p0: pac::P0, p1: pac::P1, timer: pac::TIMER0) -> Self {
        let p0_parts = p0::Parts::new(p0);
        let p1_parts = p1::Parts::new(p1);

        let row_pins: [Pin<Output<PushPull>>; LED_ROW_LENGTH] = [
            p0_parts.p0_21.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_22.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_15.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_24.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_19.into_push_pull_output(Level::Low).degrade(),
        ];

        let col_pins: [Pin<Output<PushPull>>; LED_COL_LENGTH] = [
            p0_parts.p0_28.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_11.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_31.into_push_pull_output(Level::High).degrade(),
            p1_parts.p1_05.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_30.into_push_pull_output(Level::High).degrade(),
        ];

        LedMatrix {
            timer: Timer::new(timer),
            rows: row_pins,
            cols: col_pins,
        }
    }

    pub fn clear_all(&mut self) {
        for r in self.rows.iter_mut() {
            r.set_low().ok();
        }
        for c in self.cols.iter_mut() {
            c.set_high().ok();
        }
    }

    pub fn draw(&mut self, digit: [[u8; LED_ROW_LENGTH]; LED_COL_LENGTH], duration_ms: u32) {
        let frame_count = duration_ms / FRAME_FACTOR;

        for _ in 0..frame_count {
            for (row, _) in digit.iter().enumerate().take(LED_ROW_LENGTH) {
                for r in self.rows.iter_mut() {
                    r.set_low().ok();
                }

                self.rows[row].set_high().ok();

                for col in 0..LED_COL_LENGTH {
                    if digit[row][col] == LIGHT_ON {
                        self.cols[col].set_low().ok();
                    } else {
                        self.cols[col].set_high().ok();
                    }
                }

                self.timer.delay_us(DRAW_DELAY);

                for c in self.cols.iter_mut() {
                    c.set_high().ok();
                }
            }
        }

        self.clear_all();
    }
}
```

İşin zorlayıcı kısmı metinsel ifadeyi LED matrisine uygun hale getirmek. Bunu da utiliy modülünde aşağıdaki kodlarla inşa edebiliriz.

```rust
use crate::constants::{LED_ROW_LENGTH, TOTAL_TEXT_COLUMN};
use crate::led_matrix::LedMatrix;
use crate::panel::*;
/*
    LED Matrisinde ısı değerlerini sağdan sola kaydırarak göstermek için
    kullanılan yardımcı fonksiyonlar.

    text_to_bitmap: Verilen metni 5x5'lik bir bitmap dizisine dönüştürür.
    scroll_text: Bitmap dizisini LED matrisinde kaydırarak gösterir.
    float_to_ascii: Float türünde bir sayıyı(örneği 18.7 gibi bir ısı değerini)
    ASCII karakter dizisine dönüştürür.
*/

pub fn text_to_bitmap(text: &str) -> [[u8; TOTAL_TEXT_COLUMN]; 5] {
    let mut bitmap = [[0u8; TOTAL_TEXT_COLUMN]; 5];
    let mut col_index = 0;

    /*
        LED Matris için parametre olarak gelen metni 5x5'lik bitmap dizisine metni dönüştürürken
        , her karakterin 5 satır ve 3 sütunluk bir alana yerleştirildiğini varsayıyoruz.

        Bu nedenle, her karakter için 4 sütun (3 sütun karakter + 1 boşluk)
        ayırıyoruz. Toplamda 64 sütun olduğu için, metin uzunluğu 64'ü geçerse döngüden çıkıyoruz.

    */
    for c in text.chars() {
        if col_index >= TOTAL_TEXT_COLUMN {
            break;
        }

        match c {
            '0'..='9' => {
                let digit = c.to_digit(10).unwrap() as usize;
                let pattern = &DIGITS[digit];
                for row in 0..5 {
                    for col in 0..3 {
                        bitmap[row][col_index + col] = pattern[row][col];
                    }
                }
                col_index += 4;
            }
            '.' => {
                for row in 0..LED_ROW_LENGTH {
                    bitmap[row][col_index] = DOT[row][0];
                }
                col_index += 2;
            }
            _ => {}
        }
    }

    bitmap
}

pub fn float_to_ascii(f: f32) -> [u8; 5] {
    let mut buffer = [b' '; 5];

    let int_part = f as u8; // Sayıyı tam kısmı
    let frac_part = ((f - int_part as f32) * 10.0 + 0.5) as u8; // Sayının ondalık kısmı

    // Buna göre örneğin 18.7 sayısı için 1,8,.,7,0 şeklinde bir dizi döner.

    buffer[0] = b'0' + (int_part / 10);
    buffer[1] = b'0' + (int_part % 10);
    buffer[2] = b'.';
    buffer[3] = b'0' + (frac_part % 10);
    buffer[4] = 0;

    buffer
}

/*
    LED matrisinde kaydırma işlemini gerçekleştirmek için parametre olarak gelen
    bitmap dizisini kullanark, her kaydırma adımında yeni bir frame oluşturulur ve
    bu frame matris üzerinde gösterilir.
*/
pub fn scroll_text(
    matrix: &mut LedMatrix,
    bitmap: &[[u8; TOTAL_TEXT_COLUMN]; 5],
    total_width: usize,
) {
    for offset in 0..=(total_width - 5) {
        let mut frame = [[0u8; 5]; 5];

        for row in 0..5 {
            for col in 0..5 {
                frame[row][col] = bitmap[row][col + offset];
            }
        }

        matrix.draw(frame, 1000); // Her frame'i 1 saniye göster
    }
}
```

Nihayetinde main içeriğini aşağıdaki gibi yazabiliriz.

```rust
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
use nrf52833_hal::pac::Peripherals;
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
```

ve uygulamayı cihaza almak için;

```bash
cargo embed
```

### Lighthouse

BBC Micro:bit ile gelen örnek projelerden birisi de denizfeneri uygulaması. Bu örnekte belli aralıklarda yanıp sönen bir lamba söz konusu. Elimizdeki malzemeler şöyle.

- micro:bit
- MonkMakes Relay
- MonkMakes 1V Bulb
- 1.5 Volt kalem pil

Enstrümanlar arası bağlantılar yine timsah klipsler ile gerçekleştiriliyor. Aşağıdaki referans tabloyu baz alabiliriz.

| Timsah Klips | Micro:bit | Relay | Bulp | Pil(Batarya) |
|--------------|-----------|-------|------|--------------|
| Siyah        | GND       | GND   |      |              |
| Sarı         | GPIO 0    | IN    |      |              |
| Sarı         |           | OUT   | 2V   |              |
| Kırmızı      |           | OUT   |      | + Kutup      |
| Yeşil        |           |       | 1V   | - Kutup      |

Monkmakes resmi dokümanlarında için çalışan basit bir Python kodu da mevcut.

```python
from microbit import *

while True:
    pin0.write_digital(1)
    sleep(1000)
    pin0.write_digital(0)
    sleep(2000)

```

Peki rust tarafında bunu nasıl yapacağız? Bu sefer doğrudan donanım üzerindeki peripheral bileşenlerine erişip sinyalleri yönetiyoruz. Python örneğinden farklı olarak pin0,1 ve 2 eşleştirmelerini doğru şekilde ayarlamak gerekiyor. İşte kodlarımız.

```rust
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
```

Düzeneğin çalışma zamanı ise aşağıdaki gibi.

Işık kapalı iken.

![Lights On](./images/MicroBit_14.png)

Işık açık iken.

![Lights On](./images/MicroBit_15.png)

### Morse Codes

Bu örnekte Lighthouse projesindeki düzenek kullanılarak mors kodlarına göre sinyal yayınlaması sağlanmakta. Örnekte sadece **HELLO** kelimesini kullanıyoruz.

| Harf | Mors Kodu |
|------|-----------|
| H    | .-        |
| E    | .         |
| L    | .-..      |
| L    | .-..      |
| O    | - - -     |

Sırasıyla ilerleyelim. Mors kodlarında semboller arası duraksamalar çok önemli bir kavram. Bunu kolaylaştırmak için aşağıdaki gibi bir fonksiyondan yararlanabiliriz.

```rust
use cortex_m::asm::nop;

/// Gecikme fonksiyonu. Belirtilen sayıyı kullanarak bir döngü başlatır.
/// Döngü içerisinde nop (no operation) komutu kullanılır.
/// Bu komut, işlemcinin hiçbir şey yapmadan beklemesini sağlar.
/// 
/// ## Arguments
/// * `count` - Gecikme süresi için döngü sayısı.
/// 
/// ## Example
/// ```rust
/// use morse_codes::timer::delay;
/// 
/// fn main() {
///    delay(400_000); // Microdenetleyici için yaklaşık 1 saniye gecikme sağlar
/// }
/// ```
pub fn delay(count: u32) {
    for _ in 0..count {
        nop();
    }
}
```

Şimdilik sadece alfabedeki harflerin mors kodlarını temsil eden bir veri yapısı oluşturalım.

```rust
pub const DOT_DURATION: u32 = 100_000;
pub const DASH_DURATION: u32 = DOT_DURATION * 3;
pub const ELEMENT_GAP: u32 = DOT_DURATION;
// pub const LETTER_GAP: u32 = DOT_DURATION * 3;
pub const WORD_GAP: u32 = DOT_DURATION * 7;

pub const LETTER_MAP: [(&str, &str); 26] = [
    ("A", ".-"),
    ("B", "-..."),
    ("C", "-.-."),
    ("D", "-.."),
    ("E", "."),
    ("F", "..-."),
    ("G", "--."),
    ("H", "...."),
    ("I", ".."),
    ("J", ".---"),
    ("K", "-.-"),
    ("L", ".-.."),
    ("M", "--"),
    ("N", "-."),
    ("O", "---"),
    ("P", ".--."),
    ("Q", "--.-"),
    ("R", ".-."),
    ("S", "..."),
    ("T", "-"),
    ("U", "..-"),
    ("V", "...-"),
    ("W", ".--"),
    ("X", "-..-"),
    ("Y", "-.--"),
    ("Z", "--.."),
];
// const number_morse:[(&str;&str); 10] = [
//     ("0", "-----"), ("1", ".----"), ("2", "..---"), ("3", "...--"), ("4", "....-"),
//     ("5", "....."), ("6", "-...."), ("7", "--..."), ("8", "---.."), ("9", "----.")
// ];
```

Harflerin mors kodu karşılıklarını tespit etmek içinse Coder isimli veri yapısını kullanabiliriz.

```rust
//! # Coder Module
//! 
//! Bu modül, harf karşılıklarını Morse koduna dönüştürmek için kullanılan veri modelini içerir.
//!
//! ## Example
//! ```rust
//! use morse_codes::coder::Coder;
//! let morse_code = Coder::get_letter_code("A");
//! assert_eq!(morse_code, ".-");
//! ```
//!

use crate::constants::LETTER_MAP;

/// Mors kod harf eşleşmeleri için kullanılan veri modeli.
pub struct Coder;

impl Coder {
    /// Morse kodunu çözmek için kullanılan metot.
    /// 
    /// Parametre olarak gelen harfin karşlığı olan Morse kodunu döndürür.
    /// Eğer harf bulunamazsa boş bir string döner.
    /// 
    /// ## Args
    /// - `letter`: Morse kodu çözülmek istenen harf.
    /// 
    /// ## Returns
    /// - `&str`: Harfin karşılığı olan Morse kodu.
    /// 
    /// ## Example
    /// ```rust
    /// use morse_codes::coder::Coder;
    /// let morse_code = Coder::get_letter_code("A");
    /// assert_eq!(morse_code, ".-");
    /// ```
    pub fn get_letter_code(letter: &str) -> &str {
        for i in 0..LETTER_MAP.len() {
            if LETTER_MAP[i].0 == letter {
                return LETTER_MAP[i].1;
            }
        }
        " "
    }
}
```

Sinyalleri gönderme işini yine ayrı bir enstrümanının sorumluluğuna vermek iyi olacaktır. Aynen aşağıdaki gibi;

```rust
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
```

Bu soyutlamalar ve sorumlulukların paylaşımı aslında main tarafındaki işimizi kolaylaştırdı.

```rust
#![no_std]
#![no_main]

pub mod coder;
pub mod constants;
pub mod signal;
pub mod timer;

use crate::constants::WORD_GAP;
use crate::signal::Signal;
use crate::timer::delay;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Lighthouse Morse Code - 'Hello' started!");

    let p = Peripherals::take().unwrap();

    p.P0.pin_cnf[2].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled()
    });
    let word = "HELLO";
    let mut my_buf: [u8; 4] = [0; 4];

    loop {
        for letter in word.chars() {
            let l = letter.encode_utf8(&mut my_buf);
            Signal::send_letter(&p, l);
            // delay(LETTER_GAP);
        }

        delay(WORD_GAP);
    }
}
```

Örneği cihaza almak için;

```bash
cargo embed
```

### Motion Sensor

Bu projedeki ilk amaç Micro:Bit'e haricen bağlanan speaker sensor'e ses sinyali gönderilmesini sağlamak. Eğer mümkün olursa harekete duyarlı bir alarm mekanizması kullanmak.

![Speaker Sensor](images/MicroBit_16.png)

| Timsah Klips | Micro:bit | Speaker Sensor |
|--------------|-----------|----------------|
| Siyah        | GND       | GND            |
| Kırmızı      | 3V        | 3V             |
| Sarı         | GPIO 1    | Input          |

Mücadeledeki zorluklar;

1- Harekete duyarlı bir alarm mekanizması kodları MicroPython'a göre aşağıdaki kadar basit.

```python
from microbit import *
import music

z=accelerometer.get_z()

while True:
    if accelerometer.get_z() < z -50:
        music.play(music.BA_DING)
```

2- Ama Rust tarafında biraz daha donanıma yakınız. Nota bazlı ses elde etmek için sesin frekansına göre kare sinyalleri doğru şekilde gönderebilmek gerekiyor. Bunun için aşağıdaki kodu deneyebiliriz.

```rust
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

    let p = Peripherals::take().unwrap();
    p.P0.pin_cnf[2].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled()
    });

    loop {
        p.P0.outset.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 HIGH
        delay(200_000);
        p.P0.outclr.write(|w| unsafe { w.bits(1 << 2) }); // P0.02 LOW
        delay(400_000);
    }
}

fn delay(count: u32) {
    for _ in 0..count {
        nop();
    }
}
```

Ne yazık ki bu çalışmadaki sonuçlar pek de beklediğim gibi olmadı. Ses alçak seviyeden yüksek seviyeye hızlanarak artıyor gibiydi. Bir yerlerde yanlışlık yaptım.

## Mini Sözlük

- **ADC _(Analog-to-Digital Converter)_:** Analog sinyali dijitale çeviren dönüştürücüdür. Örneğin mikrofon sensörüne gelen veriyi dijital hale çevirmekte kullanılır.
- **Bare Metal Programming:** İşletim sistemi olmadan doğrudan donanım üzerinde yazılım geliştirme yaklaşımının adıdır. Yazıda ele aldığımız BBC micro:bit gibi cihazlarda **no_std** ile yazılan kodlar **bare-metal** seviyede olur.
- **BSP _(Board Support Package)_ :** Donanım kartına özel olarak geliştirilmiş başlangıç için gerekli tüm unsurları içeren paketlerin genel adıdır. Karta özel pin tanımlarını, saat ayarlarını, buton buzzer pin ayarlarını vb içerir. Örneğin **Micro:bit** kartı için kullandığımız **microbit-v2** BSP örneklerindendir. Bu tip paketler kullanılarak HAL katmanları da geliştirilebilir.
- **Debug Probe:** Bilgisayar ile mikrodenetleyici arasındaki fiziksel **debug** bağlantısını sağlayan araçtır.
- **ELF _(Executable and Linkable Format)_ :** Derlenen programın hedef sistemde çalıştırılabilir hale getirildiği dosya
  formatıdır.
- **Flashing:** Yazılan programın mikrodenetleyici üzerinde çalıştırılması genellikle Flash bellek bölgesine taşınması ile gerçekleştirilir. Bu işlem **flashing** olarak adlandırılır. **probe-rs** veya **openocd** gibi araçlarla yapılır.
- **GDB _(GNU Debugger)_ :** GNU ekosisteminde yaygın olarak kullanılan debugger.
- **GND _(Ground)_:** Devrelerde referans gerilim noktası olarak kullanılan ve bileşenlerin ortak kullandığı topraklama bağlantısıdır. Genellikle **0 Volt** kabul edilir ve sensörler/mikrodenetleyiciler aynı GND hattına bağlanarak çalışırlar _(3.3 volt, 5 Volt gibi)_
- **GPIO _(General Purpose Input/Output)_ :** Genel amaçlı giriş/çıkış pinleridir. LED yakmak, buton okumak, sensörlerden veri almak vb işlemlerde kullanılır. Hem giriş _(Input)_ hem de çıkış _(output)_ olarak yapılandırılabilir.
- **HAL _(Hardware Abstraction Layer)_ :** Donanım seviyesindeki enstrümanlarla konuşmayı kolaylaştıran bir arayüz olarak düşünülebilir. Örneğin **GPIO** pinlerine doğrudan erişmek yerine detaylardan uzak ve kolay kullanılabilir bir soyutlama sağlar. Buna göre pin registerlarına doğrudan erişmek yerine **pin.set_high** gibi anlamlı fonksiyonlar sağlar. Bazen BSP ile karıştırılabilir. **nrf52833-hal** küfesi örnek olarak verilebilir. Bu HAL örneğin belli mikrodenetleyicileri hedefler. Birde daha genel soyutlama sağlayan **embedded-hal** gibi küfeler _(crates)_ vardır. Şöyle de düşünebiliriz; **embedded-hal** genel arayüz tanımlamalarını içerir _(traits)_, **nrf52833-hal** ise **nRF52833**'e özel trait'leri gerçekten implemente eder ve dolayısıyla cihaza özgü komutlar da içerebilir.
- **I2C _(Inter-Integrated Circuit)_:** Bir senkronize seri haberleşme protokolüdür. Veri değiş tokuşu için bir data hattı ve clock line kullanır. Örneğin Microbit kartı üzerinde yer alan **LSM303AGR** bileşeni manyetometre ve ivmeölçer hareket sensörlerini içerir. Bu çip veri iletişimi için **I2C** tabanlı bir arayüz sağlar. Dolayısıyla sensörlerden anlık verileri I2C protokolü üstünden kullanabiliriz.
- **MCU _(Microcontroller Unit)_ :** İşlemci çekirdeği, flash bellek, RAM ve çeşitli çevresel birimleri tek bir çipte
  barındıran elektronik birim olarak ifade edilebilir.
- **PAC _(Peripheral Access Crate)_ :** Mikrodenetleyici üreticisinin sağladığı register haritalarını, API'leri otomatik
  olarak Rust koduna çeviren paketlerdir. HAL kütüphaneleri genelde PAC modülleri üzerine kurulur.
- **Peripheral :** Mikrodenetleyicinin içinde bulunan **GPIO**, **UART**, **SPI**, **I2C**, **Timer**, **ADC** gibi
  birimlerdir. Her biri ayrı bir periferik modül olarak kabul edilir.
- **PWM _(Pulse Width Modulation)_:** PWM, **Pulse Width Modulation** anlamına gelir ve genellikle analog sinyalleri dijital sinyallere dönüştürmek için kullanılır. Bir sinyalin belirli bir süre boyunca açık kalma süresini _(duty cycle)_ kontrol ederek ortalama bir voltaj değeri oluşturur. Bu değer hoparlör gibi cihazların ses çıkışını kontrol etmek için kullanılabilir. Hatta bir **LED parlaklığını** kontrol etmek için de kullanılabilir.
- **Reset Vector:** Mikrodenetleyici yeniden başlatıldığında _(reset)_ çalışmaya başladığı ilk bellek adresidir. Başlangıç kodu da buradan çalıştırılır. Örneklerde embed edilen kodlar bu adresten başlatılır.
- **SAADC _(Successive Approximation Analog-to-Digital Converter)_ :** Analog sinyalleri dijital değerlere dönüştüren ve örnekleme sırasında ardışık yaklaşıklamalar kullanan bir **ADC** türüdür.
- **SPI _(Serial Peripheral Interface)_:** Ağırlıklı olarak yine mikrodenetleyicilerde ele alınan bir senkron ve seri haberleşme standardıdır.
- **SVD _(System View Description)_:** Mikrodenetleyici üzerindeki **register** ve ilişkili bitleri tarifleyen bir harita dosyası olarak düşünülebilir. **svd2rust** gibi crate'ler bu dosyaları **parse** edebilir ve bu da **Peripherals Access Crate**'lerin oluşturulmasını kolaylaştırır. Genellikle **XML _(eXtensiable Markup Language)_**
  tabanlı bir dosyadır.
- **UART _(Universal Asynchronous Receiver-Transmitter)_:** Mikrodenetleyicilerde sensör verilerinin aktarım işlemlerini
  tanımlayan bir seri iletişim protokoldür. Sadece mikrodenetleyiciler değil bilgisayarlar içinde geçerlidir.

## Kaynaklar

- [Embedded Rust Docs - Discovery](https://docs.rust-embedded.org/discovery/microbit/index.html)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html)
- [A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary/#panic-implementation)
- [Ferrous System Embedding Training](https://github.com/ferrous-systems/embedded-trainings-2020)
- [Microbit Examples](https://github.com/nrf-rs/microbit/tree/03e97a2977d22f768794dd8b0a4b6677a70f119a/examples)
- [Microbit.org Hardware](https://tech.microbit.org/hardware/)
- [Micro:bit V2 için donanım şeması](https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF)
- [The Embedded Rustacean](https://www.theembeddedrustacean.com/)
- [Embedded programming in Rust with Microbit V2](https://www.youtube.com/watch?v=b7zWIKZp4ls)
- [nRF52833 Product Specification](https://docs-be.nordicsemi.com/bundle/ps_nrf52833/attach/nRF52833_PS_v1.7.pdf?_LANG=enus)
- [LSM303AGR](https://www.st.com/en/mems-and-sensors/lsm303agr.html)
- [MonkMakes Çevre Sensörü için Bilgiler](https://monkmakes.com/mb_2a)
