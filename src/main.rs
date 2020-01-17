#![no_main]
#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt as rt;

use rt::{entry, pre_init};
use cortex_m::asm::bkpt;
use semihosting::log;
use embedded_hal::blocking::delay::DelayMs;
use crate::board::Board;

mod board;

pub mod hal {
    #[cfg(feature = "nrf_rt")]
    pub use nrf52840_hal::*;

    #[cfg(feature = "stm32_rt")]
    pub use stm32f1xx_hal::*;
}

pub mod target {
    #[cfg(feature = "nrf_rt")]
    pub use nrf52840_pac::*;
    #[cfg(feature = "stm32_rt")]
    pub use stm32f1xx_hal::pac::*;
}

pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::hal::gpio::GpioExt;
}

#[pre_init]
unsafe fn __init() {}

#[entry]
fn main() -> ! {
    log!("¡Hola!");
    let mut board = Board::new();

    loop {
        board.status_led.toggle();
        board.delay.delay_ms(1_000_u16);
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    bkpt();
    loop {}
}
