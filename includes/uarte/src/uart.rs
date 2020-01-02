use core::{
    sync::atomic::{
        compiler_fence,
        Ordering::SeqCst,
    },
    ops::Deref,
};
use crate::{
    hal::{
        prelude::*,
        uarte::{
            Baudrate as UartBaudrate,
            Parity as UartParity,
        },
    }, target::{
        self,
        uarte0,
        UARTE1,
        UARTE0,
    },
    pins::Pins,
    constants::*,
    slice_in_ram,
};

pub struct UartConfig {
    pub parity: UartParity,
    pub baudrate: UartBaudrate,
}

impl Default for UartConfig {
    fn default() -> Self {
        UartConfig {
            parity: UartParity::EXCLUDED,
            baudrate: UartBaudrate::BAUD115200,
        }
    }
}

pub fn new_uart0(p: target::Peripherals, pins: Pins, cfg: UartConfig) -> UarteDma<target::UARTE0> {
    p.UARTE0.constrain(pins, cfg.parity, cfg.baudrate)
}

pub fn new_uart1(p: target::Peripherals, pins: Pins, cfg: UartConfig) -> UarteDma<target::UARTE1> {
    p.UARTE1.constrain(pins, cfg.parity, cfg.baudrate)
}

pub trait UarteExt: Deref<Target=uarte0::RegisterBlock> + Sized {
    fn constrain(self, pins: Pins, parity: UartParity, baudrate: UartBaudrate) -> UarteDma<Self>;
}

impl UarteExt for UARTE0 {
    fn constrain(self, pins: Pins, parity: UartParity, baudrate: UartBaudrate) -> UarteDma<Self> {
        UarteDma::new(self, pins, parity, baudrate)
    }
}

impl UarteExt for UARTE1 {
    fn constrain(self, pins: Pins, parity: UartParity, baudrate: UartBaudrate) -> UarteDma<Self> {
        UarteDma::new(self, pins, parity, baudrate)
    }
}

pub trait UarteDriver {
    fn write(&mut self, tx_buffer: &[u8]) -> Result<(), Error>;
    fn read(&mut self, rx_buffer: &mut [u8]) -> Result<(), Error>;
}

pub struct UarteDma<T>(T);

impl<T> UarteDma<T> where T: UarteExt {
    pub fn new(uarte: T, mut pins: Pins, parity: UartParity, baudrate: UartBaudrate) -> Self {
        // Select pins
        uarte.psel.rxd.write(|w| {
            let w = w.port().bit(pins.rxd.port);
            w.connect().connected()
        });
        pins.txd.set_high();
        uarte.psel.txd.write(|w| {
            let w = w.port().bit(pins.txd.port);
            w.connect().connected()
        });

        // Optional pins
        uarte.psel.cts.write(|w| {
            if let Some(ref pin) = pins.cts {
                let w = w.port().bit(pin.port);
                w.connect().connected()
            } else {
                w.connect().disconnected()
            }
        });

        uarte.psel.rts.write(|w| {
            if let Some(ref pin) = pins.rts {
                let w = w.port().bit(pin.port);
                w.connect().connected()
            } else {
                w.connect().disconnected()
            }
        });

        // Enable UARTE instance
        uarte.enable.write(|w|
            w.enable().enabled()
        );

        // Configure
        let hardware_flow_control = pins.rts.is_some() && pins.cts.is_some();
        uarte.config.write(|w|
            w.hwfc().bit(hardware_flow_control)
                .parity().variant(parity)
        );

        // Configure frequency
        uarte.baudrate.write(|w|
            w.baudrate().variant(baudrate)
        );

        UarteDma(uarte)
    }

    fn start_read(&mut self, rx_buffer: &mut [u8]) -> Result<(), Error> {
        if rx_buffer.len() > EASY_DMA_SIZE {
            return Err(Error::RxBufferTooLong);
        }

        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // before any DMA action has started
        compiler_fence(SeqCst);

        // Set up the DMA read
        self.0.rxd.ptr.write(|w|
            // We're giving the register a pointer to the stack. Since we're
            // waiting for the UARTE transaction to end before this stack pointer
            // becomes invalid, there's nothing wrong here.
            //
            // The PTR field is a full 32 bits wide and accepts the full range
            // of values.
            unsafe { w.ptr().bits(rx_buffer.as_ptr() as u32) }
        );
        self.0.rxd.maxcnt.write(|w|
            // We're giving it the length of the buffer, so no danger of
            // accessing invalid memory. We have verified that the length of the
            // buffer fits in an `u8`, so the cast to `u8` is also fine.
            //
            // The MAXCNT field is at least 8 bits wide and accepts the full
            // range of values.
            unsafe { w.maxcnt().bits(rx_buffer.len() as _) });

        // Start UARTE Receive transaction
        self.0.tasks_startrx.write(|w|
            // `1` is a valid value to write to task registers.
            unsafe { w.bits(1) });

        Ok(())
    }

    /// Finalize a UARTE read transaction by clearing the event
    fn finalize_read(&mut self) {
        // Reset the event, otherwise it will always read `1` from now on.
        self.0.events_endrx.write(|w| w);

        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // after all possible DMA actions have completed
        compiler_fence(SeqCst);
    }

    /// Return the raw interface to the underlying UARTE peripheral
    pub fn free(self) -> T {
        self.0
    }
}

impl<T> UarteDriver for UarteDma<T> where T: UarteExt {
    fn write(&mut self, tx_buffer: &[u8]) -> Result<(), Error>
    {
        if !slice_in_ram(tx_buffer) {
            return Err(Error::DMABufferNotInDataMemory);
        }

        if tx_buffer.len() > EASY_DMA_SIZE {
            return Err(Error::TxBufferTooLong);
        }

        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // before any DMA action has started
        compiler_fence(SeqCst);

        // Set up the DMA write
        self.0.txd.ptr.write(|w|
            // We're giving the register a pointer to the stack. Since we're
            // waiting for the UARTE transaction to end before this stack pointer
            // becomes invalid, there's nothing wrong here.
            //
            // The PTR field is a full 32 bits wide and accepts the full range
            // of values.
            unsafe { w.ptr().bits(tx_buffer.as_ptr() as u32) }
        );
        self.0.txd.maxcnt.write(|w|
            // We're giving it the length of the buffer, so no danger of
            // accessing invalid memory. We have verified that the length of the
            // buffer fits in an `u8`, so the cast to `u8` is also fine.
            //
            // The MAXCNT field is 8 bits wide and accepts the full range of
            // values.
            unsafe { w.maxcnt().bits(tx_buffer.len() as _) });

        // Start UARTE Transmit transaction
        self.0.tasks_starttx.write(|w|
            // `1` is a valid value to write to task registers.
            unsafe { w.bits(1) });

        // Wait for transmission to end
        while self.0.events_endtx.read().bits() == 0 {}

        // Reset the event, otherwise it will always read `1` from now on.
        self.0.events_endtx.write(|w| w);

        // Conservative compiler fence to prevent optimizations that do not
        // take in to account actions by DMA. The fence has been placed here,
        // after all possible DMA actions have completed
        compiler_fence(SeqCst);

        if self.0.txd.amount.read().bits() != tx_buffer.len() as u32 {
            return Err(Error::Transmit);
        }

        Ok(())
    }

    fn read(&mut self, rx_buffer: &mut [u8]) -> Result<(), Error>
    {
        self.start_read(rx_buffer)?;

        // Wait for transmission to end
        while self.0.events_endrx.read().bits() == 0 {}

        self.finalize_read();

        if self.0.rxd.amount.read().bits() != rx_buffer.len() as u32 {
            return Err(Error::Receive);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    TxBufferTooLong,
    RxBufferTooLong,
    Transmit,
    Receive,
    DMABufferNotInDataMemory,
}
