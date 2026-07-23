/// Root block of the AcmeSensorRegisters driver
#[derive(Debug)]
pub struct AcmeSensorRegisters<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u8,
}
impl<I> AcmeSensorRegisters<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self { interface, base_address: 0 }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.device_id().read()?;
        callback(0 + 0 * 0, "device_id", reg.into());
        let reg = self.config().read()?;
        callback(1 + 0 * 0, "config", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.device_id().read_async().await?;
        callback(0 + 0 * 0, "device_id", reg.into());
        let reg = self.config().read_async().await?;
        callback(1 + 0 * 0, "config", reg.into());
        Ok(())
    }
    /// Read-only device identification register.
    pub fn device_id(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::DeviceId,
        ::device_driver::RO,
    > {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::DeviceId,
            ::device_driver::RO,
        >::new(self.interface(), address as u8, field_sets::DeviceId::new)
    }
    /// Primary configuration register.
    pub fn config(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        I,
        u8,
        field_sets::Config,
        ::device_driver::RW,
    > {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<
            '_,
            I,
            u8,
            field_sets::Config,
            ::device_driver::RW,
        >::new(self.interface(), address as u8, field_sets::Config::new)
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    #[allow(unused_imports)]
    use super::*;
    /// Read-only device identification register.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DeviceId {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for DeviceId {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DeviceId {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `id` field of the register.
        ///
        /// Fixed identifier reported by the device.
        pub fn id(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 8)
            };
            raw
        }
        ///Write the `id` field of the register.
        ///
        /// Fixed identifier reported by the device.
        pub fn set_id(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 8, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for DeviceId {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<DeviceId> for [u8; 1] {
        fn from(val: DeviceId) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for DeviceId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("DeviceId");
            d.field("id", &self.id());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DeviceId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DeviceId {{ ");
            defmt::write!(f, "id: {=u8}, ", & self.id());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for DeviceId {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for DeviceId {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for DeviceId {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for DeviceId {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for DeviceId {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for DeviceId {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for DeviceId {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Primary configuration register.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for Config {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Config {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `enable` field of the register.
        ///
        /// Enable the device.
        pub fn enable(&self) -> bool {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 0, 1)
            };
            raw > 0
        }
        ///Read the `mode` field of the register.
        ///
        /// Operating mode (0..=3).
        pub fn mode(&self) -> u8 {
            let raw = unsafe {
                ::device_driver::ops::load_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(&self.bits, 1, 3)
            };
            raw
        }
        ///Write the `enable` field of the register.
        ///
        /// Enable the device.
        pub fn set_enable(&mut self, value: bool) {
            let raw = value as _;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 0, 1, &mut self.bits)
            };
        }
        ///Write the `mode` field of the register.
        ///
        /// Operating mode (0..=3).
        pub fn set_mode(&mut self, value: u8) {
            let raw = value;
            unsafe {
                ::device_driver::ops::store_lsb0::<
                    u8,
                    ::device_driver::ops::LE,
                >(raw, 1, 3, &mut self.bits)
            };
        }
    }
    impl From<[u8; 1]> for Config {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<Config> for [u8; 1] {
        fn from(val: Config) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for Config {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("Config");
            d.field("enable", &self.enable());
            d.field("mode", &self.mode());
            d.finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Config {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Config {{ ");
            defmt::write!(f, "enable: {=bool}, ", & self.enable());
            defmt::write!(f, "mode: {=u8}, ", & self.mode());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for Config {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for Config {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for Config {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for Config {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for Config {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for Config {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for Config {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        /// Read-only device identification register.
        DeviceId(DeviceId),
        /// Primary configuration register.
        Config(Config),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::DeviceId(val) => core::fmt::Debug::fmt(val, f),
                Self::Config(val) => core::fmt::Debug::fmt(val, f),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::DeviceId(val) => defmt::Format::format(val, f),
                Self::Config(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<DeviceId> for FieldSetValue {
        fn from(val: DeviceId) -> Self {
            Self::DeviceId(val)
        }
    }
    impl From<Config> for FieldSetValue {
        fn from(val: Config) -> Self {
            Self::Config(val)
        }
    }
}
