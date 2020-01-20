use crate::{
    driver::UarteDriver,
    serial::{
        SerialDriver,
        Driver,
        SerialError,
    },
    hal::gpio::{
        Pin,
        Output,
        PushPull,
    },
};
use embedded_hal::digital::OutputPin;

pub struct Pins {
    pub reset_pin: Pin<Output<PushPull>>,
    pub power_pin: Pin<Output<PushPull>>,
    pub dtr_pin: Pin<Output<PushPull>>,
}

pub struct SimComDriver<S>(Driver<S>, Pins);

const CR: u8 = 0x0d;
const LF: u8 = 0x0a;

impl<S> SimComDriver<S> where S: UarteDriver {
    pub fn new(driver: S, pins: Pins) -> Self {
        SimComDriver(Driver::new(driver), pins)
    }

    pub fn send_command(&mut self, command: &[&str]) -> Result<(), SerialError> {
        for part in command {
            self.0.write(part.as_bytes())?
        }
        self.send_crlf()
    }

    fn send_crlf(&mut self) -> Result<(), SerialError> {
        self.0.write(&[CR, LF])
    }
}
