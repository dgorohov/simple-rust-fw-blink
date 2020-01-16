use embedded_hal::digital::*;

pub struct Led<T: v2::OutputPin>(T);

impl<T> Led<T> where T: v2::OutputPin + v2::StatefulOutputPin{
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
        if self.0.is_set_low().unwrap_or(false) {
            self.set_low()
        } else {
            self.set_high()
        }
    }
}
