// Aşağıdaki fonksiyon Newton-Raphson yöntemini kullanarak karekök hesaplaması yapar
// Normalde standart kütüphanelerde bulunan sqrt fonksiyonu kullanılabilir
// Ancak bu örnekte standart kütüphane kullanılmadığından sqrt, powi gibi fonksiyonları kullanamıyoruz.
pub fn sqrt(value: f32) -> f32 {
    if value <= 0.0 {
        return 0.0;
    }
    let mut guess = value;
    for _ in 0..10 {
        guess = 0.5 * (guess + value / guess);
    }
    guess
}