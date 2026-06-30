#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkdiv {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "24 MHz MSPI clock value."]
    Clk24 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "12 MHz MSPI clock value."]
    Clk12 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "6 MHz MSPI clock value."]
    Clk6 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "3 MHz MSPI clock value."]
    Clk3 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "1.5 MHz MSPI clock value."]
    Clk15 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Clkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkdiv {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkdiv {
    #[inline(always)]
    fn from(val: u8) -> Clkdiv {
        Clkdiv::from_bits(val)
    }
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(val: Clkdiv) -> u8 {
        Clkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpha {
    #[doc = "Clock toggles in middle of data bit. value."]
    Middle = 0x0,
    #[doc = "Clock toggles at start of data bit. value."]
    Start = 0x01,
}
impl Cpha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpha {
    #[inline(always)]
    fn from(val: u8) -> Cpha {
        Cpha::from_bits(val)
    }
}
impl From<Cpha> for u8 {
    #[inline(always)]
    fn from(val: Cpha) -> u8 {
        Cpha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpol {
    #[doc = "Clock inactive state is low. value."]
    Low = 0x0,
    #[doc = "Clock inactive state is high. value."]
    High = 0x01,
}
impl Cpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpol {
    #[inline(always)]
    fn from(val: u8) -> Cpol {
        Cpol::from_bits(val)
    }
}
impl From<Cpol> for u8 {
    #[inline(always)]
    fn from(val: Cpol) -> u8 {
        Cpol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cqflags(u16);
impl Cqflags {
    #[doc = "Software flag 0. Can be used by software to start/pause operatoins value."]
    pub const Swflag0: Self = Self(0x01);
    #[doc = "Software flag 1. Can be used by software to start/pause operations value."]
    pub const Swflag1: Self = Self(0x02);
    #[doc = "Software flag 2. Can be used by software to start/pause operatoins value."]
    pub const Swflag2: Self = Self(0x04);
    #[doc = "Software flag 3. Can be used by software to start/pause operations value."]
    pub const Swflag3: Self = Self(0x08);
    #[doc = "Software flag 4. Can be used by software to start/pause operatoins value."]
    pub const Swflag4: Self = Self(0x10);
    #[doc = "Software flag 5. Can be used by software to start/pause operations value."]
    pub const Swflag5: Self = Self(0x20);
    #[doc = "Software flag 6. Can be used by software to start/pause operatoins value."]
    pub const Swflag6: Self = Self(0x40);
    #[doc = "Software flag 7. Can be used by software to start/pause operations value."]
    pub const Swflag7: Self = Self(0x80);
    #[doc = "IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    pub const Iom0ready: Self = Self(0x0100);
    #[doc = "IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XNOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    pub const Iom1ready: Self = Self(0x0200);
    #[doc = "PIO Operation completed (STATUS bit in CTRL register) value."]
    pub const Cmdcpl: Self = Self(0x0400);
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    pub const Dmacpl: Self = Self(0x0800);
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    pub const Cqidx: Self = Self(0x4000);
    #[doc = "CQ Stop Flag. When set, CQ processing will complete. value."]
    pub const Stop: Self = Self(0x8000);
}
impl Cqflags {
    pub const fn from_bits(val: u16) -> Cqflags {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Cqflags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Swflag0"),
            0x02 => f.write_str("Swflag1"),
            0x04 => f.write_str("Swflag2"),
            0x08 => f.write_str("Swflag3"),
            0x10 => f.write_str("Swflag4"),
            0x20 => f.write_str("Swflag5"),
            0x40 => f.write_str("Swflag6"),
            0x80 => f.write_str("Swflag7"),
            0x0100 => f.write_str("Iom0ready"),
            0x0200 => f.write_str("Iom1ready"),
            0x0400 => f.write_str("Cmdcpl"),
            0x0800 => f.write_str("Dmacpl"),
            0x4000 => f.write_str("Cqidx"),
            0x8000 => f.write_str("Stop"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqflags {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Swflag0"),
            0x02 => defmt::write!(f, "Swflag1"),
            0x04 => defmt::write!(f, "Swflag2"),
            0x08 => defmt::write!(f, "Swflag3"),
            0x10 => defmt::write!(f, "Swflag4"),
            0x20 => defmt::write!(f, "Swflag5"),
            0x40 => defmt::write!(f, "Swflag6"),
            0x80 => defmt::write!(f, "Swflag7"),
            0x0100 => defmt::write!(f, "Iom0ready"),
            0x0200 => defmt::write!(f, "Iom1ready"),
            0x0400 => defmt::write!(f, "Cmdcpl"),
            0x0800 => defmt::write!(f, "Dmacpl"),
            0x4000 => defmt::write!(f, "Cqidx"),
            0x8000 => defmt::write!(f, "Stop"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Cqflags {
    #[inline(always)]
    fn from(val: u16) -> Cqflags {
        Cqflags::from_bits(val)
    }
}
impl From<Cqflags> for u16 {
    #[inline(always)]
    fn from(val: Cqflags) -> u16 {
        Cqflags::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cqmask(u16);
impl Cqmask {
    #[doc = "Software flag 0. Can be used by software to start/pause operatoins value."]
    pub const Swflag0: Self = Self(0x01);
    #[doc = "Software flag 1. Can be used by software to start/pause operations value."]
    pub const Swflag1: Self = Self(0x02);
    #[doc = "Software flag 2. Can be used by software to start/pause operatoins value."]
    pub const Swflag2: Self = Self(0x04);
    #[doc = "Software flag 3. Can be used by software to start/pause operations value."]
    pub const Swflag3: Self = Self(0x08);
    #[doc = "Software flag 4. Can be used by software to start/pause operatoins value."]
    pub const Swflag4: Self = Self(0x10);
    #[doc = "Software flag 5. Can be used by software to start/pause operations value."]
    pub const Swflag5: Self = Self(0x20);
    #[doc = "Software flag 6. Can be used by software to start/pause operatoins value."]
    pub const Swflag6: Self = Self(0x40);
    #[doc = "Software flag 7. Can be used by software to start/pause operations value."]
    pub const Swflag7: Self = Self(0x80);
    #[doc = "IOM Buffer 0 Ready Status (from selected IOM). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    pub const Iom0ready: Self = Self(0x0100);
    #[doc = "IOM Buffer 1 Ready Status (from selected IOM). This status is the result of XOR'ing the IOM0START with the incoming status from the IOM. When high, MSPI can send to the buffer. value."]
    pub const Iom1ready: Self = Self(0x0200);
    #[doc = "PIO Operation completed (STATUS bit in CTRL register) value."]
    pub const Cmdcpl: Self = Self(0x0400);
    #[doc = "DMA Complete Status (hardwired DMACPL bit in DMASTAT) value."]
    pub const Dmacpl: Self = Self(0x0800);
    #[doc = "CQ Index Pointers (CURIDX/ENDIDX) match. value."]
    pub const Cqidx: Self = Self(0x4000);
    #[doc = "CQ Stop Flag. When set, CQ processing will complete. value."]
    pub const Stop: Self = Self(0x8000);
}
impl Cqmask {
    pub const fn from_bits(val: u16) -> Cqmask {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Cqmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Swflag0"),
            0x02 => f.write_str("Swflag1"),
            0x04 => f.write_str("Swflag2"),
            0x08 => f.write_str("Swflag3"),
            0x10 => f.write_str("Swflag4"),
            0x20 => f.write_str("Swflag5"),
            0x40 => f.write_str("Swflag6"),
            0x80 => f.write_str("Swflag7"),
            0x0100 => f.write_str("Iom0ready"),
            0x0200 => f.write_str("Iom1ready"),
            0x0400 => f.write_str("Cmdcpl"),
            0x0800 => f.write_str("Dmacpl"),
            0x4000 => f.write_str("Cqidx"),
            0x8000 => f.write_str("Stop"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqmask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Swflag0"),
            0x02 => defmt::write!(f, "Swflag1"),
            0x04 => defmt::write!(f, "Swflag2"),
            0x08 => defmt::write!(f, "Swflag3"),
            0x10 => defmt::write!(f, "Swflag4"),
            0x20 => defmt::write!(f, "Swflag5"),
            0x40 => defmt::write!(f, "Swflag6"),
            0x80 => defmt::write!(f, "Swflag7"),
            0x0100 => defmt::write!(f, "Iom0ready"),
            0x0200 => defmt::write!(f, "Iom1ready"),
            0x0400 => defmt::write!(f, "Cmdcpl"),
            0x0800 => defmt::write!(f, "Dmacpl"),
            0x4000 => defmt::write!(f, "Cqidx"),
            0x8000 => defmt::write!(f, "Stop"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Cqmask {
    #[inline(always)]
    fn from(val: u16) -> Cqmask {
        Cqmask::from_bits(val)
    }
}
impl From<Cqmask> for u16 {
    #[inline(always)]
    fn from(val: Cqmask) -> u16 {
        Cqmask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cqpri {
    #[doc = "Low Priority (service as best effort) value."]
    Low = 0x0,
    #[doc = "High Priority (service immediately) value."]
    High = 0x01,
}
impl Cqpri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cqpri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cqpri {
    #[inline(always)]
    fn from(val: u8) -> Cqpri {
        Cqpri::from_bits(val)
    }
}
impl From<Cqpri> for u8 {
    #[inline(always)]
    fn from(val: Cqpri) -> u8 {
        Cqpri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Devcfg {
    _RESERVED_0 = 0x0,
    #[doc = "Single bit SPI flash on chip select 0 value."]
    Serial0 = 0x01,
    #[doc = "Single bit SPI flash on chip select 1 value."]
    Serial1 = 0x02,
    #[doc = "Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations value."]
    QuadpairedSerial = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Dual SPI flash on chip select 0 value."]
    Dual0 = 0x05,
    #[doc = "Dual bit SPI flash on chip select 1 value."]
    Dual1 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Quad SPI flash on chip select 0 value."]
    Quad0 = 0x09,
    #[doc = "Quad SPI flash on chip select 1 value."]
    Quad1 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "Octal SPI flash on chip select 0 value."]
    Octal0 = 0x0d,
    #[doc = "Octal SPI flash on chip select 1 value."]
    Octal1 = 0x0e,
    #[doc = "Dual Quad SPI flash on chip selects 0/1. value."]
    Quadpaired = 0x0f,
}
impl Devcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Devcfg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Devcfg {
    #[inline(always)]
    fn from(val: u8) -> Devcfg {
        Devcfg::from_bits(val)
    }
}
impl From<Devcfg> for u8 {
    #[inline(always)]
    fn from(val: Devcfg) -> u8 {
        Devcfg::to_bits(val)
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
pub enum Dmaen {
    #[doc = "Disable DMA Function value."]
    Dis = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Enable HW controlled DMA Function to manage DMA to flash devices. HW will automatically handle issuance of instruction/address bytes based on settings in the FLASH register. value."]
    En = 0x03,
}
impl Dmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaen {
    #[inline(always)]
    fn from(val: u8) -> Dmaen {
        Dmaen::from_bits(val)
    }
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(val: Dmaen) -> u8 {
        Dmaen::to_bits(val)
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
    #[doc = "Auto Priority (priority raised once TX FIFO empties or RX FIFO fills) value."]
    Auto = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dmapri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmapri {
        unsafe { core::mem::transmute(val & 0x03) }
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
pub enum Iomsel {
    #[doc = "ERROR: desc VALUE MISSING value."]
    Iom0 = 0x0,
    #[doc = "ERROR: desc VALUE MISSING value."]
    Iom1 = 0x01,
    #[doc = "ERROR: desc VALUE MISSING value."]
    Iom2 = 0x02,
    #[doc = "ERROR: desc VALUE MISSING value."]
    Iom3 = 0x03,
    #[doc = "ERROR: desc VALUE MISSING value."]
    Iom4 = 0x04,
    #[doc = "ERROR: desc VALUE MISSING value."]
    Iom5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "No IOM selected. Signals always zero. value."]
    Disabled = 0x07,
}
impl Iomsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iomsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iomsel {
    #[inline(always)]
    fn from(val: u8) -> Iomsel {
        Iomsel::from_bits(val)
    }
}
impl From<Iomsel> for u8 {
    #[inline(always)]
    fn from(val: Iomsel) -> u8 {
        Iomsel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Outen(u16);
impl Outen {
    #[doc = "Serial (2 data + 1 clock) value."]
    pub const Serial0: Self = Self(0x0103);
    #[doc = "Quad0 (4 data + 1 clock) value."]
    pub const Quad0: Self = Self(0x010f);
    #[doc = "Quad1 (4 data + 1 clock) value."]
    pub const Quad1: Self = Self(0x01f0);
    #[doc = "Octal (8 data + 1 clock) value."]
    pub const Octal: Self = Self(0x01ff);
}
impl Outen {
    pub const fn from_bits(val: u16) -> Outen {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Outen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0103 => f.write_str("Serial0"),
            0x010f => f.write_str("Quad0"),
            0x01f0 => f.write_str("Quad1"),
            0x01ff => f.write_str("Octal"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outen {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0103 => defmt::write!(f, "Serial0"),
            0x010f => defmt::write!(f, "Quad0"),
            0x01f0 => defmt::write!(f, "Quad1"),
            0x01ff => defmt::write!(f, "Octal"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Outen {
    #[inline(always)]
    fn from(val: u16) -> Outen {
        Outen::from_bits(val)
    }
}
impl From<Outen> for u16 {
    #[inline(always)]
    fn from(val: Outen) -> u16 {
        Outen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxcap {
    #[doc = "RX Capture phase aligns with CPHA setting value."]
    Normal = 0x0,
    #[doc = "RX Capture phase is delayed from CPHA setting by one clock edge value."]
    Delay = 0x01,
}
impl Rxcap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxcap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxcap {
    #[inline(always)]
    fn from(val: u8) -> Rxcap {
        Rxcap::from_bits(val)
    }
}
impl From<Rxcap> for u8 {
    #[inline(always)]
    fn from(val: Rxcap) -> u8 {
        Rxcap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxneg {
    #[doc = "RX data sampled on posedge of internal clock value."]
    Normal = 0x0,
    #[doc = "RX data sampled on negedge of internal clock value."]
    Negedge = 0x01,
}
impl Rxneg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxneg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxneg {
    #[inline(always)]
    fn from(val: u8) -> Rxneg {
        Rxneg::from_bits(val)
    }
}
impl From<Rxneg> for u8 {
    #[inline(always)]
    fn from(val: Rxneg) -> u8 {
        Rxneg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txneg {
    #[doc = "TX launched from posedge internal clock value."]
    Normal = 0x0,
    #[doc = "TX data launched from negedge of internal clock value."]
    Negedge = 0x01,
}
impl Txneg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txneg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txneg {
    #[inline(always)]
    fn from(val: u8) -> Txneg {
        Txneg::from_bits(val)
    }
}
impl From<Txneg> for u8 {
    #[inline(always)]
    fn from(val: Txneg) -> u8 {
        Txneg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xipack {
    #[doc = "No acknowledege sent. Data IOs are tristated the first turnaround cycle value."]
    Noack = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Positive acknowledege sent. Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode value."]
    Ack = 0x02,
    #[doc = "Negative acknowledege sent. Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode. XIPSENDI should be reenabled for the next transfer value."]
    Terminate = 0x03,
}
impl Xipack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xipack {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xipack {
    #[inline(always)]
    fn from(val: u8) -> Xipack {
        Xipack::from_bits(val)
    }
}
impl From<Xipack> for u8 {
    #[inline(always)]
    fn from(val: Xipack) -> u8 {
        Xipack::to_bits(val)
    }
}
