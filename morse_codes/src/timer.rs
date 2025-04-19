use cortex_m::asm::nop;

/// Gecikme fonksiyonu. Belirtilen sayıyı kullanarak bir döngü başlatır.
/// Döngü içerisinde nop (no operation) komutu kullanılır.
/// Bu komut, işlemcinin hiçbir şey yapmadan beklemesini sağlar.
/// 
/// ## Arguments
/// * `count` - Gecikme süresi için döngü sayısı.
/// 
/// ## Example
/// ```rust
/// use morse_codes::timer::delay;
/// 
/// fn main() {
///    delay(400_000); // Microdenetleyici için yaklaşık 1 saniye gecikme sağlar
/// }
/// ```
pub fn delay(count: u32) {
    for _ in 0..count {
        nop();
    }
}
