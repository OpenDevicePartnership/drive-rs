{%- assign device = project-name | pascal_case -%}
{%- assign has_bus = false -%}
{%- if interfaces contains "i2c" or interfaces contains "spi" -%}{%- assign has_bus = true -%}{%- endif -%}
# {{ project-name }}

An [`embedded-hal`](https://docs.rs/embedded-hal) driver for the **{{ device }}**,
built on the [`device-driver`](https://docs.rs/device-driver) toolkit.

Scaffolded with the [`drive-rs`](https://github.com/OpenDevicePartnership/drive-rs)
driver template.

## Selected configuration

- Interfaces: `{{ interfaces }}`
- API mode: `{{ mode }}`

## Layout

{% if has_bus %}- `device.yaml` — the `device-driver` manifest: the single source of truth for the register map.
- `src/registers.rs` — the register map generated from `device.yaml` (committed; do not edit by hand).
- `src/interface.rs` — bridges an `embedded-hal` bus to the register map.
- `src/driver.rs` — the high-level [`{{ device }}`] driver with named methods.
{% endif %}{% if interfaces contains "gpio" %}- `src/gpio.rs` — discrete GPIO line handling.
{% endif %}- `src/error.rs` — the [`Error`] type.
- `tests/integration.rs` — mock-based tests, no hardware required.
- `examples/pico.rs` — run against real hardware over a Pico de Gallo USB bridge.
{% if has_bus %}
## Regenerating the register map

Edit `device.yaml`, then regenerate the committed Rust module:

```sh
cargo install device-driver-cli   # once
device-driver-cli -m device.yaml -o src/registers.rs -d {{ device }}Registers
```
{% endif %}

## Usage

```rust,ignore
use {{ crate_name }}::{{ device }};
{% if interfaces contains "i2c" %}
// Construct on an I2C bus:
let mut dev = {{ device }}::new_i2c(i2c, 0x00);
{% if mode == "async" %}let id = dev.device_id_async().await?;{% else %}let id = dev.device_id()?;{% endif %}
{% elsif interfaces contains "spi" %}
// Construct on an SPI device (chip-select managed for you):
let mut dev = {{ device }}::new_spi(spi);
{% if mode == "async" %}let id = dev.device_id_async().await?;{% else %}let id = dev.device_id()?;{% endif %}
{% else %}
// Construct from an output pin and an input pin:
let mut dev = {{ device }}::new(output_pin, input_pin);
{% endif %}```

## Testing

```sh
cargo test
```

## License

Licensed under either of Apache-2.0 or MIT at your option.
