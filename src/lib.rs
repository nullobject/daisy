#![deny(warnings)]
#![no_std]

//! Board support crate for Daisy hardware
//!
//! # Usage - see examples/

#[cfg(all(feature = "seed_1_1", feature = "seed_1_0"))]
compile_error!("feature \"seed_1_1\" and feature \"seed_1_0\" cannot be enabled at the same time");

#[cfg(not(any(feature = "seed_1_1", feature = "seed_1_0")))]
compile_error!("target board must be selected using a feature: \"seed_1_1\" | \"seed_1_0\"");

// - modules ------------------------------------------------------------------

pub mod audio;
pub mod board;
pub mod clocks;
pub mod flash;
#[cfg(any(feature = "log-itm"))]
pub mod itm;
pub mod led;
pub mod pins;

// - log macros ---------------------------------------------------------------

#[cfg(any(feature = "log-itm"))]
#[macro_export]
macro_rules! loggit {
    ($($arg:tt)*) => (
        let itm = unsafe { &mut *cortex_m::peripheral::ITM::ptr() };
        cortex_m::iprintln!(&mut itm.stim[0], $($arg)*);
    )
}

#[cfg(not(feature = "log-itm"))]
#[macro_export]
macro_rules! loggit {
    ($($arg:tt)*) => (
        cortex_m_semihosting::hprintln!($($arg)*).unwrap();
    )
}

// - exports ------------------------------------------------------------------

pub use hal::hal as embedded_hal;
pub use hal::pac;
pub use stm32h7xx_hal as hal;

pub use board::Board;
