#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum B2mstate {
    #[doc = "Reset State value."]
    Reset = 0x0,
    #[doc = "Sleep state. value."]
    Sleep = 0x01,
    #[doc = "Standby State value."]
    Standby = 0x02,
    #[doc = "Idle state value."]
    Idle = 0x03,
    #[doc = "Active state. value."]
    Active = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl B2mstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> B2mstate {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for B2mstate {
    #[inline(always)]
    fn from(val: u8) -> B2mstate {
        B2mstate::from_bits(val)
    }
}
impl From<B2mstate> for u8 {
    #[inline(always)]
    fn from(val: B2mstate) -> u8 {
        B2mstate::to_bits(val)
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cqpen(u16);
impl Cqpen {
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    pub const Swflgen0: Self = Self(0x01);
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    pub const Swflagen1: Self = Self(0x02);
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    pub const Swflagen2: Self = Self(0x04);
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    pub const Swflagen3: Self = Self(0x08);
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    pub const Swflagen4: Self = Self(0x10);
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
    pub const Swflagen5: Self = Self(0x20);
    #[doc = "Pause the command queue when software flag bit 7 is '1' value."]
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
    #[doc = "Pauses command queue processing when HWCNT matches SWCNT value."]
    pub const Cnteq: Self = Self(0x8000);
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
            0x01 => f.write_str("Swflgen0"),
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
            0x8000 => f.write_str("Cnteq"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqpen {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Swflgen0"),
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
            0x8000 => defmt::write!(f, "Cnteq"),
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
pub enum Fcpol {
    #[doc = "SPI_STATUS signal from BLE Core high(1) creates flow control and new read spi transactions will not be started until the signal goes low.(default) value."]
    Normal = 0x0,
    #[doc = "SPI_STATUS signal from BLE Core low(0) creates flow control and new read spi transactions will not be started until the signal goes high. value."]
    Inverted = 0x01,
}
impl Fcpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fcpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fcpol {
    #[inline(always)]
    fn from(val: u8) -> Fcpol {
        Fcpol::from_bits(val)
    }
}
impl From<Fcpol> for u8 {
    #[inline(always)]
    fn from(val: Fcpol) -> u8 {
        Fcpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Powerctl {
    #[doc = "BLEH Power-on signal is controlled by the PWRSM logic and automatically controlled value."]
    Auto = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "BLEH Power-on signal is set to off (0). value."]
    Off = 0x02,
    #[doc = "BLEH Power-on reg signal is set to on (1). value."]
    On = 0x03,
}
impl Powerctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Powerctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Powerctl {
    #[inline(always)]
    fn from(val: u8) -> Powerctl {
        Powerctl::from_bits(val)
    }
}
impl From<Powerctl> for u8 {
    #[inline(always)]
    fn from(val: Powerctl) -> u8 {
        Powerctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrst {
    #[doc = "Internal power state machine is disabled and will not sequence the BLEH power domain. The values of the overrides will be used to drive the output sequencing signals value."]
    Off = 0x0,
    #[doc = "Initialization state. BLEH not powered value."]
    Init = 0x01,
    #[doc = "Waiting for the powerup of the BLEH value."]
    Pwron = 0x02,
    #[doc = "The BLE Core is powered and active value."]
    Active = 0x03,
    #[doc = "The BLE Core is in shutdown mode value."]
    Shutdown = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "The BLE Core has entered sleep mode and the power request is inactive value."]
    Sleep = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pwrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrst {
    #[inline(always)]
    fn from(val: u8) -> Pwrst {
        Pwrst::from_bits(val)
    }
}
impl From<Pwrst> for u8 {
    #[inline(always)]
    fn from(val: Pwrst) -> u8 {
        Pwrst::to_bits(val)
    }
}
