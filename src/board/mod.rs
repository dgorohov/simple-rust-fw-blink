use embedded_hal::digital::v2::OutputPin;
use crate::board::led::Led;
use crate::hal::delay::Delay;

pub mod led;

#[cfg(feature = "nrf_rt")]
pub mod nrf;

#[cfg(feature = "stm32_rt")]
pub mod stm32_bluepill;

pub struct Board<SL> where SL: OutputPin {
    pub status_led: Led<SL>,
    pub delay: Delay,
}
