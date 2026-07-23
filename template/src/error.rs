//! Error types for the {{ project-name | pascal_case }} driver.

/// Errors returned by the {{ project-name | pascal_case }} driver.
///
/// The generic parameter `E` is the error type of the underlying transport:
/// the `embedded-hal` I2C/SPI bus error, or a GPIO pin error. It is preserved
/// (never stringified or discarded) so callers can distinguish, for example, a
/// `NoAcknowledge` from a transient bus glitch and choose to retry or re-init.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// An error from the underlying transport (I2C/SPI bus or GPIO pin).
    Transport(E),
}

impl<E: core::fmt::Debug> core::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Transport(e) => write!(f, "transport error: {e:?}"),
        }
    }
}

impl<E: core::fmt::Debug> core::error::Error for Error<E> {}
