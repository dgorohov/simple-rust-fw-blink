use embedded_hal::digital::v2::OutputPin;

pub mod led;

#[cfg(feature = "nrf_rt")]
pub mod nrf;

#[cfg(feature = "stm32_rt")]
pub mod stm32;

use crate::board::led::Led;
use stm32f1xx_hal::delay::Delay;

pub struct Board<SL> where SL: OutputPin {
    pub status_led: Led<SL>,
    pub delay: Delay
}
