#[cfg(feature = "nrf_rt")]
pub use nrf52_hal_common::*;

#[cfg(feature = "stm32_rt")]
pub use stm32f1xx_hal::*;
