use embedded_hal::digital::*;
use semihosting::log;
pub struct Led<T: v2::OutputPin>(T);

impl<T> Led<T> where T: v2::OutputPin + v2::StatefulOutputPin {
    pub fn new(pin: T) -> Self {
        Led(pin)
    }

    fn set_low(&mut self) {
        self.0.set_low().unwrap_or(())
    }

    fn set_high(&mut self) {
        self.0.set_high().unwrap_or(())
    }

    pub fn toggle(&mut self) {
        log!("--> toggle");
        match self.0.is_set_low() {
            Ok(false) => self.set_low(),
            Ok(true) => self.set_high(),
            Err(_) => (),
        }
    }
}
