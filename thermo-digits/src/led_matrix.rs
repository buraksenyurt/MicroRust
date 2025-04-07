use crate::constants::*;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{
    gpio::{Level, Output, Pin, PushPull, p0, p1},
    pac,
    timer::Timer,
};

pub struct LedMatrix {
    pub rows: [Pin<Output<PushPull>>; LED_ROW_LENGTH],
    pub cols: [Pin<Output<PushPull>>; LED_COL_LENGTH],
    pub timer: Timer<pac::TIMER0>,
}

impl LedMatrix {
    pub fn new(p0: pac::P0, p1: pac::P1, timer: pac::TIMER0) -> Self {
        let p0_parts = p0::Parts::new(p0);
        let p1_parts = p1::Parts::new(p1);

        let row_pins: [Pin<Output<PushPull>>; LED_ROW_LENGTH] = [
            p0_parts.p0_21.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_22.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_15.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_24.into_push_pull_output(Level::Low).degrade(),
            p0_parts.p0_19.into_push_pull_output(Level::Low).degrade(),
        ];

        let col_pins: [Pin<Output<PushPull>>; LED_COL_LENGTH] = [
            p0_parts.p0_28.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_11.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_31.into_push_pull_output(Level::High).degrade(),
            p1_parts.p1_05.into_push_pull_output(Level::High).degrade(),
            p0_parts.p0_30.into_push_pull_output(Level::High).degrade(),
        ];

        LedMatrix {
            timer: Timer::new(timer),
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

    pub fn draw(&mut self, digit: [[u8; LED_ROW_LENGTH]; LED_COL_LENGTH], duration_ms: u32) {
        let frame_count = duration_ms / FRAME_FACTOR;

        for _ in 0..frame_count {
            for (row, _) in digit.iter().enumerate().take(LED_ROW_LENGTH) {
                for r in self.rows.iter_mut() {
                    r.set_low().ok();
                }

                self.rows[row].set_high().ok();

                for col in 0..LED_COL_LENGTH {
                    if digit[row][col] == LIGHT_ON {
                        self.cols[col].set_low().ok();
                    } else {
                        self.cols[col].set_high().ok();
                    }
                }

                self.timer.delay_us(DRAW_DELAY);

                for c in self.cols.iter_mut() {
                    c.set_high().ok();
                }
            }
        }

        self.clear_all();
    }
}
