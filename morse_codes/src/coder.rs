use crate::constants::LETTER_MAP;

pub struct Coder;

impl Coder {
    pub fn get_letter_code(letter: &str) -> &str {
        for i in 0..LETTER_MAP.len() {
            if LETTER_MAP[i].0 == letter {
                return LETTER_MAP[i].1;
            }
        }
        " "
    }
}
