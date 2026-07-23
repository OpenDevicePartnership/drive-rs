//! Tests for the AcmeSensor driver, driven by an in-memory register mock so
//! no bus or hardware is required.

use acme_sensor::AcmeSensor;

/// A fake device: a flat 256-byte register file. `DeviceId` (0x00) is preset.
struct MockInterface {
    regs: [u8; 256],
}

impl MockInterface {
    fn new() -> Self {
        let mut regs = [0u8; 256];
        regs[0x00] = 0xAB; // DeviceId
        Self { regs }
    }
}

impl device_driver::RegisterInterface for MockInterface {
    type Error = core::convert::Infallible;
    type AddressType = u8;

    fn write_register(&mut self, address: u8, _bits: u32, data: &[u8]) -> Result<(), Self::Error> {
        let start = address as usize;
        self.regs[start..start + data.len()].copy_from_slice(data);
        Ok(())
    }

    fn read_register(&mut self, address: u8, _bits: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        let start = address as usize;
        assert!(start + data.len() <= self.regs.len(), "register read out of bounds");
        data.copy_from_slice(&self.regs[start..start + data.len()]);
        Ok(())
    }
}

impl device_driver::AsyncRegisterInterface for MockInterface {
    type Error = core::convert::Infallible;
    type AddressType = u8;

    async fn write_register(&mut self, address: u8, _bits: u32, data: &[u8]) -> Result<(), Self::Error> {
        let start = address as usize;
        assert!(start + data.len() <= self.regs.len(), "register write out of bounds");
        self.regs[start..start + data.len()].copy_from_slice(data);
        Ok(())
    }

    async fn read_register(&mut self, address: u8, _bits: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        let start = address as usize;
        data.copy_from_slice(&self.regs[start..start + data.len()]);
        Ok(())
    }
}

#[test]
fn sync_reads_id_and_toggles_enable() {
    let mut dev = AcmeSensor::new(MockInterface::new());
    assert_eq!(dev.device_id().unwrap(), 0xAB);

    dev.set_enable(true).unwrap();
    assert!(dev.registers().config().read().unwrap().enable());
}

#[tokio::test]
async fn async_reads_id_and_toggles_enable() {
    let mut dev = AcmeSensor::new(MockInterface::new());
    assert_eq!(dev.device_id_async().await.unwrap(), 0xAB);

    dev.set_enable_async(true).await.unwrap();
}
