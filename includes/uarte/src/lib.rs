#![no_std]

use nrf52840_hal as hal;
use nrf52840_pac as target;

pub mod prelude {
    pub use crate::hal::prelude::*;
}

pub mod constants {
    pub use nrf52_hal_common::target_constants::*;
}

pub mod pins {
    pub use nrf52_hal_common::uarte::Pins;
}

pub mod uart;

pub(crate) fn slice_in_ram(slice: &[u8]) -> bool {
    let ptr = slice.as_ptr() as usize;
    ptr >= constants::SRAM_LOWER &&
        (ptr + slice.len()) < constants::SRAM_UPPER
}
