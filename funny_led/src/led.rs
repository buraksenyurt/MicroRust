use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{
    gpio::{Level, Output, Pin, PushPull, p0, p1},
    pac,
    timer::Timer,
};

pub struct LedMatrix {
    pub rows: [Pin<Output<PushPull>>; 5],
    pub cols: [Pin<Output<PushPull>>; 5],
    pub timer: Timer<pac::TIMER0>,
}

impl LedMatrix {
    pub fn new() -> Self {
        let p = pac::Peripherals::take().unwrap();

        let p0_parts = p0::Parts::new(p.P0);
        let p1_parts = p1::Parts::new(p.P1);

        let row_pins: [Pin<Output<PushPull>>; 5] = [
            p0_parts.p0_21.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_22.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_15.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_24.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_19.into_push_pull_output(Level::Low).degrade(),
        ];

        let col_pins: [Pin<Output<PushPull>>; 5] = [
            p0_parts.p0_28.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_11.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_31.into_push_pull_output(Level::High).degrade(),
            p1_parts.p1_05.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_30.into_push_pull_output(Level::High).degrade(),
        ];

        LedMatrix {
            timer: Timer::new(p.TIMER0),
            rows: row_pins,
            cols: col_pins,
        }
    }

    pub fn clear_all(&mut self) {
        for r in self.rows.iter_mut() {
            r.set_low().ok();
        }
        for c in self.cols.iter_mut() {
            c.set_high().ok();
        }
    }

    pub fn draw(&mut self, shape: [[u8; 5]; 5], duration_ms: u32) {
        let frame_count = duration_ms / 5;

        for _ in 0..frame_count {
            for row in 0..5 {
                for r in self.rows.iter_mut() {
                    r.set_low().ok();
                }

                self.rows[row].set_high().ok();

                for col in 0..5 {
                    if shape[row][col] == 1 {
                        self.cols[col].set_low().ok();
                    } else {
                        self.cols[col].set_high().ok();
                    }
                }

                self.timer.delay_ms(1);
            }
        }

        self.clear_all();
    }
}
