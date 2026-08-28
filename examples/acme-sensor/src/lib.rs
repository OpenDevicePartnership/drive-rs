#![no_std]
#![doc = concat!("`", env!("CARGO_PKG_NAME"), "` — an embedded-hal driver.")]
//!
//! This crate was scaffolded with the `drive-rs` device-driver template. It is
//! `no_std` and generic over the `embedded-hal` traits, so the same driver runs
//! on real firmware and on a host machine (see `examples/pico.rs`).

pub mod error;
pub use error::Error;

mod driver;
mod interface;
// Generated ahead of time from `device.ddsl` by `ddc`. As
// hand-untouched generated code it is exempt from our lints.
#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[rustfmt::skip]
mod registers;

pub use driver::AcmeSensor;
pub use interface::I2cInterface;
// The low-level, generated register map is exposed for advanced use.
pub use registers::AcmeSensorRegisters;
