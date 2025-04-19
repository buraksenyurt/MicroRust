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
