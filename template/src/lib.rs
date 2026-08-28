{%- assign device = project-name | pascal_case -%}
{%- assign has_bus = false -%}
{%- if interfaces contains "i2c" or interfaces contains "spi" or interfaces contains "uart" -%}{%- assign has_bus = true -%}{%- endif -%}
#![no_std]
#![doc = concat!("`", env!("CARGO_PKG_NAME"), "` — an embedded-hal driver.")]
//!
//! This crate was scaffolded with the `drive-rs` device-driver template. It is
//! `no_std` and generic over the `embedded-hal` traits, so the same driver runs
//! on real firmware and on a host machine (see `examples/pico.rs`).

pub mod error;
pub use error::Error;
{% if has_bus %}
mod driver;
mod interface;
// Generated ahead of time from `device.ddsl` by `ddc`. As
// hand-untouched generated code it is exempt from our lints.
#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
#[rustfmt::skip]
mod registers;

pub use driver::{{ device }};
{% if interfaces contains "i2c" %}pub use interface::I2cInterface;
{% endif %}{% if interfaces contains "spi" %}pub use interface::SpiInterface;
{% endif %}{% if interfaces contains "uart" %}pub use interface::UartInterface;
{% endif %}// The low-level, generated register map is exposed for advanced use.
pub use registers::{{ device }}Registers;
{% endif %}{% if interfaces contains "gpio" %}
mod gpio;
pub use gpio::{% if has_bus %}{{ device }}Pins{% else %}{{ device }}{% endif %};
{% endif %}
