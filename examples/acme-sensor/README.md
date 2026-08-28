# acme-sensor

An [`embedded-hal`](https://docs.rs/embedded-hal) driver for the **AcmeSensor**,
built on the [`device-driver`](https://docs.rs/device-driver) toolkit.

Scaffolded with the [`drive-rs`](https://github.com/OpenDevicePartnership/drive-rs)
driver template.

## Selected configuration

- Interfaces: `i2c`
- API mode: `both`

## Layout

- `device.ddsl` — the `device-driver` manifest: the single source of truth for the register map.
- `src/registers.rs` — the register map generated from `device.ddsl` (committed; do not edit by hand).
- `src/interface.rs` — bridges an `embedded-hal` bus to the register map.
- `src/driver.rs` — the high-level [`AcmeSensor`] driver with named methods.
- `src/error.rs` — the [`Error`] type.
- `tests/integration.rs` — mock-based tests, no hardware required.
- `examples/pico.rs` — run against real hardware over a Pico de Gallo USB bridge.

## Regenerating the register map

Edit `device.ddsl`, then regenerate the committed Rust module:

```sh
cargo install device-driver-cli   # once
ddc build rust -s device.ddsl -o src/registers.rs --rust-defmt-feature=defmt
```


## Usage

```rust,ignore
use acme_sensor::AcmeSensor;

// Construct on an I2C bus:
let mut dev = AcmeSensor::new_i2c(i2c, 0x00);
let id = dev.device_id()?;
```

## Testing

```sh
cargo test
```

## License

Licensed under either of Apache-2.0 or MIT at your option.
