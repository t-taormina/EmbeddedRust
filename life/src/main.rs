//   Tyler Taormina
//   taormina@pdx.edu

//   Game of Life on Microbit-V2

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, timer::Timer},
};

use panic_halt as _;
mod life;
use life::*;

enum State {
    RUNNING,
    DONE(Frames),
    COMPLEMENT,
    RANDOM,
}

enum Frames {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
}

fn decrement(frame: Frames) -> Frames {
    match frame {
        Frames::FIVE => Frames::FOUR,
        Frames::FOUR => Frames::THREE,
        Frames::THREE => Frames::TWO,
        Frames::TWO => Frames::ONE,
        _ => Frames::ZERO,
    }
}

struct StateMachine {
    state: State,
    random: u128,
    led: [[u8; 5]; 5],
    complement_sleep: u8,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            state: State::RANDOM,
            random: 3,
            led: [[0; 5]; 5],
            complement_sleep: 0,
        }
    }

    pub fn check_counters(&mut self) {
        if self.random > 100000 {
            self.random = 1;
        } else {
            self.random += 3;
        }
        if self.complement_sleep < 6 {
            self.complement_sleep += 1;
        }
    }
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER1);
    let button_a = board.buttons.button_a.into_pullup_input();
    let button_b = board.buttons.button_b.into_pullup_input();

    let mut state_machine = StateMachine::new();

    loop {
        let (ba, bb) = (button_a.is_low().unwrap(), button_b.is_low().unwrap());
        match (ba, bb) {
            (true, _) => state_machine.state = State::RANDOM,
            (_, true) => state_machine.state = State::COMPLEMENT,
            (false, false) => state_machine.state = state_machine.state,
        }

        state_machine.state = match state_machine.state {
            State::RANDOM => {
                random(&mut state_machine.led, state_machine.random);
                State::RUNNING
            }
            State::RUNNING => {
                if done(&mut state_machine.led) {
                    State::DONE(Frames::FIVE)
                } else {
                    life(&mut state_machine.led);
                    State::RUNNING
                }
            }
            State::DONE(fr) => match fr {
                Frames::ZERO => State::RANDOM,
                _ => State::DONE(decrement(fr)),
            },
            State::COMPLEMENT => {
                if state_machine.complement_sleep > 5 {
                    complement(&mut state_machine.led);
                    state_machine.complement_sleep = 0;
                }
                State::RUNNING
            }
        };

        display.show(&mut timer, state_machine.led, 100);
        state_machine.check_counters();
    }
}
