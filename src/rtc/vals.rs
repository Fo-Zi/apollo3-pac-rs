#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cb {
    #[doc = "Century is 2000s value."]
    _2000 = 0x0,
    #[doc = "Century is 1900s/2100s value."]
    _19002100 = 0x01,
}
impl Cb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cb {
    #[inline(always)]
    fn from(val: u8) -> Cb {
        Cb::from_bits(val)
    }
}
impl From<Cb> for u8 {
    #[inline(always)]
    fn from(val: Cb) -> u8 {
        Cb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cterr {
    #[doc = "No read error occurred value."]
    Noerr = 0x0,
    #[doc = "Read error occurred value."]
    Rderr = 0x01,
}
impl Cterr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cterr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cterr {
    #[inline(always)]
    fn from(val: u8) -> Cterr {
        Cterr::from_bits(val)
    }
}
impl From<Cterr> for u8 {
    #[inline(always)]
    fn from(val: Cterr) -> u8 {
        Cterr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hr1224 {
    #[doc = "Hours in 24 hour mode value."]
    _24hr = 0x0,
    #[doc = "Hours in 12 hour mode value."]
    _12hr = 0x01,
}
impl Hr1224 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hr1224 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hr1224 {
    #[inline(always)]
    fn from(val: u8) -> Hr1224 {
        Hr1224::from_bits(val)
    }
}
impl From<Hr1224> for u8 {
    #[inline(always)]
    fn from(val: Hr1224) -> u8 {
        Hr1224::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpt {
    #[doc = "Alarm interrupt disabled value."]
    Dis = 0x0,
    #[doc = "Interrupt every year value."]
    Year = 0x01,
    #[doc = "Interrupt every month value."]
    Month = 0x02,
    #[doc = "Interrupt every week value."]
    Week = 0x03,
    #[doc = "Interrupt every day value."]
    Day = 0x04,
    #[doc = "Interrupt every hour value."]
    Hr = 0x05,
    #[doc = "Interrupt every minute value."]
    Min = 0x06,
    #[doc = "Interrupt every second/10th/100th value."]
    Sec = 0x07,
}
impl Rpt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpt {
    #[inline(always)]
    fn from(val: u8) -> Rpt {
        Rpt::from_bits(val)
    }
}
impl From<Rpt> for u8 {
    #[inline(always)]
    fn from(val: Rpt) -> u8 {
        Rpt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstop {
    #[doc = "Allow the RTC input clock to run value."]
    Run = 0x0,
    #[doc = "Stop the RTC input clock value."]
    Stop = 0x01,
}
impl Rstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstop {
    #[inline(always)]
    fn from(val: u8) -> Rstop {
        Rstop::from_bits(val)
    }
}
impl From<Rstop> for u8 {
    #[inline(always)]
    fn from(val: Rstop) -> u8 {
        Rstop::to_bits(val)
    }
}
