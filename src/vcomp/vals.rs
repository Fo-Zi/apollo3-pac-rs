#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpout {
    #[doc = "The negative input of the comparator is greater than the positive input. value."]
    VoutLow = 0x0,
    #[doc = "The positive input of the comparator is greater than the negative input. value."]
    VoutHigh = 0x01,
}
impl Cmpout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpout {
    #[inline(always)]
    fn from(val: u8) -> Cmpout {
        Cmpout::from_bits(val)
    }
}
impl From<Cmpout> for u8 {
    #[inline(always)]
    fn from(val: Cmpout) -> u8 {
        Cmpout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvlsel {
    #[doc = "Set Reference input to 0.58 Volts. value."]
    _0p58v = 0x0,
    #[doc = "Set Reference input to 0.77 Volts. value."]
    _0p77v = 0x01,
    #[doc = "Set Reference input to 0.97 Volts. value."]
    _0p97v = 0x02,
    #[doc = "Set Reference input to 1.16 Volts. value."]
    _1p16v = 0x03,
    #[doc = "Set Reference input to 1.35 Volts. value."]
    _1p35v = 0x04,
    #[doc = "Set Reference input to 1.55 Volts. value."]
    _1p55v = 0x05,
    #[doc = "Set Reference input to 1.74 Volts. value."]
    _1p74v = 0x06,
    #[doc = "Set Reference input to 1.93 Volts. value."]
    _1p93v = 0x07,
    #[doc = "Set Reference input to 2.13 Volts. value."]
    _2p13v = 0x08,
    #[doc = "Set Reference input to 2.32 Volts. value."]
    _2p32v = 0x09,
    #[doc = "Set Reference input to 2.51 Volts. value."]
    _2p51v = 0x0a,
    #[doc = "Set Reference input to 2.71 Volts. value."]
    _2p71v = 0x0b,
    #[doc = "Set Reference input to 2.90 Volts. value."]
    _2p90v = 0x0c,
    #[doc = "Set Reference input to 3.09 Volts. value."]
    _3p09v = 0x0d,
    #[doc = "Set Reference input to 3.29 Volts. value."]
    _3p29v = 0x0e,
    #[doc = "Set Reference input to 3.48 Volts. value."]
    _3p48v = 0x0f,
}
impl Lvlsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvlsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvlsel {
    #[inline(always)]
    fn from(val: u8) -> Lvlsel {
        Lvlsel::from_bits(val)
    }
}
impl From<Lvlsel> for u8 {
    #[inline(always)]
    fn from(val: Lvlsel) -> u8 {
        Lvlsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nsel {
    #[doc = "Use external reference 1 for reference input. value."]
    Vrefext1 = 0x0,
    #[doc = "Use external reference 2 for reference input. value."]
    Vrefext2 = 0x01,
    #[doc = "Use external reference 3 for reference input. value."]
    Vrefext3 = 0x02,
    #[doc = "Use DAC output selected by LVLSEL for reference input. value."]
    Dac = 0x03,
}
impl Nsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nsel {
    #[inline(always)]
    fn from(val: u8) -> Nsel {
        Nsel::from_bits(val)
    }
}
impl From<Nsel> for u8 {
    #[inline(always)]
    fn from(val: Nsel) -> u8 {
        Nsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psel {
    #[doc = "Use VDDADJ for the positive input. value."]
    Vddadj = 0x0,
    #[doc = "Use the temperature sensor output for the positive input. Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on. The bandgap circuit requires 11us to stabalize. value."]
    Vtemp = 0x01,
    #[doc = "Use external voltage 0 for positive input. value."]
    Vext1 = 0x02,
    #[doc = "Use external voltage 1 for positive input. value."]
    Vext2 = 0x03,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pwdkey(u32);
impl Pwdkey {
    #[doc = "Key value."]
    pub const Key: Self = Self(0x37);
}
impl Pwdkey {
    pub const fn from_bits(val: u32) -> Pwdkey {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Pwdkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x37 => f.write_str("Key"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwdkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x37 => defmt::write!(f, "Key"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Pwdkey {
    #[inline(always)]
    fn from(val: u32) -> Pwdkey {
        Pwdkey::from_bits(val)
    }
}
impl From<Pwdkey> for u32 {
    #[inline(always)]
    fn from(val: Pwdkey) -> u32 {
        Pwdkey::to_bits(val)
    }
}
