//! Run the AcmeSensor driver on your laptop against real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! ```text
//! cargo run --example pico
//! ```

use acme_sensor::AcmeSensor;
use pico_de_gallo_hal::Hal;

fn main() {
    let hal = Hal::new();
    // The device responds at 7-bit I2C address 0x00 — change to match yours.
    let mut dev = AcmeSensor::new_i2c(hal.i2c(), 0x00);
    match dev.device_id() {
        Ok(id) => println!("AcmeSensor id: {id:#04x}"),
        Err(e) => eprintln!("error talking to the device: {e}"),
    }
}
