#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debugen {
    #[doc = "Debug Disabled value."]
    Off = 0x0,
    #[doc = "Debug Arb values value."]
    Arb = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Debugen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debugen {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debugen {
    #[inline(always)]
    fn from(val: u8) -> Debugen {
        Debugen::from_bits(val)
    }
}
impl From<Debugen> for u8 {
    #[inline(always)]
    fn from(val: Debugen) -> u8 {
        Debugen::to_bits(val)
    }
}
