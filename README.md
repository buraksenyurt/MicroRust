# MicroRust

Bu repoda mikrodenetleyiciler üzerinde Rust ile kodlama pratiklerine yer verilmesi planlanmaktadır.

- [İçerik](#microrust)
  - [Microbit Üzerinde Geliştirme](#microbit-üzerinde-geliştirme)
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
    - [Mini Sözlük](#mini-sözlük)
    - [Kaynaklar](#kaynaklar)

## Microbit Üzerinde Geliştirme

Mikrodenetleyiciler genel olarak sınırlı kapasiteye sahip, çoğunlukla işletim sistemi ile birlikte gelmeyen, çeşitli sensörler yardımıyla çevresel ortamlardan veri toplanması gibi işlerde sıklıkla kullanılan entegre kartlardır. Portatif ve ekonomik olmaları birçok düzeneğe dahil edilmelerini mümkün kılar. Mikrodenetleyiciler üzerine geliştirme yapmak için farklı programlama dilleri kullanılabilir ancak RTOS _(real-time operating system-RTOS)_ ile birlikte gelmedikleri durumlarda bare-metal programming pratiklerini uygulamak gerekir.

Bu repoya konu olan [BBC micro:bit](https://microbit.org/) üzerinde Python, Scratch, Microsoft Make ile programlama yapılabileceği gibi C ve Rust gibi dillerle de geliştirme yapmak mümkündür.

## Cihaz Hakkında

Bu repodaki örnekler BBC Micro:bit v2.2 üzerinde geliştirilmektedir. ARM tabanlı Cortex işlemciye _(nRF52833, Nordic Semiconductor)_ sahip olan cihaz 512 Kb Flash ve 128 Kb Ram belleğe sahiptir.

![Micro:bit 00](./images/MicroBit_00.jpg)

![Micro:bit 01](./images/MicroBit_01.jpg)

- Doğrudan Microcontroller Unit üzerinde programlama yaparken kartın donanım şema bilgilerine ihtiyaç duyulacaktır. [Kaynak](https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF)

Mikrodenetleyici USB üzerinden bilgisayara bağlanabilir. Bilgisayara bağlandıktan sonra COM3 portundan bağlı bir cihaz gibi görünür.

```bash
# Kontrol için
mode
```

![COM3 Status](./images/MicroBit_02.png)

## Gerekli Kurulumlar

Örnekler Windows 11 işletim sistemi üzerinde geliştirilmektedir.

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

Bu örnek düzenli aralıklarla Windows makinedeki terminal ekranına mesaj göndermektedir.

```bash
# Kod kontrolü
cargo check

# Flashing (Cihaza dağıtım)
cargo embed
```

Beklenen çıktı.

![First Contact Runtime](./images/MicroBit_03.png)

### Debugging

Terminal bazlı debug operasyonlarında GDB kullanılır.

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

### Blinking Led

Bu örnekte 5x5 Led matrisinin ortasındaki led ışığının saniyede bir yanıp sönmesi sağlanır. Doğrudan mikrodenetleyicinin GPIO adresleri üzerinde işlem yapılarak ilerlenmiştir.

```bash
cargo embed
```

Beklenen çıktı ortadaki led ışığının saniyede bir yanıp sönmesidir.

### Blinking Led v2

Blinking Led örneğindekinden farklı olarak bu örnekte Hardware Abstraction Layer crate'ler kullanılmıştır.

```bash
cargo embed
```

Beklenen çıktı ortadaki led ışığının yaklaşık 1.5 saniyelik sürelerde yanıp sönmesidir.

### Blinking Rust

Bu örnekte A düğmesine basıldığında LED matriste sırasyıla R, U , S ve T harfleri görünür.

```bash
cargo embed
```

Beklenen çıktı A düğmesine basıldığında LED ışıklarında RUST kelimesinin harflerinin sıralı bir şekilde görünmesidir.

### Beep

Bu örnekte ise B düğmesine basıldığında denetleyici üzerindeki hoparlörden beep benzeri bir ses çıkartılması sağlanmaktadır.

```bash
cargo embed
```

Beklenen çıktı, B düğmesine basıldığında beep sesi duyulmasıdır.

### Uart

UART _(Universal Asynchronous Receiver/Transmitter)_ mikrodenetleyici üzerinde yer alan bir çevresel iletişim birimidir _(Peripheral)_. Bu arabirimi kullanarak mikrodenetleyici ve bilgisayar arasında haberleşme sağlanabilir. Modül Transmitter ve Receiver için pin'lere sahiptir. Örneğin bilgisayarın COM portuna seri haberleşme protokolü üzerinden mesaj gönderilebilir veya bilgisayardan dönen mesaj okunabilir.

```bash
# Microbit bilgisayar USB kablosu ile bağlandıktan sonra
# mode komutu ile COM port bilgileri alınabilir.
mode

# Flashing için
cargo embed
```

COM portuna gelen mesajları görmek için PuTTY uygulamasından yararlanılabilir. Uygulama ayarları aşağıdaki gibi yapılandırılmalıdır.

![PuTTY Session](./images/MicroBit_06.png)

![PuTTY](./images/MicroBit_05.png)

![UART Runtime](./images/MicroBit_07.png)

## Mini Sözlük

- **ADC _(Analog-to-Digital Converter)_:**  Analog sinyali dijitale çeviren dönüştürücüdür. Örneğin mikrofon sensörüne gelen veriyi dijital hale çevirmekte kullanılır.
- **BSP _(Board Support Package)_ :** Donanım kartına özel olarak geliştirilmiş başlangıç için gerekli tüm unsurları içeren paketlerin genel adıdır. Karta özel pin tanımlarını, saat ayarlarını, buton buzzer pin ayarlarını vb içerir. Örneğin Micro:bit için kullandığımız [microbit-v2](https://crates.io/crates/microbit-v2) BSP örneklerindendir. Bu tip paketler kullanılarak HAL katmanları da geliştirilebilir.
- **ELF _(Executable and Linkable Format)_ :** Derlenen programın hedef sistemde çalıştırılabilir hale getirildiği dosya formatıdır.
- **GPIO _(General Purpose Input/Output)_ :** Genel amaçlı giriş/çıkış pinleridir. LED yakmak, buton okumak, sensörlerden veri almak vb işlemlerde kullanılır. Hem giriş _(Input)_ hem de çıkış _(output)_ olarak yapılandırılabilir.
- **UART _(Universal Asynchronous Receiver-Transmitter)_:** Mikrodenetleyicilerde sensör verilerinin aktarım işlemlerini tanımlayan bir seri iletişim protokoldür. Sadece mikrodenetleyiciler değil bilgisayarlar içinde geçerlidir.
- **SPI _(Serial Peripheral Interface)_:** Ağırlıklı olarak yine mikrodenetleyicilerde ele alınan bir senkron ve seri haberleşme standardıdır.
- **I2C _(Inter-Integrated Circuit)_:**
- **HAL _(Hardware Abstraction Layer)_ :** Donanım seviyesindeki enstrümanlarla konuşmayı kolaylaştıran bir arayüz olarak düşünülebilir. Örneğin GPIO pinlerine doğrudan erişmek yerine detaylardan uzak ve kolay kullanılabilir bir soyutlama sağlar. Örneğin pin registerlarına doğrudan yazmak yerine pin.set_high gibi anlamlı fonksiyonlar sağlar. Bazen BSP ile karıştırılabilir.[nrf52833-hal](https://crates.io/crates/nrf52833-hal) örnek olarak verilebilir. Bu HAL örneğin belli mikrodenetleyicileri hedefler. Birde daha genel soyutlama sağlayan [embedded-hal](https://crates.io/crates/embedded-hal) gibi küfeler vardır. Bunu şöyle de düşünebiliriz; embedded-hal genel arayüz tanımlamalarını içerir _(traits)_, nrf52833-hal ise nRF52833'e özel olarak ilgil trait'leri gerçekten implemente eder. Dolayısıyla cihaza özgü komutlar da içerebilir.
- **Peripheral :** Mikrodenetleyicinin içinde bulunan **GPIO**, **UART**, **SPI**, **I2C**, **Timer**, **ADC** gibi birimlerdir. Her biri ayrı bir periferik modül olarak kabul edilir.
- **PAC _(Peripheral Access Crate)_ :** Mikrodenetleyici üreticisinin sağladığı register haritalarını, API'leri otomatik olarak Rust koduna çeviren paketlerdir. HAL kütüphaneleri genelde PAC modülleri üzerine kurulur.
- **MCU _(Microcontroller Unit)_ :** İşlemci çekirdeği, flash bellek, RAM ve .eşitli çevresel birimleri tekbir çipte barındıran elektronik birim.
- **Flashing :** Yazılan programın mikrodenetleyici üzerinde çalıştırılması genellikle Flask bellek bölgesine taşınması ile gerçekleştirilir. Bu işlem flashing olarak adlandırılır. probe-rs veya openocd gibi araçlarla yapılır.
- **GDB _(GNU Debugger)_ :** GNU ekosisteminde yaygın olarak kullanılan debugger.
- **Bare Metal Programming:** İşletim sistemi olmadan doğrudan donanım üzerinde yazılım geliştirme yaklaşımının adıdır. Yazıda ele aldığımız **BBC micro:bit** gibi cihazlarda **no_std** ile yazılan kodlar bare-metal seviyede olur.
- **SVD _(System View Description)_:** Mikrodenetleyici üzerindeki register ve ilişkili bitleri tarifleyen bir harita dosyası olarak düşünülebilir. [svd2rust](https://crates.io/crates/svd2rust) gibi crate'ler bu dosyaları parse edebilir ve buda **Peripherals Access Create**'lerin oluşturulmasını kolaylaştırır. Genellikle XML tabanlı bir dosyadır.
- **Reset Vector:** Mikrodenetleyici yeniden başlatıldığında _(reset)_ çalışmaya başladığı ilk bellek adresidir. Başlangıç kodu da buradan çalıştırılır. Örneklerde attığımız kodlar bu adresten başlatılır.
- **Debug Probe:** Bilgisayar ile mikrodenetleyici arasındaki fiziksel debug bağlantısını sağlayan araçtır.
- **PWM _(Pulse With Modulation)_:** PWM, Pulse Width Modulation anlamına gelir ve genellikle analog sinyalleri dijital sinyallere dönüştürmek için kullanılır. Bir sinyalin belirli bir süre boyunca açık kalma süresini (duty cycle) kontrol ederek ortalama bir voltaj değeri oluşturur. Bu değer hoparlör gibi cihazların ses çıkışını kontrol etmek için kullanılabilir. Hatta bir LED parlaklığını kontrol etmek için de kullanılabilir.

## Kaynaklar

- [Embedded Rust Docs - Discovery](https://docs.rust-embedded.org/discovery/microbit/index.html)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html)
- [A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary/#panic-implementation)
- [Ferrous System Embedding Training](https://github.com/ferrous-systems/embedded-trainings-2020)
- [Microbit Examples](https://github.com/nrf-rs/microbit/tree/03e97a2977d22f768794dd8b0a4b6677a70f119a/examples)
- [Microbit.org](https://microbit.org/)
- [The Embedded Rustacean](https://www.theembeddedrustacean.com/)
- [Embedded programming in Rust with Microbit V2](https://www.youtube.com/watch?v=b7zWIKZp4ls)
- [Micro:bit V2 için donanım şeması](https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF)
- [nRF52833 Product Specification](https://docs-be.nordicsemi.com/bundle/ps_nrf52833/attach/nRF52833_PS_v1.7.pdf?_LANG=enus)
