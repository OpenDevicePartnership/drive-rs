//! Transport interfaces bridging an `embedded-hal` bus to the `device-driver`
//! register traits.
//!
//! Each register access is a single bus transaction so the device's internal
//! address auto-increment is never disturbed by an intervening STOP.

use crate::error::Error;

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

impl<I2C: embedded_hal::i2c::ErrorType> device_driver::RegisterInterfaceBase for I2cInterface<I2C> {
    type Error = Error<I2C::Error>;
    type AddressType = u8;
}

impl<I2C: embedded_hal::i2c::I2c> device_driver::RegisterInterface for I2cInterface<I2C> {
    fn write_register(
        &mut self,
        address: u8,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
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
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        // `write_read` is a single repeated-start transaction (no STOP between
        // the register pointer write and the data read).
        self.i2c
            .write_read(self.address, &[address], data)
            .map_err(Error::Transport)
    }
}

impl<I2C: embedded_hal_async::i2c::I2c> device_driver::AsyncRegisterInterface
    for I2cInterface<I2C>
{
    async fn write_register(
        &mut self,
        address: u8,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
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
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(self.address, &[address], data)
            .await
            .map_err(Error::Transport)
    }
}
