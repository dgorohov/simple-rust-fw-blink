use crate::{
    driver::UarteDriver,
    serial::{
        SerialDriver,
        Driver,
        SerialError,
    },
};

pub struct SimComDriver<S>(Driver<S>);

const CR: u8 = 0x0d;
const LF: u8 = 0x0a;

impl<S> SimComDriver<S> where S: UarteDriver {
    pub fn new(driver: S) -> Self {
        SimComDriver(Driver::new(driver))
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
