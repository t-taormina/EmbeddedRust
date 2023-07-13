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
        pac::TIMER1,
        timer::Timer,
    },
};

use nanorand::{pcg64::Pcg64, Rng, SeedableRng};
use panic_halt as _;
mod life;
use life::*;

#[entry]
fn main() -> ! {

    let mut led: [[u8; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
    ];

    let mut rng = nanorand::Pcg64::new_seed(1);
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let b: bool = rng.generate();

    loop {
    }
}

/*
const LET_T: [[u8; 5]; 5] = [
        [1, 1, 1, 1, 1],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ];

const LET_A: [[u8; 5]; 5] = [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
    ];

const LET_U: [[u8; 5]; 5] = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];

const LET_R: [[u8; 5]; 5] = [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ];

const LET_Y: [[u8; 5]; 5] = [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ];

const NAME: [[[u8; 5]; 5]; 5] = [LET_T, LET_A, LET_U, LET_R, LET_Y];
*/
