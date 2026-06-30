#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdstat {
    _RESERVED_0 = 0x0,
    #[doc = "Error encountered with command value."]
    Err = 0x01,
    #[doc = "Actively processing command value."]
    Active = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Idle state, no active command, no error value."]
    Idle = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Command in progress, but waiting on data from host value."]
    Wait = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cmdstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdstat {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdstat {
    #[inline(always)]
    fn from(val: u8) -> Cmdstat {
        Cmdstat::from_bits(val)
    }
}
impl From<Cmdstat> for u8 {
    #[inline(always)]
    fn from(val: Cmdstat) -> u8 {
        Cmdstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmadir {
    #[doc = "Peripheral to Memory (SRAM) transaction value."]
    P2m = 0x0,
    #[doc = "Memory to Peripheral transaction value."]
    M2p = 0x01,
}
impl Dmadir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmadir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmadir {
    #[inline(always)]
    fn from(val: u8) -> Dmadir {
        Dmadir::from_bits(val)
    }
}
impl From<Dmadir> for u8 {
    #[inline(always)]
    fn from(val: Dmadir) -> u8 {
        Dmadir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsel {
    #[doc = "Selects the minimum power clock. This setting should be used whenever the IOM is not active. value."]
    MinPwr = 0x0,
    #[doc = "Selects the HFRC as the input clock. value."]
    Hfrc = 0x01,
    #[doc = "Selects the HFRC / 2 as the input clock. value."]
    HfrcDiv2 = 0x02,
    #[doc = "Selects the HFRC / 4 as the input clock. value."]
    HfrcDiv4 = 0x03,
    #[doc = "Selects the HFRC / 8 as the input clock. value."]
    HfrcDiv8 = 0x04,
    #[doc = "Selects the HFRC / 16 as the input clock. value."]
    HfrcDiv16 = 0x05,
    #[doc = "Selects the HFRC / 32 as the input clock. value."]
    HfrcDiv32 = 0x06,
    #[doc = "Selects the HFRC / 64 as the input clock. value."]
    HfrcDiv64 = 0x07,
}
impl Fsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsel {
    #[inline(always)]
    fn from(val: u8) -> Fsel {
        Fsel::from_bits(val)
    }
}
impl From<Fsel> for u8 {
    #[inline(always)]
    fn from(val: Fsel) -> u8 {
        Fsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spha {
    #[doc = "Sample on the leading (first) clock edge, rising or falling dependant on the value of SPOL value."]
    SampleLeadingEdge = 0x0,
    #[doc = "Sample on the trailing (second) clock edge, rising of falling dependant on the value of SPOL value."]
    SampleTrailingEdge = 0x01,
}
impl Spha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spha {
    #[inline(always)]
    fn from(val: u8) -> Spha {
        Spha::from_bits(val)
    }
}
impl From<Spha> for u8 {
    #[inline(always)]
    fn from(val: Spha) -> u8 {
        Spha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spilsb {
    #[doc = "Send and receive MSB bit first value."]
    Msb = 0x0,
    #[doc = "Send and receive LSB bit first value."]
    Lsb = 0x01,
}
impl Spilsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spilsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spilsb {
    #[inline(always)]
    fn from(val: u8) -> Spilsb {
        Spilsb::from_bits(val)
    }
}
impl From<Spilsb> for u8 {
    #[inline(always)]
    fn from(val: Spilsb) -> u8 {
        Spilsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol {
    #[doc = "The initial value of the clock is 0. value."]
    ClkBase0 = 0x0,
    #[doc = "The initial value of the clock is 1. value."]
    ClkBase1 = 0x01,
}
impl Spol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol {
    #[inline(always)]
    fn from(val: u8) -> Spol {
        Spol::from_bits(val)
    }
}
impl From<Spol> for u8 {
    #[inline(always)]
    fn from(val: Spol) -> u8 {
        Spol::to_bits(val)
    }
}
