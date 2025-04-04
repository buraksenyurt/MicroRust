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
