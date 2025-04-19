//! # Coder Module
//! 
//! Bu modül, harf karşılıklarını Morse koduna dönüştürmek için kullanılan veri modelini içerir.
//!
//! ## Example
//! ```rust
//! use morse_codes::coder::Coder;
//! let morse_code = Coder::get_letter_code("A");
//! assert_eq!(morse_code, ".-");
//! ```
//!

use crate::constants::LETTER_MAP;

/// Mors kod harf eşleşmeleri için kullanılan veri modeli.
pub struct Coder;

impl Coder {
    /// Morse kodunu çözmek için kullanılan metot.
    /// 
    /// Parametre olarak gelen harfin karşlığı olan Morse kodunu döndürür.
    /// Eğer harf bulunamazsa boş bir string döner.
    /// 
    /// ## Args
    /// - `letter`: Morse kodu çözülmek istenen harf.
    /// 
    /// ## Returns
    /// - `&str`: Harfin karşılığı olan Morse kodu.
    /// 
    /// ## Example
    /// ```rust
    /// use morse_codes::coder::Coder;
    /// let morse_code = Coder::get_letter_code("A");
    /// assert_eq!(morse_code, ".-");
    /// ```
    pub fn get_letter_code(letter: &str) -> &str {
        for i in 0..LETTER_MAP.len() {
            if LETTER_MAP[i].0 == letter {
                return LETTER_MAP[i].1;
            }
        }
        " "
    }
}
