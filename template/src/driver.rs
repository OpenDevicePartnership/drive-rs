{%- assign device = project-name | pascal_case -%}
{%- assign wants_sync = false -%}
{%- if mode == "sync" or mode == "both" -%}{%- assign wants_sync = true -%}{%- endif -%}
{%- assign wants_async = false -%}
{%- if mode == "async" or mode == "both" -%}{%- assign wants_async = true -%}{%- endif -%}
//! High-level, ergonomic driver for the {{ device }}.
//!
//! This wraps the generated [`{{ device }}Registers`] map with named,
//! purpose-built methods. It is generic over the transport interface, so it can
//! be driven by I2C, SPI, or a test mock interchangeably.

use crate::registers::{{ device }}Registers;

/// Driver for the {{ device }}.
pub struct {{ device }}<I> {
    regs: {{ device }}Registers<I>,
}

impl<I> {{ device }}<I> {
    /// Build the driver from an already-constructed register interface.
    pub fn new(interface: I) -> Self {
        Self {
            regs: {{ device }}Registers::new(interface),
        }
    }

    /// Access the low-level register map directly.
    pub fn registers(&mut self) -> &mut {{ device }}Registers<I> {
        &mut self.regs
    }
}
{% if interfaces contains "i2c" %}
impl<I2C> {{ device }}<crate::interface::I2cInterface<I2C>> {
    /// Build the driver on an I2C bus for the device at 7-bit `address`.
    pub fn new_i2c(i2c: I2C, address: u8) -> Self {
        Self::new(crate::interface::I2cInterface::new(i2c, address))
    }
}
{% endif %}{% if interfaces contains "spi" %}
impl<SPI> {{ device }}<crate::interface::SpiInterface<SPI>> {
    /// Build the driver on a CS-managed SPI device.
    pub fn new_spi(spi: SPI) -> Self {
        Self::new(crate::interface::SpiInterface::new(spi))
    }
}
{% endif %}{% if wants_sync %}
impl<I: device_driver::RegisterInterface<AddressType = u8>> {{ device }}<I> {
    /// Read the fixed device identifier.
    pub fn device_id(&mut self) -> Result<u8, I::Error> {
        Ok(self.regs.device_id().read()?.id())
    }

    /// Enable or disable the device.
    pub fn set_enable(&mut self, enable: bool) -> Result<(), I::Error> {
        self.regs.config().modify(|reg| {
            reg.set_enable(enable);
        })
    }
}
{% endif %}{% if wants_async %}
impl<I: device_driver::AsyncRegisterInterface<AddressType = u8>> {{ device }}<I> {
    /// Read the fixed device identifier.
    pub async fn device_id_async(&mut self) -> Result<u8, I::Error> {
        Ok(self.regs.device_id().read_async().await?.id())
    }

    /// Enable or disable the device.
    pub async fn set_enable_async(&mut self, enable: bool) -> Result<(), I::Error> {
        self.regs
            .config()
            .modify_async(|reg| {
                reg.set_enable(enable);
            })
            .await
    }
}
{% endif %}