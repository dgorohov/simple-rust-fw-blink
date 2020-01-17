use embedded_hal::digital::v2::{
    OutputPin,
    StatefulOutputPin,
};

pub struct Led<T: OutputPin>(T);

impl<T> Led<T> where T: OutputPin + StatefulOutputPin {
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
        match self.0.is_set_low() {
            Ok(false) => self.set_low(),
            Ok(true) => self.set_high(),
            Err(_) => (),
        }
    }
}
