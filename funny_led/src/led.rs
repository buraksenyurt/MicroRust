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
