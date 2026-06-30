#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addrsz {
    #[doc = "Use 7b addressing for I2C master transactions value."]
    Addrsz7 = 0x0,
    #[doc = "Use 10b addressing for I2C master transactions value."]
    Addrsz10 = 0x01,
}
impl Addrsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addrsz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addrsz {
    #[inline(always)]
    fn from(val: u8) -> Addrsz {
        Addrsz::from_bits(val)
    }
}
impl From<Addrsz> for u8 {
    #[inline(always)]
    fn from(val: Addrsz) -> u8 {
        Addrsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arben {
    #[doc = "Disable multi-master bus arbitration support for this i2c master value."]
    Arbdis = 0x0,
    #[doc = "Enable multi-master bus arbitration support for this i2c master value."]
    Arben = 0x01,
}
impl Arben {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arben {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arben {
    #[inline(always)]
    fn from(val: u8) -> Arben {
        Arben::from_bits(val)
    }
}
impl From<Arben> for u8 {
    #[inline(always)]
    fn from(val: Arben) -> u8 {
        Arben::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd {
    _RESERVED_0 = 0x0,
    #[doc = "Write command using count of offset bytes specified in the OFFSETCNT field value."]
    Write = 0x01,
    #[doc = "Read command using count of offset bytes specified in the OFFSETCNT field value."]
    Read = 0x02,
    #[doc = "SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field value."]
    Tmw = 0x03,
    #[doc = "SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input value."]
    Tmr = 0x04,
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
    _RESERVED_10 = 0x10,
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
}
impl Cmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmd {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cqpen(u16);
impl Cqpen {
    #[doc = "Pause the command queue when software flag bit 0 is '1' value."]
    pub const Swflagen0: Self = Self(0x01);
    #[doc = "Pause the command queue when software flag bit 1 is '1' value."]
    pub const Swflagen1: Self = Self(0x02);
    #[doc = "Pause the command queue when software flag bit 2 is '1' value."]
    pub const Swflagen2: Self = Self(0x04);
    #[doc = "Pause the command queue when software flag bit 3 is '1' value."]
    pub const Swflagen3: Self = Self(0x08);
    #[doc = "Pause the command queue when software flag bit 4 is '1' value."]
    pub const Swflagen4: Self = Self(0x10);
    #[doc = "Pause the command queue when software flag bit 5 is '1' value."]
    pub const Swflagen5: Self = Self(0x20);
    #[doc = "Pause the command queue when software flag bit 6 is '1' value."]
    pub const Swflagen6: Self = Self(0x40);
    #[doc = "Pause the command queue when software flag bit 7 is '1'. value."]
    pub const Swflagen7: Self = Self(0x80);
    #[doc = "Pause command queue when input MSPI0 bit XORed with SWFLAG0 is '1' value."]
    pub const Mspi0xoren: Self = Self(0x0100);
    #[doc = "Pause command queue when input MSPI1 bit XORed with SWFLAG1 is '1' value."]
    pub const Mspi1xoren: Self = Self(0x0200);
    #[doc = "Pause command queue when input MSPI0 bit XNORed with SWFLAG0 is '1' value."]
    pub const Mspi0xnoren: Self = Self(0x0400);
    #[doc = "Pause command queue when input MSPI1 bit XNORed with SWFLAG1 is '1' value."]
    pub const Mspi1xnoren: Self = Self(0x0800);
    #[doc = "Pause command queue when input GPIO irq_bit XORed with SWFLAG2 is '1' value."]
    pub const Gpioxoren: Self = Self(0x1000);
    #[doc = "Pause command queue when input IOM bit XORed with SWFLAG3 is '1' value."]
    pub const Iomxoren: Self = Self(0x2000);
    #[doc = "Pause command queue when input BLE bit XORed with SWFLAG4 is '1' value."]
    pub const Blexoren: Self = Self(0x4000);
    #[doc = "Pauses the command queue when the current index matches the last index value."]
    pub const Idxeq: Self = Self(0x8000);
}
impl Cqpen {
    pub const fn from_bits(val: u16) -> Cqpen {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Cqpen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Swflagen0"),
            0x02 => f.write_str("Swflagen1"),
            0x04 => f.write_str("Swflagen2"),
            0x08 => f.write_str("Swflagen3"),
            0x10 => f.write_str("Swflagen4"),
            0x20 => f.write_str("Swflagen5"),
            0x40 => f.write_str("Swflagen6"),
            0x80 => f.write_str("Swflagen7"),
            0x0100 => f.write_str("Mspi0xoren"),
            0x0200 => f.write_str("Mspi1xoren"),
            0x0400 => f.write_str("Mspi0xnoren"),
            0x0800 => f.write_str("Mspi1xnoren"),
            0x1000 => f.write_str("Gpioxoren"),
            0x2000 => f.write_str("Iomxoren"),
            0x4000 => f.write_str("Blexoren"),
            0x8000 => f.write_str("Idxeq"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqpen {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Swflagen0"),
            0x02 => defmt::write!(f, "Swflagen1"),
            0x04 => defmt::write!(f, "Swflagen2"),
            0x08 => defmt::write!(f, "Swflagen3"),
            0x10 => defmt::write!(f, "Swflagen4"),
            0x20 => defmt::write!(f, "Swflagen5"),
            0x40 => defmt::write!(f, "Swflagen6"),
            0x80 => defmt::write!(f, "Swflagen7"),
            0x0100 => defmt::write!(f, "Mspi0xoren"),
            0x0200 => defmt::write!(f, "Mspi1xoren"),
            0x0400 => defmt::write!(f, "Mspi0xnoren"),
            0x0800 => defmt::write!(f, "Mspi1xnoren"),
            0x1000 => defmt::write!(f, "Gpioxoren"),
            0x2000 => defmt::write!(f, "Iomxoren"),
            0x4000 => defmt::write!(f, "Blexoren"),
            0x8000 => defmt::write!(f, "Idxeq"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Cqpen {
    #[inline(always)]
    fn from(val: u16) -> Cqpen {
        Cqpen::from_bits(val)
    }
}
impl From<Cqpen> for u16 {
    #[inline(always)]
    fn from(val: Cqpen) -> u16 {
        Cqpen::to_bits(val)
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
pub enum Dmadir {
    #[doc = "Peripheral to Memory (SRAM) transaction. To be set when doing IOM read operations, ie reading data from external devices. value."]
    P2m = 0x0,
    #[doc = "Memory to Peripheral transaction. To be set when doing IOM write operations, ie writing data to external devices. value."]
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
pub enum I2clsb {
    #[doc = "Byte data is transmitted MSB first onto the bus/read from the bus value."]
    Msbfirst = 0x0,
    #[doc = "Byte data is transmitted LSB first onto the bus/read from the bus value."]
    Lsbfirst = 0x01,
}
impl I2clsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2clsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2clsb {
    #[inline(always)]
    fn from(val: u8) -> I2clsb {
        I2clsb::from_bits(val)
    }
}
impl From<I2clsb> for u8 {
    #[inline(always)]
    fn from(val: I2clsb) -> u8 {
        I2clsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mosiinv {
    #[doc = "MOSI is set to 0 in read mode and 1 in write mode. value."]
    Normal = 0x0,
    #[doc = "MOSI is set to 1 in read mode and 0 in write mode. value."]
    Invert = 0x01,
}
impl Mosiinv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mosiinv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mosiinv {
    #[inline(always)]
    fn from(val: u8) -> Mosiinv {
        Mosiinv::from_bits(val)
    }
}
impl From<Mosiinv> for u8 {
    #[inline(always)]
    fn from(val: Mosiinv) -> u8 {
        Mosiinv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdfcpol {
    #[doc = "Flow control signal high creates flow control. value."]
    High = 0x0,
    #[doc = "Flow control signal low creates flow control. value."]
    Low = 0x01,
}
impl Rdfcpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdfcpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdfcpol {
    #[inline(always)]
    fn from(val: u8) -> Rdfcpol {
        Rdfcpol::from_bits(val)
    }
}
impl From<Rdfcpol> for u8 {
    #[inline(always)]
    fn from(val: Rdfcpol) -> u8 {
        Rdfcpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smod0type {
    #[doc = "MSPI submodule value."]
    SpiMaster = 0x0,
    #[doc = "I2C Master submodule value."]
    I2cMaster = 0x01,
    #[doc = "SPI Slave submodule value."]
    Sspi = 0x02,
    #[doc = "I2C Slave submodule value."]
    Si2c = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "NOT INSTALLED value."]
    Na = 0x07,
}
impl Smod0type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smod0type {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smod0type {
    #[inline(always)]
    fn from(val: u8) -> Smod0type {
        Smod0type::from_bits(val)
    }
}
impl From<Smod0type> for u8 {
    #[inline(always)]
    fn from(val: Smod0type) -> u8 {
        Smod0type::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smod1type {
    #[doc = "SPI Master submodule value."]
    Mspi = 0x0,
    #[doc = "MI2C submodule value."]
    I2cMaster = 0x01,
    #[doc = "SPI Slave submodule value."]
    Sspi = 0x02,
    #[doc = "I2C Slave submodule value."]
    Si2c = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "NOT INSTALLED value."]
    Na = 0x07,
}
impl Smod1type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smod1type {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smod1type {
    #[inline(always)]
    fn from(val: u8) -> Smod1type {
        Smod1type::from_bits(val)
    }
}
impl From<Smod1type> for u8 {
    #[inline(always)]
    fn from(val: Smod1type) -> u8 {
        Smod1type::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spha {
    #[doc = "Sample on the leading (first) clock edge. value."]
    SampleLeadingEdge = 0x0,
    #[doc = "Sample on the trailing (second) clock edge. value."]
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
    #[doc = "The base value of the clock is 0. value."]
    ClkBase0 = 0x0,
    #[doc = "The base value of the clock is 1. value."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wtfcirq {
    #[doc = "MISO is used as the write mode flow control signal. value."]
    Miso = 0x0,
    #[doc = "IRQ is used as the write mode flow control signal. value."]
    Irq = 0x01,
}
impl Wtfcirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wtfcirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wtfcirq {
    #[inline(always)]
    fn from(val: u8) -> Wtfcirq {
        Wtfcirq::from_bits(val)
    }
}
impl From<Wtfcirq> for u8 {
    #[inline(always)]
    fn from(val: Wtfcirq) -> u8 {
        Wtfcirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wtfcpol {
    #[doc = "Flow control signal high(1) creates flow control and byte transfers will stop until the flow control signal goes low. value."]
    High = 0x0,
    #[doc = "Flow control signal low(0) creates flow control and byte transfers will stop until the flow control signal goes high(1). value."]
    Low = 0x01,
}
impl Wtfcpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wtfcpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wtfcpol {
    #[inline(always)]
    fn from(val: u8) -> Wtfcpol {
        Wtfcpol::from_bits(val)
    }
}
impl From<Wtfcpol> for u8 {
    #[inline(always)]
    fn from(val: Wtfcpol) -> u8 {
        Wtfcpol::to_bits(val)
    }
}
