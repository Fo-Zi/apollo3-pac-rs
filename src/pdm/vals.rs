#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bclkinv {
    #[doc = "BCLK inverted. value."]
    Inv = 0x0,
    #[doc = "BCLK not inverted. value."]
    Norm = 0x01,
}
impl Bclkinv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bclkinv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bclkinv {
    #[inline(always)]
    fn from(val: u8) -> Bclkinv {
        Bclkinv::from_bits(val)
    }
}
impl From<Bclkinv> for u8 {
    #[inline(always)]
    fn from(val: Bclkinv) -> u8 {
        Bclkinv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chset {
    #[doc = "Channel disabled. value."]
    Dis = 0x0,
    #[doc = "Mono left channel. value."]
    Left = 0x01,
    #[doc = "Mono right channel. value."]
    Right = 0x02,
    #[doc = "Stereo channels. value."]
    Stereo = 0x03,
}
impl Chset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chset {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chset {
    #[inline(always)]
    fn from(val: u8) -> Chset {
        Chset::from_bits(val)
    }
}
impl From<Chset> for u8 {
    #[inline(always)]
    fn from(val: Chset) -> u8 {
        Chset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmadir {
    #[doc = "Peripheral to Memory (SRAM) transaction. THe PDM module will only DMA to memory. value."]
    P2m = 0x0,
    #[doc = "Memory to Peripheral transaction. Not available for PDM module value."]
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
pub enum Dmapri {
    #[doc = "Low Priority (service as best effort) value."]
    Low = 0x0,
    #[doc = "High Priority (service immediately) value."]
    High = 0x01,
}
impl Dmapri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmapri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmapri {
    #[inline(always)]
    fn from(val: u8) -> Dmapri {
        Dmapri::from_bits(val)
    }
}
impl From<Dmapri> for u8 {
    #[inline(always)]
    fn from(val: Dmapri) -> u8 {
        Dmapri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmickdel {
    #[doc = "No delay. value."]
    _0cyc = 0x0,
    #[doc = "1 cycle delay. value."]
    _1cyc = 0x01,
}
impl Dmickdel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmickdel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmickdel {
    #[inline(always)]
    fn from(val: u8) -> Dmickdel {
        Dmickdel::from_bits(val)
    }
}
impl From<Dmickdel> for u8 {
    #[inline(always)]
    fn from(val: Dmickdel) -> u8 {
        Dmickdel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrswap {
    #[doc = "No channel swapping (IFO Read LEFT_RIGHT). value."]
    Noswap = 0x0,
    #[doc = "Swap left and right channels (FIFO Read RIGHT_LEFT). value."]
    En = 0x01,
}
impl Lrswap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrswap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrswap {
    #[inline(always)]
    fn from(val: u8) -> Lrswap {
        Lrswap::from_bits(val)
    }
}
impl From<Lrswap> for u8 {
    #[inline(always)]
    fn from(val: Lrswap) -> u8 {
        Lrswap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkdiv {
    #[doc = "Divide input clock by 1 value."]
    Mckdiv1 = 0x0,
    #[doc = "Divide input clock by 2 value."]
    Mckdiv2 = 0x01,
    #[doc = "Divide input clock by 3 value."]
    Mckdiv3 = 0x02,
    #[doc = "Divide input clock by 4 value."]
    Mckdiv4 = 0x03,
}
impl Mclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkdiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Mclkdiv {
        Mclkdiv::from_bits(val)
    }
}
impl From<Mclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Mclkdiv) -> u8 {
        Mclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdmclksel {
    #[doc = "Static value. value."]
    Disable = 0x0,
    #[doc = "PDM clock is 12 MHz. value."]
    _12mHz = 0x01,
    #[doc = "PDM clock is 6 MHz. value."]
    _6mHz = 0x02,
    #[doc = "PDM clock is 3 MHz. value."]
    _3mHz = 0x03,
    #[doc = "PDM clock is 1.5 MHz. value."]
    _15mHz = 0x04,
    #[doc = "PDM clock is 750 KHz. value."]
    _750kHz = 0x05,
    #[doc = "PDM clock is 375 KHz. value."]
    _375kHz = 0x06,
    #[doc = "PDM clock is 187.5 KHz. value."]
    _187kHz = 0x07,
}
impl Pdmclksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdmclksel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdmclksel {
    #[inline(always)]
    fn from(val: u8) -> Pdmclksel {
        Pdmclksel::from_bits(val)
    }
}
impl From<Pdmclksel> for u8 {
    #[inline(always)]
    fn from(val: Pdmclksel) -> u8 {
        Pdmclksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pgaleft {
    #[doc = "-6.0 db gain. value."]
    M60db = 0x0,
    #[doc = "-4.5 db gain. value."]
    M45db = 0x01,
    #[doc = "-3.0 db gain. value."]
    M300db = 0x02,
    #[doc = "-1.5 db gain. value."]
    M15db = 0x03,
    #[doc = "0.0 db gain. value."]
    _0db = 0x04,
    #[doc = "1.5 db gain. value."]
    P15db = 0x05,
    #[doc = "3.0 db gain. value."]
    P30db = 0x06,
    #[doc = "4.5 db gain. value."]
    P45db = 0x07,
    #[doc = "6.0 db gain. value."]
    P60db = 0x08,
    #[doc = "7.5 db gain. value."]
    P75db = 0x09,
    #[doc = "9.0 db gain. value."]
    P90db = 0x0a,
    #[doc = "10.5 db gain. value."]
    P105db = 0x0b,
    #[doc = "12.0 db gain. value."]
    P120db = 0x0c,
    #[doc = "13.5 db gain. value."]
    P135db = 0x0d,
    #[doc = "15.0 db gain. value."]
    P150db = 0x0e,
    #[doc = "16.5 db gain. value."]
    P165db = 0x0f,
    #[doc = "18.0 db gain. value."]
    P180db = 0x10,
    #[doc = "19.5 db gain. value."]
    P195db = 0x11,
    #[doc = "21.0 db gain. value."]
    P210db = 0x12,
    #[doc = "22.5 db gain. value."]
    P225db = 0x13,
    #[doc = "24.0 db gain. value."]
    P240db = 0x14,
    #[doc = "25.5 db gain. value."]
    P255db = 0x15,
    #[doc = "27.0 db gain. value."]
    P270db = 0x16,
    #[doc = "28.5 db gain. value."]
    P285db = 0x17,
    #[doc = "30.0 db gain. value."]
    P300db = 0x18,
    #[doc = "31.5 db gain. value."]
    P315db = 0x19,
    #[doc = "33.0 db gain. value."]
    P330db = 0x1a,
    #[doc = "34.5 db gain. value."]
    P345db = 0x1b,
    #[doc = "36.0 db gain. value."]
    P360db = 0x1c,
    #[doc = "37.5 db gain. value."]
    P375db = 0x1d,
    #[doc = "39.0 db gain. value."]
    P390db = 0x1e,
    #[doc = "40.5 db gain. value."]
    P405db = 0x1f,
}
impl Pgaleft {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pgaleft {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pgaleft {
    #[inline(always)]
    fn from(val: u8) -> Pgaleft {
        Pgaleft::from_bits(val)
    }
}
impl From<Pgaleft> for u8 {
    #[inline(always)]
    fn from(val: Pgaleft) -> u8 {
        Pgaleft::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pgaright {
    #[doc = "-6.0 db gain. value."]
    M60db = 0x0,
    #[doc = "-4.5 db gain. value."]
    M45db = 0x01,
    #[doc = "-3.0 db gain. value."]
    M300db = 0x02,
    #[doc = "-1.5 db gain. value."]
    M15db = 0x03,
    #[doc = "0.0 db gain. value."]
    _0db = 0x04,
    #[doc = "1.5 db gain. value."]
    P15db = 0x05,
    #[doc = "3.0 db gain. value."]
    P30db = 0x06,
    #[doc = "4.5 db gain. value."]
    P45db = 0x07,
    #[doc = "6.0 db gain. value."]
    P60db = 0x08,
    #[doc = "7.5 db gain. value."]
    P75db = 0x09,
    #[doc = "9.0 db gain. value."]
    P90db = 0x0a,
    #[doc = "10.5 db gain. value."]
    P105db = 0x0b,
    #[doc = "12.0 db gain. value."]
    P120db = 0x0c,
    #[doc = "13.5 db gain. value."]
    P135db = 0x0d,
    #[doc = "15.0 db gain. value."]
    P150db = 0x0e,
    #[doc = "16.5 db gain. value."]
    P165db = 0x0f,
    #[doc = "18.0 db gain. value."]
    P180db = 0x10,
    #[doc = "19.5 db gain. value."]
    P195db = 0x11,
    #[doc = "21.0 db gain. value."]
    P210db = 0x12,
    #[doc = "22.5 db gain. value."]
    P225db = 0x13,
    #[doc = "24.0 db gain. value."]
    P240db = 0x14,
    #[doc = "25.5 db gain. value."]
    P255db = 0x15,
    #[doc = "27.0 db gain. value."]
    P270db = 0x16,
    #[doc = "28.5 db gain. value."]
    P285db = 0x17,
    #[doc = "30.0 db gain. value."]
    P300db = 0x18,
    #[doc = "31.5 db gain. value."]
    P315db = 0x19,
    #[doc = "33.0 db gain. value."]
    P330db = 0x1a,
    #[doc = "34.5 db gain. value."]
    P345db = 0x1b,
    #[doc = "36.0 db gain. value."]
    P360db = 0x1c,
    #[doc = "37.5 db gain. value."]
    P375db = 0x1d,
    #[doc = "39.0 db gain. value."]
    P390db = 0x1e,
    #[doc = "40.5 db gain. value."]
    P405db = 0x1f,
}
impl Pgaright {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pgaright {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pgaright {
    #[inline(always)]
    fn from(val: u8) -> Pgaright {
        Pgaright::from_bits(val)
    }
}
impl From<Pgaright> for u8 {
    #[inline(always)]
    fn from(val: Pgaright) -> u8 {
        Pgaright::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstb {
    #[doc = "Reset the core. value."]
    Reset = 0x0,
    #[doc = "Enable the core. value."]
    Norm = 0x01,
}
impl Rstb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstb {
    #[inline(always)]
    fn from(val: u8) -> Rstb {
        Rstb::from_bits(val)
    }
}
impl From<Rstb> for u8 {
    #[inline(always)]
    fn from(val: Rstb) -> u8 {
        Rstb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Selap {
    #[doc = "Clock source from internal clock generator. value."]
    Internal = 0x0,
    #[doc = "Clock source from I2S BCLK. value."]
    I2s = 0x01,
}
impl Selap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Selap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Selap {
    #[inline(always)]
    fn from(val: u8) -> Selap {
        Selap::from_bits(val)
    }
}
impl From<Selap> for u8 {
    #[inline(always)]
    fn from(val: Selap) -> u8 {
        Selap::to_bits(val)
    }
}
