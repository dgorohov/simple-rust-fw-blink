#![no_std]

pub mod driver {
    pub use uarte::uart::UarteDriver;
}

pub use nrf52840_hal as hal;
pub use nrf52840_pac as target;

pub mod serial;
pub mod simcom;
