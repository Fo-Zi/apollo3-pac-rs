#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Blebuckon {
    #[doc = "Indicates the the LDO is supplying the BLE/Burst power domain value."]
    Ldo = 0x0,
    #[doc = "Indicates the the Buck is supplying the BLE/Burst power domain value."]
    Buck = 0x01,
}
impl Blebuckon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Blebuckon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Blebuckon {
    #[inline(always)]
    fn from(val: u8) -> Blebuckon {
        Blebuckon::from_bits(val)
    }
}
impl From<Blebuckon> for u8 {
    #[inline(always)]
    fn from(val: Blebuckon) -> u8 {
        Blebuckon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtcm {
    #[doc = "Do not enable power to any DTCMs value."]
    None = 0x0,
    #[doc = "Power ON only GROUP0_DTCM0 value."]
    Group0dtcm0 = 0x01,
    #[doc = "Power ON only GROUP0_DTCM1 value."]
    Group0dtcm1 = 0x02,
    #[doc = "Power ON only DTCMs in group0 value."]
    Group0 = 0x03,
    #[doc = "Power ON only DTCMs in group1 value."]
    Group1 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Power ON all DTCMs value."]
    All = 0x07,
}
impl Dtcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtcm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtcm {
    #[inline(always)]
    fn from(val: u8) -> Dtcm {
        Dtcm::from_bits(val)
    }
}
impl From<Dtcm> for u8 {
    #[inline(always)]
    fn from(val: Dtcm) -> u8 {
        Dtcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtcmen {
    #[doc = "Do not enable DTCM power-on status event value."]
    None = 0x0,
    #[doc = "Enable GROUP0_DTCM0 power on status event value."]
    Group0dtcm0en = 0x01,
    #[doc = "Enable GROUP0_DTCM1 power on status event value."]
    Group0dtcm1en = 0x02,
    #[doc = "Enable DTCMs in group0 power on status event value."]
    Group0en = 0x03,
    #[doc = "Enable DTCMs in group1 power on status event value."]
    Group1en = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enable all DTCM power on status event value."]
    All = 0x07,
}
impl Dtcmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtcmen {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtcmen {
    #[inline(always)]
    fn from(val: u8) -> Dtcmen {
        Dtcmen::from_bits(val)
    }
}
impl From<Dtcmen> for u8 {
    #[inline(always)]
    fn from(val: Dtcmen) -> u8 {
        Dtcmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtcmpwdslp {
    #[doc = "All DTCM retained value."]
    None = 0x0,
    #[doc = "Group0_DTCM0 powered down in deep sleep (0KB-8KB) value."]
    Group0dtcm0 = 0x01,
    #[doc = "Group0_DTCM1 powered down in deep sleep (8KB-32KB) value."]
    Group0dtcm1 = 0x02,
    #[doc = "Both DTCMs in group0 are powered down in deep sleep (0KB-32KB) value."]
    Group0 = 0x03,
    #[doc = "Group1 DTCM powered down in deep sleep (32KB-64KB) value."]
    Group1 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Group1 and Group0_DTCM1 are powered down in deep sleep (8KB-64KB) value."]
    Allbutgroup0dtcm0 = 0x06,
    #[doc = "All DTCMs powered down in deep sleep (0KB-64KB) value."]
    All = 0x07,
}
impl Dtcmpwdslp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtcmpwdslp {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtcmpwdslp {
    #[inline(always)]
    fn from(val: u8) -> Dtcmpwdslp {
        Dtcmpwdslp::from_bits(val)
    }
}
impl From<Dtcmpwdslp> for u8 {
    #[inline(always)]
    fn from(val: Dtcmpwdslp) -> u8 {
        Dtcmpwdslp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sram(u16);
impl Sram {
    #[doc = "Do not power ON any of the SRAM banks value."]
    pub const None: Self = Self(0x0);
    #[doc = "Power ON only SRAM group0 (0KB-32KB) value."]
    pub const Group0: Self = Self(0x01);
    #[doc = "Power ON only SRAM group1 (32KB-64KB) value."]
    pub const Group1: Self = Self(0x02);
    #[doc = "Power ON only lower 64k value."]
    pub const Sram64k: Self = Self(0x03);
    #[doc = "Power ON only SRAM group2 (64KB-96KB) value."]
    pub const Group2: Self = Self(0x04);
    #[doc = "Power ON only SRAM group3 (96KB-128KB) value."]
    pub const Group3: Self = Self(0x08);
    #[doc = "Power ON only lower 128k value."]
    pub const Sram128k: Self = Self(0x0f);
    #[doc = "Power ON only SRAM group4 (128KB-160KB) value."]
    pub const Group4: Self = Self(0x10);
    #[doc = "Power ON only SRAM group5 (160KB-192KB) value."]
    pub const Group5: Self = Self(0x20);
    #[doc = "Power ON only SRAM group6 (192KB-224KB) value."]
    pub const Group6: Self = Self(0x40);
    #[doc = "Power ON only SRAM group7 (224KB-256KB) value."]
    pub const Group7: Self = Self(0x80);
    #[doc = "Power ON only lower 256k value."]
    pub const Sram256k: Self = Self(0xff);
    #[doc = "Power ON only SRAM group8 (256KB-288KB) value."]
    pub const Group8: Self = Self(0x0100);
    #[doc = "Power ON only SRAM group9 (288KB-320KB) value."]
    pub const Group9: Self = Self(0x0200);
    #[doc = "All SRAM banks (320K) powered ON value."]
    pub const All: Self = Self(0x03ff);
}
impl Sram {
    pub const fn from_bits(val: u16) -> Sram {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Sram {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x01 => f.write_str("Group0"),
            0x02 => f.write_str("Group1"),
            0x03 => f.write_str("Sram64k"),
            0x04 => f.write_str("Group2"),
            0x08 => f.write_str("Group3"),
            0x0f => f.write_str("Sram128k"),
            0x10 => f.write_str("Group4"),
            0x20 => f.write_str("Group5"),
            0x40 => f.write_str("Group6"),
            0x80 => f.write_str("Group7"),
            0xff => f.write_str("Sram256k"),
            0x0100 => f.write_str("Group8"),
            0x0200 => f.write_str("Group9"),
            0x03ff => f.write_str("All"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sram {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x01 => defmt::write!(f, "Group0"),
            0x02 => defmt::write!(f, "Group1"),
            0x03 => defmt::write!(f, "Sram64k"),
            0x04 => defmt::write!(f, "Group2"),
            0x08 => defmt::write!(f, "Group3"),
            0x0f => defmt::write!(f, "Sram128k"),
            0x10 => defmt::write!(f, "Group4"),
            0x20 => defmt::write!(f, "Group5"),
            0x40 => defmt::write!(f, "Group6"),
            0x80 => defmt::write!(f, "Group7"),
            0xff => defmt::write!(f, "Sram256k"),
            0x0100 => defmt::write!(f, "Group8"),
            0x0200 => defmt::write!(f, "Group9"),
            0x03ff => defmt::write!(f, "All"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Sram {
    #[inline(always)]
    fn from(val: u16) -> Sram {
        Sram::from_bits(val)
    }
}
impl From<Sram> for u16 {
    #[inline(always)]
    fn from(val: Sram) -> u16 {
        Sram::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sramen(u16);
impl Sramen {
    #[doc = "Disable SRAM power-on status event value."]
    pub const None: Self = Self(0x0);
    #[doc = "Enable SRAM group0 (0KB-32KB) power on status event value."]
    pub const Group0en: Self = Self(0x01);
    #[doc = "Enable SRAM group1 (32KB-64KB) power on status event value."]
    pub const Group1en: Self = Self(0x02);
    #[doc = "Enable SRAM group2 (64KB-96KB) power on status event value."]
    pub const Group2en: Self = Self(0x04);
    #[doc = "Enable SRAM group3 (96KB-128KB) power on status event value."]
    pub const Group3en: Self = Self(0x08);
    #[doc = "Enable SRAM group4 (128KB-160KB) power on status event value."]
    pub const Group4en: Self = Self(0x10);
    #[doc = "Enable SRAM group5 (160KB-192KB) power on status event value."]
    pub const Group5en: Self = Self(0x20);
    #[doc = "Enable SRAM group6 (192KB-224KB) power on status event value."]
    pub const Group6en: Self = Self(0x40);
    #[doc = "Enable SRAM group7 (224KB-256KB) power on status event value."]
    pub const Group7en: Self = Self(0x80);
    #[doc = "Enable SRAM group8 (256KB-288KB) power on status event value."]
    pub const Group8en: Self = Self(0x0100);
    #[doc = "Enable SRAM group9 (288KB-320KB) power on status event value."]
    pub const Group9en: Self = Self(0x0200);
}
impl Sramen {
    pub const fn from_bits(val: u16) -> Sramen {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Sramen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x01 => f.write_str("Group0en"),
            0x02 => f.write_str("Group1en"),
            0x04 => f.write_str("Group2en"),
            0x08 => f.write_str("Group3en"),
            0x10 => f.write_str("Group4en"),
            0x20 => f.write_str("Group5en"),
            0x40 => f.write_str("Group6en"),
            0x80 => f.write_str("Group7en"),
            0x0100 => f.write_str("Group8en"),
            0x0200 => f.write_str("Group9en"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramen {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x01 => defmt::write!(f, "Group0en"),
            0x02 => defmt::write!(f, "Group1en"),
            0x04 => defmt::write!(f, "Group2en"),
            0x08 => defmt::write!(f, "Group3en"),
            0x10 => defmt::write!(f, "Group4en"),
            0x20 => defmt::write!(f, "Group5en"),
            0x40 => defmt::write!(f, "Group6en"),
            0x80 => defmt::write!(f, "Group7en"),
            0x0100 => defmt::write!(f, "Group8en"),
            0x0200 => defmt::write!(f, "Group9en"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Sramen {
    #[inline(always)]
    fn from(val: u16) -> Sramen {
        Sramen::from_bits(val)
    }
}
impl From<Sramen> for u16 {
    #[inline(always)]
    fn from(val: Sramen) -> u16 {
        Sramen::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sramlightsleep(u16);
impl Sramlightsleep {
    #[doc = "Disables LIGHT SLEEP for ALL SRAMs value."]
    pub const Dis: Self = Self(0x0);
    #[doc = "Enable LIGHT SLEEP for ALL SRAMs value."]
    pub const All: Self = Self(0xff);
}
impl Sramlightsleep {
    pub const fn from_bits(val: u16) -> Sramlightsleep {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Sramlightsleep {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Dis"),
            0xff => f.write_str("All"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramlightsleep {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Dis"),
            0xff => defmt::write!(f, "All"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Sramlightsleep {
    #[inline(always)]
    fn from(val: u16) -> Sramlightsleep {
        Sramlightsleep::from_bits(val)
    }
}
impl From<Sramlightsleep> for u16 {
    #[inline(always)]
    fn from(val: Sramlightsleep) -> u16 {
        Sramlightsleep::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Srampwdslp(u16);
impl Srampwdslp {
    #[doc = "All banks retained value."]
    pub const None: Self = Self(0x0);
    #[doc = "SRAM GROUP0 powered down (64KB-96KB) value."]
    pub const Group0: Self = Self(0x01);
    #[doc = "SRAM GROUP1 powered down (96KB-128KB) value."]
    pub const Group1: Self = Self(0x02);
    #[doc = "Powerdown lower 64k SRAM (64KB-128KB) value."]
    pub const Sram64k: Self = Self(0x03);
    #[doc = "SRAM GROUP2 powered down (128KB-160KB) value."]
    pub const Group2: Self = Self(0x04);
    #[doc = "SRAM GROUP3 powered down (160KB-192KB) value."]
    pub const Group3: Self = Self(0x08);
    #[doc = "Powerdown lower 128k SRAM (64KB-192KB) value."]
    pub const Sram128k: Self = Self(0x0f);
    #[doc = "SRAM GROUP4 powered down (192KB-224KB) value."]
    pub const Group4: Self = Self(0x10);
    #[doc = "SRAM GROUP5 powered down (224KB-256KB) value."]
    pub const Group5: Self = Self(0x20);
    #[doc = "SRAM GROUP6 powered down (256KB-288KB) value."]
    pub const Group6: Self = Self(0x40);
    #[doc = "SRAM GROUP7 powered down (288KB-320KB) value."]
    pub const Group7: Self = Self(0x80);
    #[doc = "SRAM GROUP8 powered down (320KB-352KB) value."]
    pub const Group8: Self = Self(0x0100);
    #[doc = "SRAM GROUP9 powered down (352KB-384KB) value."]
    pub const Group9: Self = Self(0x0200);
    #[doc = "All banks but lower 128k powered down. value."]
    pub const Allbutlower128k: Self = Self(0x03f0);
    #[doc = "All banks but lower 64k powered down. value."]
    pub const Allbutlower64k: Self = Self(0x03fc);
    #[doc = "All SRAM banks but lower 32k powered down (96KB-384KB). value."]
    pub const Allbutlower32k: Self = Self(0x03fe);
    #[doc = "All banks powered down. value."]
    pub const All: Self = Self(0x03ff);
}
impl Srampwdslp {
    pub const fn from_bits(val: u16) -> Srampwdslp {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Srampwdslp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x01 => f.write_str("Group0"),
            0x02 => f.write_str("Group1"),
            0x03 => f.write_str("Sram64k"),
            0x04 => f.write_str("Group2"),
            0x08 => f.write_str("Group3"),
            0x0f => f.write_str("Sram128k"),
            0x10 => f.write_str("Group4"),
            0x20 => f.write_str("Group5"),
            0x40 => f.write_str("Group6"),
            0x80 => f.write_str("Group7"),
            0x0100 => f.write_str("Group8"),
            0x0200 => f.write_str("Group9"),
            0x03f0 => f.write_str("Allbutlower128k"),
            0x03fc => f.write_str("Allbutlower64k"),
            0x03fe => f.write_str("Allbutlower32k"),
            0x03ff => f.write_str("All"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srampwdslp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x01 => defmt::write!(f, "Group0"),
            0x02 => defmt::write!(f, "Group1"),
            0x03 => defmt::write!(f, "Sram64k"),
            0x04 => defmt::write!(f, "Group2"),
            0x08 => defmt::write!(f, "Group3"),
            0x0f => defmt::write!(f, "Sram128k"),
            0x10 => defmt::write!(f, "Group4"),
            0x20 => defmt::write!(f, "Group5"),
            0x40 => defmt::write!(f, "Group6"),
            0x80 => defmt::write!(f, "Group7"),
            0x0100 => defmt::write!(f, "Group8"),
            0x0200 => defmt::write!(f, "Group9"),
            0x03f0 => defmt::write!(f, "Allbutlower128k"),
            0x03fc => defmt::write!(f, "Allbutlower64k"),
            0x03fe => defmt::write!(f, "Allbutlower32k"),
            0x03ff => defmt::write!(f, "All"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Srampwdslp {
    #[inline(always)]
    fn from(val: u16) -> Srampwdslp {
        Srampwdslp::from_bits(val)
    }
}
impl From<Srampwdslp> for u16 {
    #[inline(always)]
    fn from(val: Srampwdslp) -> u16 {
        Srampwdslp::to_bits(val)
    }
}
