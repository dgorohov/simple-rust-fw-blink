#![no_main]
#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#![feature(asm)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(naked_functions)]

#[cfg(feature = "rt")]
extern crate cortex_m_rt as rt;

use rt::{entry, pre_init};
use crate::board::Board;
use cortex_m::asm::bkpt;
use nrf::_nrf_delay_ms;
use semihosting::log;

use nrf52840_hal as hal;
use nrf52840_pac as target;

pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::hal::gpio::GpioExt;
}

mod board;

#[pre_init]
unsafe fn __init() {}

#[entry]
fn main() -> ! {
    log!("¡Hola!");
    let mut board = Board::new();

    loop {
        board.led0.toggle();
        unsafe {
            _nrf_delay_ms(200);
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    bkpt();
    loop {}
}
