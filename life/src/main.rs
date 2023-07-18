#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        prelude::*,
        timer::Timer,
    },
};

use nanorand::{pcg64::Pcg64, Rng};
use panic_halt as _;
mod life;
use life::*;

pub fn random(fb: &mut [[u8; 5]; 5], fr: u128) {
    let mut rng: Pcg64 = nanorand::Pcg64::new_seed(fr);
    let mut b: bool;

    for row in 0..5 {
        for col in 0..5 {
            b = rng.generate();
            if b {
                fb[row][col] = 1
            } else {
                fb[row][col] = 0
            }
        }
    }
}

pub fn complement(fb: &mut [[u8; 5]; 5]) {
    for row in 0..5 {
        for col in 0..5 {
            if fb[row][col] == 1 {
                fb[row][col] = 0
            } else {
                fb[row][col] = 1
            }
        }
    }
}

#[entry]
fn main() -> ! {
    let mut frames = 0;
    let mut b_sleep = 0;
    let mut no_life_fr = 0;

    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER1);
    let button_a = board.buttons.button_a.into_pullup_input();
    let button_b = board.buttons.button_b.into_pullup_input();

    let mut led: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    random(& mut led, frames);

    loop {
        if button_a.is_low().unwrap() {
            random(& mut led, frames);
            no_life_fr = 0;
        }

        else if button_b.is_low().unwrap() {
            if b_sleep > 5 {
                complement(& mut led);
                b_sleep = 0;
                no_life_fr = 0;
            }
        }

        else if done(& mut led) {
            no_life_fr += 1;
            if no_life_fr > 5 {
                random(& mut led, frames);
                no_life_fr = 0;
            }
        }

        else {
            life(& mut led);
        }
        display.show(&mut timer, led, 100);
        // clear the display again
        //display.clear();
        if frames > 100000 {
            frames = 1;
        } else {
            frames += 1;
        }
        if b_sleep < 20 {
            b_sleep += 1;
        } else {
            b_sleep = 6;
        }
    }
}
