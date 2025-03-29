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
        - [Blinking Rust](#blinking-rust)
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

### Debugging

### Blinking Led

### Blinking Rust

## Mini Sözlük

## Kaynaklar
