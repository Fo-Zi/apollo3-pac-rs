#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Config {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Direct mapped, 128-bit linesize, 512 entries (4 SRAMs active) value."]
    W1128b512e = 0x04,
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active) value."]
    W2128b512e = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Direct mapped, 128-bit linesize, 1024 entries (8 SRAMs active) value."]
    W1128b1024e = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Config {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Config {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Config {
    #[inline(always)]
    fn from(val: u8) -> Config {
        Config::from_bits(val)
    }
}
impl From<Config> for u8 {
    #[inline(always)]
    fn from(val: Config) -> u8 {
        Config::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpmmode {
    #[doc = "High power mode (LPM not used). value."]
    Never = 0x0,
    #[doc = "Fast Standby mode. LPM deasserted for read operations, but asserted while flash IDLE. value."]
    Standby = 0x01,
    #[doc = "Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accomodate longer read access times. value."]
    Always = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpmmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpmmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpmmode {
    #[inline(always)]
    fn from(val: u8) -> Lpmmode {
        Lpmmode::from_bits(val)
    }
}
impl From<Lpmmode> for u8 {
    #[inline(always)]
    fn from(val: Lpmmode) -> u8 {
        Lpmmode::to_bits(val)
    }
}
