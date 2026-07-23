{%- assign device = project-name | pascal_case -%}
{%- assign has_bus = false -%}
{%- if interfaces contains "i2c" or interfaces contains "spi" -%}{%- assign has_bus = true -%}{%- endif -%}
{%- assign wants_sync = false -%}
{%- if mode == "sync" or mode == "both" -%}{%- assign wants_sync = true -%}{%- endif -%}
{%- assign wants_async = false -%}
{%- if mode == "async" or mode == "both" -%}{%- assign wants_async = true -%}{%- endif -%}
{% if has_bus %}//! Tests for the {{ device }} driver, driven by an in-memory register mock so
//! no bus or hardware is required.

use {{ crate_name }}::{{ device }};

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
{% if wants_sync %}
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
        data.copy_from_slice(&self.regs[start..start + data.len()]);
        Ok(())
    }
}
{% endif %}{% if wants_async %}
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
{% endif %}{% if wants_sync %}
#[test]
fn sync_reads_id_and_toggles_enable() {
    let mut dev = {{ device }}::new(MockInterface::new());
    assert_eq!(dev.device_id().unwrap(), 0xAB);

    dev.set_enable(true).unwrap();
    assert!(dev.registers().config().read().unwrap().enable());
}
{% endif %}{% if wants_async %}
#[tokio::test]
async fn async_reads_id_and_toggles_enable() {
    let mut dev = {{ device }}::new(MockInterface::new());
    assert_eq!(dev.device_id_async().await.unwrap(), 0xAB);

    dev.set_enable_async(true).await.unwrap();
}
{% endif %}{% else %}//! Tests for the {{ device }} pin driver, driven by mock pins.

use {{ crate_name }}::{{ device }};

/// A fake GPIO pin backed by a single boolean level.
struct MockPin {
    // Only read by the blocking `InputPin` mock; the async `Wait` mock never
    // blocks, so this is unused in async-only builds.
    #[allow(dead_code)]
    level: bool,
}

impl embedded_hal::digital::ErrorType for MockPin {
    type Error = core::convert::Infallible;
}
{% if wants_sync %}
impl embedded_hal::digital::OutputPin for MockPin {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.level = true;
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.level = false;
        Ok(())
    }
}

impl embedded_hal::digital::InputPin for MockPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.level)
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.level)
    }
}
{% endif %}{% if wants_async %}
impl embedded_hal_async::digital::Wait for MockPin {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
{% endif %}{% if wants_sync %}
#[test]
fn sync_drives_and_reads_pins() {
    let mut pins = {{ device }}::new(MockPin { level: false }, MockPin { level: true });
    pins.set_output_high().unwrap();
    assert!(pins.input_is_high().unwrap());
    pins.set_output_low().unwrap();
}
{% endif %}{% if wants_async %}
#[tokio::test]
async fn async_waits_on_input() {
    let mut pins = {{ device }}::new(MockPin { level: false }, MockPin { level: true });
    pins.wait_for_input_high().await.unwrap();
}
{% endif %}{% endif %}