{%- assign device = project-name | pascal_case -%}
{%- assign has_bus = false -%}
{%- if interfaces contains "i2c" or interfaces contains "spi" or interfaces contains "uart" -%}{%- assign has_bus = true -%}{%- endif -%}
{%- if has_bus -%}{%- assign pin_type = device | append: "Pins" -%}{%- else -%}{%- assign pin_type = device -%}{%- endif -%}
{%- assign wants_sync = false -%}
{%- if mode == "sync" or mode == "both" -%}{%- assign wants_sync = true -%}{%- endif -%}
{%- assign wants_async = false -%}
{%- if mode == "async" or mode == "both" -%}{%- assign wants_async = true -%}{%- endif -%}
{% if has_bus %}//! Auxiliary discrete GPIO lines for the {{ device }} (reset, interrupt,
//! enable, …). These are separate from the bus transport; in particular the
//! SPI chip-select is owned by the `SpiDevice` and must never be wired here.
{% else %}//! Discrete-pin driver for the {{ device }}.
{% endif %}
use crate::error::Error;

/// A pair of discrete GPIO lines: one output and one input.
///
/// Pins start in whatever state the caller handed them in. Drive them to a
/// known state explicitly (e.g. hold reset, wait `t_reset` per the datasheet,
/// then release) before relying on the device.
pub struct {{ pin_type }}<OUT, IN> {
    output: OUT,
    input: IN,
}

impl<OUT, IN> {{ pin_type }}<OUT, IN> {
    /// Wrap an output pin and an input pin.
    pub const fn new(output: OUT, input: IN) -> Self {
        Self { output, input }
    }

    /// Release the wrapped pins.
    pub fn release(self) -> (OUT, IN) {
        (self.output, self.input)
    }
}
{% if wants_sync %}
impl<OUT: embedded_hal::digital::OutputPin, IN: embedded_hal::digital::InputPin>
    {{ pin_type }}<OUT, IN>
{
    /// Drive the output pin high.
    pub fn set_output_high(&mut self) -> Result<(), Error<OUT::Error>> {
        self.output.set_high().map_err(Error::Transport)
    }

    /// Drive the output pin low.
    pub fn set_output_low(&mut self) -> Result<(), Error<OUT::Error>> {
        self.output.set_low().map_err(Error::Transport)
    }

    /// Read the input pin level.
    pub fn input_is_high(&mut self) -> Result<bool, Error<IN::Error>> {
        self.input.is_high().map_err(Error::Transport)
    }
}
{% endif %}{% if wants_async %}
impl<OUT, IN: embedded_hal_async::digital::Wait> {{ pin_type }}<OUT, IN> {
    /// Wait until the input pin is high.
    pub async fn wait_for_input_high(&mut self) -> Result<(), Error<IN::Error>> {
        self.input.wait_for_high().await.map_err(Error::Transport)
    }

    /// Wait until the input pin is low.
    pub async fn wait_for_input_low(&mut self) -> Result<(), Error<IN::Error>> {
        self.input.wait_for_low().await.map_err(Error::Transport)
    }
}
{% endif %}