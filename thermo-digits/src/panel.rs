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
