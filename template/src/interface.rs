{%- assign wants_sync = false -%}
{%- if mode == "sync" or mode == "both" -%}{%- assign wants_sync = true -%}{%- endif -%}
{%- assign wants_async = false -%}
{%- if mode == "async" or mode == "both" -%}{%- assign wants_async = true -%}{%- endif -%}
//! Transport interfaces bridging an `embedded-hal` bus to the `device-driver`
//! register traits.
//!
//! Each register access is a single bus transaction so the device's internal
//! address auto-increment is never disturbed by an intervening STOP.

use crate::error::Error;
{% if interfaces contains "i2c" %}
/// I2C transport for the register map.
///
/// `address` is the device's 7-bit I2C address — do not confuse it with the
/// register address passed to [`device_driver::RegisterInterface`].
pub struct I2cInterface<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> I2cInterface<I2C> {
    /// Create a new I2C transport for the device at 7-bit `address`.
    pub const fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Release the underlying bus.
    pub fn release(self) -> I2C {
        self.i2c
    }
}
{% if wants_sync %}
impl<I2C: embedded_hal::i2c::I2c> device_driver::RegisterInterface for I2cInterface<I2C> {
    type Error = Error<I2C::Error>;
    type AddressType = u8;

    fn write_register(
        &mut self,
        address: u8,
        size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        // The runtime pre-sizes `data`; `size_bits` is only a sanity check.
        debug_assert_eq!(data.len(), size_bits.div_ceil(8) as usize);
        self.i2c
            .transaction(
                self.address,
                &mut [
                    embedded_hal::i2c::Operation::Write(&[address]),
                    embedded_hal::i2c::Operation::Write(data),
                ],
            )
            .map_err(Error::Transport)
    }

    fn read_register(
        &mut self,
        address: u8,
        size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        debug_assert_eq!(data.len(), size_bits.div_ceil(8) as usize);
        // `write_read` is a single repeated-start transaction (no STOP between
        // the register pointer write and the data read).
        self.i2c
            .write_read(self.address, &[address], data)
            .map_err(Error::Transport)
    }
}
{% endif %}{% if wants_async %}
impl<I2C: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface for I2cInterface<I2C> {
    type Error = Error<I2C::Error>;
    type AddressType = u8;

    async fn write_register(
        &mut self,
        address: u8,
        size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        debug_assert_eq!(data.len(), size_bits.div_ceil(8) as usize);
        self.i2c
            .transaction(
                self.address,
                &mut [
                    embedded_hal_async::i2c::Operation::Write(&[address]),
                    embedded_hal_async::i2c::Operation::Write(data),
                ],
            )
            .await
            .map_err(Error::Transport)
    }

    async fn read_register(
        &mut self,
        address: u8,
        size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        debug_assert_eq!(data.len(), size_bits.div_ceil(8) as usize);
        self.i2c
            .write_read(self.address, &[address], data)
            .await
            .map_err(Error::Transport)
    }
}
{% endif %}{% endif %}{% if interfaces contains "spi" %}
/// SPI transport for the register map.
///
/// Uses an `embedded-hal` `SpiDevice`, which owns chip-select: CS is asserted
/// for the whole register transaction and deasserted afterwards, so this type
/// must never also be handed the CS pin as a GPIO.
pub struct SpiInterface<SPI> {
    spi: SPI,
}

/// Command-byte bit that marks a read. **Adjust per datasheet** — the position
/// and polarity of the read/write bit is device-specific.
const SPI_READ_FLAG: u8 = 0x80;
/// Mask selecting the register-address bits of the command byte.
const SPI_ADDR_MASK: u8 = 0x7f;

impl<SPI> SpiInterface<SPI> {
    /// Create a new SPI transport around a CS-managed `SpiDevice`.
    pub const fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Release the underlying bus.
    pub fn release(self) -> SPI {
        self.spi
    }
}
{% if wants_sync %}
impl<SPI: embedded_hal::spi::SpiDevice> device_driver::RegisterInterface for SpiInterface<SPI> {
    type Error = Error<SPI::Error>;
    type AddressType = u8;

    fn write_register(
        &mut self,
        address: u8,
        size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        debug_assert_eq!(data.len(), size_bits.div_ceil(8) as usize);
        self.spi
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(&[address & SPI_ADDR_MASK]),
                embedded_hal::spi::Operation::Write(data),
            ])
            .map_err(Error::Transport)
    }

    fn read_register(
        &mut self,
        address: u8,
        size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        debug_assert_eq!(data.len(), size_bits.div_ceil(8) as usize);
        // Send the command byte, then clock in the data in the same CS window.
        self.spi
            .transaction(&mut [
                embedded_hal::spi::Operation::Write(&[address | SPI_READ_FLAG]),
                embedded_hal::spi::Operation::Read(data),
            ])
            .map_err(Error::Transport)
    }
}
{% endif %}{% if wants_async %}
impl<SPI: embedded_hal_async::spi::SpiDevice> device_driver::AsyncRegisterInterface
    for SpiInterface<SPI>
{
    type Error = Error<SPI::Error>;
    type AddressType = u8;

    // NOTE: register transactions are NOT atomic under future cancellation. If
    // this future is dropped mid-transfer the device may be left partially
    // updated; treat a cancelled call as unknown device state and re-read.
    async fn write_register(
        &mut self,
        address: u8,
        size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        debug_assert_eq!(data.len(), size_bits.div_ceil(8) as usize);
        self.spi
            .transaction(&mut [
                embedded_hal_async::spi::Operation::Write(&[address & SPI_ADDR_MASK]),
                embedded_hal_async::spi::Operation::Write(data),
            ])
            .await
            .map_err(Error::Transport)
    }

    async fn read_register(
        &mut self,
        address: u8,
        size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        debug_assert_eq!(data.len(), size_bits.div_ceil(8) as usize);
        self.spi
            .transaction(&mut [
                embedded_hal_async::spi::Operation::Write(&[address | SPI_READ_FLAG]),
                embedded_hal_async::spi::Operation::Read(data),
            ])
            .await
            .map_err(Error::Transport)
    }
}
{% endif %}{% endif %}