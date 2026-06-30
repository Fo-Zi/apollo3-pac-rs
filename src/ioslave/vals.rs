#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ifcsel {
    #[doc = "Selects I2C interface for the IO Slave. value."]
    I2c = 0x0,
    #[doc = "Selects SPI interface for the IO Slave. value."]
    Spi = 0x01,
}
impl Ifcsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ifcsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ifcsel {
    #[inline(always)]
    fn from(val: u8) -> Ifcsel {
        Ifcsel::from_bits(val)
    }
}
impl From<Ifcsel> for u8 {
    #[inline(always)]
    fn from(val: Ifcsel) -> u8 {
        Ifcsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsb {
    #[doc = "Data is assumed to be sent and received with MSB first. value."]
    MsbFirst = 0x0,
    #[doc = "Data is assumed to be sent and received with LSB first. value."]
    LsbFirst = 0x01,
}
impl Lsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsb {
    #[inline(always)]
    fn from(val: u8) -> Lsb {
        Lsb::from_bits(val)
    }
}
impl From<Lsb> for u8 {
    #[inline(always)]
    fn from(val: Lsb) -> u8 {
        Lsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol {
    #[doc = "Polarity 0, handles SPI modes 0 and 3. value."]
    SpiModes03 = 0x0,
    #[doc = "Polarity 1, handles SPI modes 1 and 2. value."]
    SpiModes12 = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Startrd {
    #[doc = "Initiate I/O RAM read late in each transferred byte. value."]
    Late = 0x0,
    #[doc = "Initiate I/O RAM read early in each transferred byte. value."]
    Early = 0x01,
}
impl Startrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Startrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Startrd {
    #[inline(always)]
    fn from(val: u8) -> Startrd {
        Startrd::from_bits(val)
    }
}
impl From<Startrd> for u8 {
    #[inline(always)]
    fn from(val: Startrd) -> u8 {
        Startrd::to_bits(val)
    }
}
