{%- assign device = project-name | pascal_case -%}
{%- assign has_bus = false -%}
{%- if interfaces contains "i2c" or interfaces contains "spi" -%}{%- assign has_bus = true -%}{%- endif -%}
{%- assign example_async = false -%}
{%- if mode == "async" -%}{%- assign example_async = true -%}{%- endif -%}
//! Run the {{ device }} driver on your laptop against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! ```text
//! cargo run --example pico
//! ```

use pico_de_gallo_hal::Hal;
use {{ crate_name }}::{{ device }};
{% if example_async %}
#[tokio::main]
async fn main() {
    let hal = Hal::new();
{% if interfaces contains "i2c" %}    // The device responds at 7-bit I2C address 0x00 — change to match yours.
    let mut dev = {{ device }}::new_i2c(hal.i2c(), 0x00);
    match dev.device_id_async().await {
        Ok(id) => println!("{{ device }} id: {id:#04x}"),
        Err(e) => eprintln!("error talking to the device: {e}"),
    }
{% elsif interfaces contains "spi" %}    // Chip-select is driven on GPIO 0 — change to match your wiring.
    let spi = hal.spi_device(0).expect("failed to open SPI device");
    let mut dev = {{ device }}::new_spi(spi);
    match dev.device_id_async().await {
        Ok(id) => println!("{{ device }} id: {id:#04x}"),
        Err(e) => eprintln!("error talking to the device: {e}"),
    }
{% else %}    // Output on GPIO 0, input on GPIO 1 — change to match your wiring.
    let mut dev = {{ device }}::new(hal.gpio(0), hal.gpio(1));
    dev.wait_for_input_high().await.expect("wait failed");
    println!("input went high");
{% endif %}}
{% else %}
fn main() {
    let hal = Hal::new();
{% if interfaces contains "i2c" %}    // The device responds at 7-bit I2C address 0x00 — change to match yours.
    let mut dev = {{ device }}::new_i2c(hal.i2c(), 0x00);
    match dev.device_id() {
        Ok(id) => println!("{{ device }} id: {id:#04x}"),
        Err(e) => eprintln!("error talking to the device: {e}"),
    }
{% elsif interfaces contains "spi" %}    // Chip-select is driven on GPIO 0 — change to match your wiring.
    let spi = hal.spi_device(0).expect("failed to open SPI device");
    let mut dev = {{ device }}::new_spi(spi);
    match dev.device_id() {
        Ok(id) => println!("{{ device }} id: {id:#04x}"),
        Err(e) => eprintln!("error talking to the device: {e}"),
    }
{% else %}    // Output on GPIO 0, input on GPIO 1 — change to match your wiring.
    let mut dev = {{ device }}::new(hal.gpio(0), hal.gpio(1));
    dev.set_output_high().expect("set high failed");
    println!("input high? {}", dev.input_is_high().expect("read failed"));
{% endif %}}
{% endif %}