use embedded_hal::serial;
use nb::block;

pub struct SerialDriver<S> {
    serial: S,
    read_buf: [u8; 64],
}

pub enum Error {
    SerialWrite,
    SerialRead,
    ReadBufferTooSmall,
    CommandFailed,
    EncodingError,
}

const CR: u8 = 0x0d;
const LF: u8 = 0x0a;
const OK: [u8; 2] = [b'O', b'K'];

impl<S, E> SerialDriver<S>
    where S: serial::Read<u8, Error=E> + serial::Write<u8, Error=E> {
    fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        block!(self.serial.write(byte)).map_err(|_| Error::SerialWrite)
    }

    fn write_crlf(&mut self) -> Result<(), Error> {
        self.write_byte(CR)?;
        self.write_byte(LF)
    }

    fn write_all(&mut self, buffer: &[u8]) -> Result<(), Error> {
        for byte in buffer {
            self.write_byte(*byte)?;
        }
        Ok(())
    }

    fn read_byte(&mut self) -> Result<u8, Error> {
        block!(self.serial.read()).map_err(|_| Error::SerialRead)
    }

    fn read_line(&mut self) -> Result<&[u8], Error> {
        let buflen = self.read_buf.len();
        let mut i = 0;
        loop {
            match self.read_byte()? {
                LF if self.read_buf[i - 1] == CR => {
                    return Ok(&self.read_buf[0..(i - 1)]);
                }
                other => {
                    self.read_buf[i] = other;
                }
            }
            i += 1;
            if i >= buflen {
                return Err(Error::ReadBufferTooSmall);
            }
        }
    }

    fn send_raw_command(&mut self, command: &[&str]) -> Result<&[u8], Error> {
        for part in command {
            self.write_all(part.as_bytes())?;
        }
        self.write_crlf()?;
        self.read_line()
    }
}
