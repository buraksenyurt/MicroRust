#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

/*
    Bu örnekte micro:bit kartı üzerinde bulunan LSM303AGR ivmeölçer (Accelerometer) kullanılarak hız hesaplaması yapılmaktadır.
    LSM303AGR bileşeni x,y,z eksenlerinde ivme ölçümü yapabilen bir sensördür.
    Örnekte ivmeölçeri 50Hz örnekleme hızı ile çalıştırıyoruz.
    Bu değerin büyüklüğü ivmeölçerin hassasiyetini etkiler.
    Örnekleme hızı arttıkça ivmeölçerin hassasiyeti artar ancak işlemci üzerindeki yük de artar.
    Doğru bir ayarlama için kalibrasyon yapılması gerekebilir.
    İvmeölçer, her 20ms'de bir ivme verilerini okur ve bu verileri kullanarak hız hesaplaması yapar.
    Hız hesaplaması, eksen bazlı ivme değerlerinin zamanla çarpımına bakılmak suretiyle yapılır.
*/

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer0 = Timer::new(board.TIMER0);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    sensor.init().unwrap();
    rprintln!("Sensor initialized successfully!");
    sensor
        .set_accel_mode_and_odr(
            &mut timer0,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();

    let (mut vx, mut vy, mut vz) = (0.0_f32, 0.0_f32, 0.0_f32);

    let delta_time = 0.02; // 50Hz örnekleme → 20ms
    const MILL_G: f32 = 9.80665; // 1g = 9.80665 m/s^2, 1g = 1000mg (Yerçekimi ivmesinin milli-g cinsinden değeri)

    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();

            let (ax, ay, az) = (
                x as f32 * MILL_G / 1000.0,
                y as f32 * MILL_G / 1000.0,
                z as f32 * MILL_G / 1000.0,
            );

            vx += ax * delta_time;
            vy += ay * delta_time;
            vz += az * delta_time;

            let speed = sqrt(vx * vx + vy * vy + vz * vz);
            rprintln!("Current Speed : {:.2} m/s", speed);
            rprintln!("Hız (m/s): x: {:.2}, y: {:.2}, z: {:.2}", vx, vy, vz);
        }
    }
}

// Aşağıdaki fonksiyon Newton-Raphson yöntemini kullanarak karekök hesaplaması yapar
// Normalde standart kütüphanelerde bulunan sqrt fonksiyonu kullanılabilir
// Ancak bu örnekte standart kütüphane kullanılmadığından sqrt, powi gibi fonksiyonları kullanamıyoruz.
fn sqrt(value: f32) -> f32 {
    if value <= 0.0 {
        return 0.0;
    }
    let mut guess = value;
    for _ in 0..10 {
        guess = 0.5 * (guess + value / guess);
    }
    guess
}
