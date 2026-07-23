//! High-level, ergonomic driver for the AcmeSensor.
//!
//! This wraps the generated [`AcmeSensorRegisters`] map with named,
//! purpose-built methods. It is generic over the transport interface, so it can
//! be driven by I2C, SPI, or a test mock interchangeably.

use crate::registers::AcmeSensorRegisters;

/// Driver for the AcmeSensor.
pub struct AcmeSensor<I> {
    regs: AcmeSensorRegisters<I>,
}

impl<I> AcmeSensor<I> {
    /// Build the driver from an already-constructed register interface.
    pub fn new(interface: I) -> Self {
        Self {
            regs: AcmeSensorRegisters::new(interface),
        }
    }

    /// Access the low-level register map directly.
    pub fn registers(&mut self) -> &mut AcmeSensorRegisters<I> {
        &mut self.regs
    }
}

impl<I2C> AcmeSensor<crate::interface::I2cInterface<I2C>> {
    /// Build the driver on an I2C bus for the device at 7-bit `address`.
    pub fn new_i2c(i2c: I2C, address: u8) -> Self {
        Self::new(crate::interface::I2cInterface::new(i2c, address))
    }
}

impl<I: device_driver::RegisterInterface<AddressType = u8>> AcmeSensor<I> {
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

impl<I: device_driver::AsyncRegisterInterface<AddressType = u8>> AcmeSensor<I> {
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
