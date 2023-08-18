//! Use microbit buttons for paddle control

use crate::*;

use microbit::board::Buttons;
use microbit::hal::gpio::{p0::P0_14, p0::P0_23, Input, PullUp};

/// Material needed for doing button reads.
pub struct Button {
    button_a: P0_14<Input<PullUp>>,
    button_b: P0_23<Input<PullUp>>,
    value: f32,
}

impl Button {
    /// Set up to read buttons on board.
    pub fn new(buttons: Buttons) -> Self {
        Self {
            button_a: buttons.button_a.into_pullup_input(),
            button_b: buttons.button_b.into_pullup_input(),
            value: -0.1,
        }
    }

    /// Read the buttons. Returns `Some` fraction 0.0..1.0.
    /// However returns `None` if the button value is set to default
    /// of -0.1. Button <a> presses decrease self.value and button <b>
    /// presses increase self.value
    pub fn read(&mut self) -> Option<f32> {
        let (ba, bb) = (
            self.button_a.is_low().unwrap(),
            self.button_b.is_low().unwrap(),
        );
        match (ba, bb) {
            (true, _) => {
                if self.value < 0.0 {
                    self.value = 0.4
                } else {
                    self.value -= 0.08;
                }
            }
            (_, true) => {
                if self.value < 0.0 {
                    self.value = 0.6
                } else {
                    self.value += 0.08;
                }
            }
            (false, false) => self.value = self.value,
        }
        if self.value < 0.0 {
            None
        } else {
            self.value = self.value.clamp(0.0, 1.0);
            Some(self.value)
        }
    }
}
