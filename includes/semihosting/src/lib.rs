#![cfg_attr(feature = "inline-asm", feature(asm))]
#![deny(warnings)]
#![no_std]

#[cfg(feature = "semihosting")]
pub fn _log_hprintln(s: &str) {
    cortex_m_semihosting::export::hstdout_str(s).unwrap()
}

#[cfg(feature = "semihosting")]
pub fn _log_hprintln_fmt(args: core::fmt::Arguments) {
    cortex_m_semihosting::export::hstdout_fmt(args).unwrap()
}

#[cfg(not(feature = "semihosting"))]
#[inline(always)]
pub fn _log_hprintln(_s: &str) {}

#[cfg(not(feature = "semihosting"))]
#[inline(always)]
pub fn _log_hprintln_fmt(_args: core::fmt::Arguments) {}

#[macro_export]
macro_rules! log {
    () => {
        $crate::_log_hprintln("\n")
    };
    ($s:expr) => {
        $crate::_log_hprintln(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::_log_hprintln_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}
