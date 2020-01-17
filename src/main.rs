#![no_main]
#![no_std]

#[cfg(feature = "rt")]
extern crate cortex_m_rt as rt;
mod board;
mod hal;
mod target;

use rt::{entry, pre_init};
use cortex_m::asm::bkpt;
use semihosting::log;
use embedded_hal::blocking::delay::DelayMs;
use crate::board::Board;

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
