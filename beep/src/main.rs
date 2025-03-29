#![no_main]
#![no_std]

use panic_halt as _;

use crate::gpio::{p0::P0_00, Output, PushPull};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    hal::{
        gpio,
        prelude::*,
        pwm::{self, Pwm},
        Timer,
    },
    Board,
};

fn play_beep(
    board_pwm: microbit::pac::PWM0,
    speaker_pin: P0_00<Output<PushPull>>,
    board_timer: microbit::pac::TIMER0,
) {
    let pwm = Pwm::new(board_pwm);
    pwm.set_output_pin(pwm::Channel::C0, speaker_pin.degrade());
    pwm.set_prescaler(pwm::Prescaler::Div1);
    pwm.set_counter_mode(pwm::CounterMode::UpAndDown);
    pwm.set_max_duty(32767);
    pwm.set_period(432u32.hz());
    pwm.set_duty_on_common(32767 / 2);
    pwm.enable();

    let mut timer = Timer::new(board_timer);
    timer.delay_ms(500u32);

    pwm.disable();
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::High);
    
    play_beep(board.PWM0, speaker_pin, board.TIMER0);

    loop {
        cortex_m::asm::wfi();
    }
}

/*
    Bu program Micro:bit V2 hoparlöründen 432 Hz frekansında ses çalmak için
    bir PWM sinyali oluşturur (Beep sesine benzer bir ses).
    Hoparlör, belirli bir süre boyunca (500 ms) aktif kalır ve ardından
    PWM devre dışı bırakılır.

    Kabaca aşağıdaki gibi bir kare sinyal oluşturur:

       HIGH ____      ____      ____
        |   |    |   |    |   |
        |   |    |   |    |   |
   LOW  |___|____|___|____|___|____
          <----> <----> <---->
           periyot aralığı 1/432 saniye


    PWM, Pulse Width Modulation anlamına gelir ve genellikle
    analog sinyalleri dijital sinyallere dönüştürmek için
    kullanılır. Bir sinyalin belirli bir süre boyunca
    açık kalma süresini (duty cycle) kontrol ederek
    ortalama bir voltaj değeri oluşturur. Bu değer hoparlör
    gibi cihazların ses çıkışını kontrol etmek için
    kullanılabilir. Hatta bir LED parlaklığını kontrol etmek için
    de kullanılabilir. PWM, genellikle bir mikrodenetleyici
    üzerinde bir zamanlayıcı (timer) kullanılarak
    gerçekleştirilir. Bu zamanlayıcı, belirli bir frekansta
    (örneğin 432 Hz) bir sinyal oluşturur ve bu sinyalin
    açık kalma süresini (duty cycle) ayarlayarak
    ortalama bir voltaj değeri oluşturur. Bu voltaj değeri,
    hoparlör gibi bir yük üzerinde uygulandığında, yükün
    ortalama bir voltaj değeri almasını sağlar. Bu, hoparlörün
    ses çıkarmasını sağlar. Örneğin, %50 duty cycle
    kullanıldığında, sinyalin yarısı açık ve yarısı kapalıdır.
    Bu, hoparlörün ortalama bir voltaj değeri alarak ses çıkarmasını
    sağlar.
*/
