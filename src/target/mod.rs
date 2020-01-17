#[cfg(feature = "nrf_rt")]
pub use nrf52840_pac::*;

#[cfg(feature = "stm32_rt")]
pub use stm32f1xx_hal::pac::*;
