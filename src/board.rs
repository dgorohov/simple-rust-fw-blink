use crate::hal::{
    gpio::{
        Output, Pin, PushPull, Level,
    },
};

use crate::prelude::*;
use crate::target::Peripherals;

#[allow(deprecated)]
use embedded_hal::digital::StatefulOutputPin;

pub struct Led(Pin<Output<PushPull>>);

impl Led {
    pub fn new(pin: Pin<Output<PushPull>>) -> Self {
        Led(pin.into_push_pull_output(Level::High))
    }

    #[allow(deprecated)]
    pub fn toggle(&mut self) {
        if self.0.is_set_low() {
            self.0.set_high();
        } else {
            self.0.set_low();
        }
    }
}

pub struct Board {
    pub led0: Led,
}

impl Board {
    pub fn new() -> Self {
        let p = Peripherals::take().unwrap();
        let port0 = p.P0.split();
        let led0 = port0.p0_24.into_push_pull_output(Level::Low).degrade();
        Board {
            led0: Led::new(led0),
        }
    }
}
