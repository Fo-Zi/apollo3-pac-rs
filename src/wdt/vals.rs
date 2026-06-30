#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksel {
    #[doc = "Low Power Mode. This setting disables the watch dog timer. value."]
    Off = 0x0,
    #[doc = "128 Hz LFRC clock. value."]
    _128hz = 0x01,
    #[doc = "16 Hz LFRC clock. value."]
    _16hz = 0x02,
    #[doc = "1 Hz LFRC clock. value."]
    _1hz = 0x03,
    #[doc = "1/16th Hz LFRC clock. value."]
    _116hz = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksel {
    #[inline(always)]
    fn from(val: u8) -> Clksel {
        Clksel::from_bits(val)
    }
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(val: Clksel) -> u8 {
        Clksel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lock(u8);
impl Lock {
    #[doc = "This is the key value to write to WDTLOCK to lock the WDT. value."]
    pub const Keyvalue: Self = Self(0x3a);
}
impl Lock {
    pub const fn from_bits(val: u8) -> Lock {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x3a => f.write_str("Keyvalue"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x3a => defmt::write!(f, "Keyvalue"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Lock {
    #[inline(always)]
    fn from(val: u8) -> Lock {
        Lock::from_bits(val)
    }
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(val: Lock) -> u8 {
        Lock::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rstrt(u8);
impl Rstrt {
    #[doc = "This is the key value to write to WDTRSTRT to restart the WDT. This is a write only register. value."]
    pub const Keyvalue: Self = Self(0xb2);
}
impl Rstrt {
    pub const fn from_bits(val: u8) -> Rstrt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Rstrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xb2 => f.write_str("Keyvalue"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstrt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xb2 => defmt::write!(f, "Keyvalue"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Rstrt {
    #[inline(always)]
    fn from(val: u8) -> Rstrt {
        Rstrt::from_bits(val)
    }
}
impl From<Rstrt> for u8 {
    #[inline(always)]
    fn from(val: Rstrt) -> u8 {
        Rstrt::to_bits(val)
    }
}
