#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Swpoikey(u8);
impl Swpoikey {
    #[doc = "Writing 0x1B key value generates a software POI reset. value."]
    pub const Keyvalue: Self = Self(0x1b);
}
impl Swpoikey {
    pub const fn from_bits(val: u8) -> Swpoikey {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Swpoikey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x1b => f.write_str("Keyvalue"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swpoikey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x1b => defmt::write!(f, "Keyvalue"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Swpoikey {
    #[inline(always)]
    fn from(val: u8) -> Swpoikey {
        Swpoikey::from_bits(val)
    }
}
impl From<Swpoikey> for u8 {
    #[inline(always)]
    fn from(val: Swpoikey) -> u8 {
        Swpoikey::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Swporkey(u8);
impl Swporkey {
    #[doc = "Writing 0xD4 key value generates a software POR reset. value."]
    pub const Keyvalue: Self = Self(0xd4);
}
impl Swporkey {
    pub const fn from_bits(val: u8) -> Swporkey {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Swporkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xd4 => f.write_str("Keyvalue"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swporkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xd4 => defmt::write!(f, "Keyvalue"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Swporkey {
    #[inline(always)]
    fn from(val: u8) -> Swporkey {
        Swporkey::from_bits(val)
    }
}
impl From<Swporkey> for u8 {
    #[inline(always)]
    fn from(val: Swporkey) -> u8 {
        Swporkey::to_bits(val)
    }
}
