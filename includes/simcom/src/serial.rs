use crate::driver::UarteDriver;

pub enum SerialError {
    SerialWrite,
    SerialRead,
}

pub trait SerialDriver {
    fn write(&mut self, buffer: &[u8]) -> Result<(), SerialError>;
}

pub struct Driver<S>(S);

impl<S> Driver<S> where S: UarteDriver {
    pub fn new(serial: S) -> Self {
        Driver(serial)
    }
}

impl<S> SerialDriver for Driver<S> where S: UarteDriver {
    fn write(&mut self, buffer: &[u8]) -> Result<(), SerialError> {
        let buf = &mut [0; 16][..]; // avoiding EasyDMA reading the buffer from the flash
        for block in buffer.chunks(16) {
            buf[..block.len()].copy_from_slice(block);
            self.0.write(&buf[..block.len()]).map_err(|_| SerialError::SerialWrite)?
        }
        Ok(())
    }
}
