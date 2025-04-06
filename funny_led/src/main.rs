#![no_main]
#![no_std]

mod led;
mod shape;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use led::LedMatrix;
use panic_halt as _;
use shape::*;

#[entry]
fn main() -> ! {
    let mut led_matrix = LedMatrix::new();
    let shapes = [
        get(Shape::Square),
        get(Shape::Hearth),
        get(Shape::UpArrow),
        get(Shape::DownArrow),
    ];
    let mut current_shape = 0;

    loop {
        led_matrix.clear_all();
        led_matrix.draw(shapes[current_shape], 5000);
        led_matrix.timer.delay_ms(500);
        current_shape = (current_shape + 1) % shapes.len();
    }
}
