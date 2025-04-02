#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use embedded_hal::digital::InputPin;
use microbit::{
    hal::{Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

/*
    Bu örnekte micro:bit kartı üzerinde bulunan LSM303AGR ivmeölçer (Accelerometer) kullanılarak hız hesaplaması yapılmaktadır.
    LSM303AGR bileşeni x,y,z eksenlerinde ivme ölçümü yapabilen bir sensördür.
    Örnekte ivmeölçeri 50Hz örnekleme hızı ile çalıştırıyoruz ki bu değerin büyüklüğü ivmeölçerin hassasiyetini etkiler.
    Örnekleme hızı arttıkça ivmeölçerin hassasiyeti artar ancak işlemci üzerindeki yük de buna bağlı olarak artar.
    İvmeölçer eğer A düğmesine basıldıysa her 20ms'de bir ivme verilerini okur ve bu verileri kullanarak hız hesaplaması yapar.
    Hız hesaplaması, eksen bazlı ivme değerlerinin zamanla çarpımına bakılmak suretiyle yapılır.
    Olası sapmaların önüne geçmek için belli sayıda ivme verisi ölçülür ortalamaları alınır ve bu ortalamalar
    ivmeölçer verilerinden çıkarılır. Bu sayede ivmeölçer verileri filtrelenmiş olur.
    Ayrıca gürültü filtrelemesi de yapılır. Eğer ivme değerleri belirli bir eşiğin altındaysa bu değerler sıfırlanır.s
*/

const CALIB_SAMPLES: usize = 100; // Ortalama hesaplamaları için alınacak örnek sayısı
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

    /*
        Buradan itibaren kalibrasyon yapmaya çalışıyoruz.
        Öncelikle 100 adet ivmeölçer verisi topluyoruz.
        Bu verilerden x, y, z eksenlerindeki ivme değerlerinin ortalamasını alıyoruz ve bias ile başlayan değişkenlerde topluyoruz.
        Amacımız birazdan ivmeölçer verilerini filtrelemek ve gürültüden arındırmak.
        Zira durduğu yerde dahi yerçekimi değerine bağlı olarak veri üretilecektir.
    */
    let mut sum_x = 0;
    let mut sum_y = 0;
    let mut sum_z = 0;
    let mut collected = 0;
    rprintln!("Calibrating...");
    while collected < CALIB_SAMPLES {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
            sum_x += x;
            sum_y += y;
            sum_z += z;
            collected += 1;
        }
    }

    let (bias_x, bias_y, bias_z) = (
        sum_x / CALIB_SAMPLES as i32,
        sum_y / CALIB_SAMPLES as i32,
        sum_z / CALIB_SAMPLES as i32,
    );

    let (mut vx, mut vy, mut vz) = (0.0_f32, 0.0_f32, 0.0_f32);

    loop {
        if let Ok(true) = board.buttons.button_a.is_low() {
            // Eğer A butonuna basılırsa
            if sensor.accel_status().unwrap().xyz_new_data() {
                // ve ivmeölçer yeni veri ürettiyse
                let (x, y, z) = sensor.acceleration().unwrap().xyz_mg(); // bu verileri oku

                let (raw_x, raw_y, raw_z) = (x - bias_x, y - bias_y, z - bias_z); // sapma değerlerine göre ham x,y,z değerlerini hesapla

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
