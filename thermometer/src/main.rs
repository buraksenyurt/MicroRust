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
