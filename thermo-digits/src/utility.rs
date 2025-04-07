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
