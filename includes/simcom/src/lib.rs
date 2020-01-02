#![no_std]

pub mod serial;
pub mod simcom;

pub mod driver {
    pub use uarte::uart::UarteDriver;
}
