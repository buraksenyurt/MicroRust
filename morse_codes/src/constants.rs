pub const DOT_DURATION: u32 = 100_000;
pub const DASH_DURATION: u32 = DOT_DURATION * 3;
pub const ELEMENT_GAP: u32 = DOT_DURATION;
pub const LETTER_GAP: u32 = DOT_DURATION * 3;
pub const WORD_GAP: u32 = DOT_DURATION * 7;

pub const LETTER_MAP: [(&str, &str); 26] = [
    ("A", ".-"),
    ("B", "-..."),
    ("C", "-.-."),
    ("D", "-.."),
    ("E", "."),
    ("F", "..-."),
    ("G", "--."),
    ("H", "...."),
    ("I", ".."),
    ("J", ".---"),
    ("K", "-.-"),
    ("L", ".-.."),
    ("M", "--"),
    ("N", "-."),
    ("O", "---"),
    ("P", ".--."),
    ("Q", "--.-"),
    ("R", ".-."),
    ("S", "..."),
    ("T", "-"),
    ("U", "..-"),
    ("V", "...-"),
    ("W", ".--"),
    ("X", "-..-"),
    ("Y", "-.--"),
    ("Z", "--.."),
];
// const number_morse:[(&str;&str); 10] = [
//     ("0", "-----"), ("1", ".----"), ("2", "..---"), ("3", "...--"), ("4", "....-"),
//     ("5", "....."), ("6", "-...."), ("7", "--..."), ("8", "---.."), ("9", "----.")
// ];
