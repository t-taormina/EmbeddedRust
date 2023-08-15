//! Use microbit buttons for paddle control

use crate::*;

use microbit::board::Buttons;
use microbit::hal::gpio::{p0::P0_14, p0::P0_23, Input, PullUp};

/// Material needed for doing button reads.
pub struct Inputs {
    button_a: P0_14<Input<PullUp>>,
    button_b: P0_23<Input<PullUp>>,
    value: f32,
}

impl Inputs {
    /// Set up to read buttons.
    pub fn new(buttons: Buttons) -> Self {
        Self {
            button_a: buttons.button_a.into_pullup_input(),
            button_b: buttons.button_b.into_pullup_input(),
            value: 0.5,
        }
    }

    /// Read the buttons. Returns `Some` fraction 0.0..1.0 of
    /// knob rotation if the knob is 0.3..0.7 full voltage,
    /// clamping on the left and right. However, returns
    /// `None` if the knob is less than 0.1 full voltage.
    pub fn read(&mut self) -> Option<f32> {
        let (ba, bb) = (
            self.button_a.is_low().unwrap(),
            self.button_b.is_low().unwrap(),
        );
        match (ba, bb) {
            (true, _) => self.value -= 0.1,
            (_, true) => self.value += 0.1,
            (false, false) => self.value = self.value,
        }
        if self.value == 0.5 {
            None
        } else {
            self.value = self.value.clamp(0.0, 1.0);
            Some(self.value)
        }
    }
}
