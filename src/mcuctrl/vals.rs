#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acwarmup {
    #[doc = "Warmup period of 1-2 seconds value."]
    Sec1 = 0x0,
    #[doc = "Warmup period of 2-4 seconds value."]
    Sec2 = 0x01,
    #[doc = "Warmup period of 4-8 seconds value."]
    Sec4 = 0x02,
    #[doc = "Warmup period of 8-16 seconds value."]
    Sec8 = 0x03,
}
impl Acwarmup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acwarmup {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acwarmup {
    #[inline(always)]
    fn from(val: u8) -> Acwarmup {
        Acwarmup::from_bits(val)
    }
}
impl From<Acwarmup> for u8 {
    #[inline(always)]
    fn from(val: Acwarmup) -> u8 {
        Acwarmup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bypcmprxtal {
    #[doc = "Use the XTAL oscillator comparator. value."]
    Usecomp = 0x0,
    #[doc = "Bypass the XTAL oscillator comparator. value."]
    Bypcomp = 0x01,
}
impl Bypcmprxtal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bypcmprxtal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bypcmprxtal {
    #[inline(always)]
    fn from(val: u8) -> Bypcmprxtal {
        Bypcmprxtal::from_bits(val)
    }
}
impl From<Bypcmprxtal> for u8 {
    #[inline(always)]
    fn from(val: Bypcmprxtal) -> u8 {
        Bypcmprxtal::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Chipid0(u32);
impl Chipid0 {
    #[doc = "Apollo3 CHIPID0. value."]
    pub const Apollo3: Self = Self(0x0);
}
impl Chipid0 {
    pub const fn from_bits(val: u32) -> Chipid0 {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Chipid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Apollo3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chipid0 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Apollo3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Chipid0 {
    #[inline(always)]
    fn from(val: u32) -> Chipid0 {
        Chipid0::from_bits(val)
    }
}
impl From<Chipid0> for u32 {
    #[inline(always)]
    fn from(val: Chipid0) -> u32 {
        Chipid0::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Chipid1(u32);
impl Chipid1 {
    #[doc = "Apollo3 CHIPID1. value."]
    pub const Apollo3: Self = Self(0x0);
}
impl Chipid1 {
    pub const fn from_bits(val: u32) -> Chipid1 {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Chipid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Apollo3"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chipid1 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Apollo3"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Chipid1 {
    #[inline(always)]
    fn from(val: u32) -> Chipid1 {
        Chipid1::from_bits(val)
    }
}
impl From<Chipid1> for u32 {
    #[inline(always)]
    fn from(val: Chipid1) -> u32 {
        Chipid1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksel {
    #[doc = "Low power state. value."]
    Lowpwr = 0x0,
    #[doc = "Selects HFRC divided by 2 as the source TPIU clk value."]
    Hfrcdiv2 = 0x01,
    #[doc = "Selects HFRC divided by 8 as the source TPIU clk value."]
    Hfrcdiv8 = 0x02,
    #[doc = "Selects HFRC divided by 16 as the source TPIU clk value."]
    Hfrcdiv16 = 0x03,
    #[doc = "Selects HFRC divided by 32 as the source TPIU clk value."]
    Hfrcdiv32 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Clksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksel {
    #[inline(always)]
    fn from(val: u8) -> Clksel {
        Clksel::from_bits(val)
    }
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(val: Clksel) -> u8 {
        Clksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fdbkdsblxtal {
    #[doc = "Enable XTAL oscillator comparator. value."]
    En = 0x0,
    #[doc = "Disable XTAL oscillator comparator. value."]
    Dis = 0x01,
}
impl Fdbkdsblxtal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdbkdsblxtal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdbkdsblxtal {
    #[inline(always)]
    fn from(val: u8) -> Fdbkdsblxtal {
        Fdbkdsblxtal::from_bits(val)
    }
}
impl From<Fdbkdsblxtal> for u8 {
    #[inline(always)]
    fn from(val: Fdbkdsblxtal) -> u8 {
        Fdbkdsblxtal::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kextclksel(u32);
impl Kextclksel {
    #[doc = "Key value."]
    pub const Key: Self = Self(0x53);
}
impl Kextclksel {
    pub const fn from_bits(val: u32) -> Kextclksel {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Kextclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x53 => f.write_str("Key"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kextclksel {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x53 => defmt::write!(f, "Key"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Kextclksel {
    #[inline(always)]
    fn from(val: u32) -> Kextclksel {
        Kextclksel::from_bits(val)
    }
}
impl From<Kextclksel> for u32 {
    #[inline(always)]
    fn from(val: Kextclksel) -> u32 {
        Kextclksel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Partnum(u32);
impl Partnum {
    #[doc = "Bit position for the qualified field. value."]
    pub const QualS: Self = Self(0x0);
    #[doc = "Bit position for the temperature field. value."]
    pub const TempS: Self = Self(0x01);
    #[doc = "Bit position for the pins field. value."]
    pub const PinsS: Self = Self(0x03);
    #[doc = "Bit position for the package field. value."]
    pub const PkgS: Self = Self(0x06);
    #[doc = "Bit position for the revision field. value."]
    pub const RevS: Self = Self(0x08);
    #[doc = "Bit position for the SRAM_SIZE field. value."]
    pub const SramsizeS: Self = Self(0x10);
    #[doc = "Bit position for the FLASH_SIZE field. value."]
    pub const FlashsizeS: Self = Self(0x14);
    #[doc = "Bit position for the part number field. value."]
    pub const PnS: Self = Self(0x18);
    #[doc = "Mask for the pins field. Values: 0: 25 pins 1: 49 pins 2: 64 pins 3: 81 pins value."]
    pub const PinsM: Self = Self(0x38);
    #[doc = "Mask for the package field. Values: 0: SIP 1: QFN 2: BGA 3: CSP value."]
    pub const PkgM: Self = Self(0xc0);
    #[doc = "Mask for the revision field. Bits \\[15:12\\] are major rev, \\[11:8\\] are minor rev. Values: 0: Major Rev A, Minor Rev 0 1: Major Rev B, Minor Rev 1 value."]
    pub const RevM: Self = Self(0xff00);
    #[doc = "Mask for the SRAM_SIZE field. Values: 0: 16KB 1: 32KB 2: 64KB 3: 128KB 4: 256KB 5: 512KB 6: 1MB 7: 384KB value."]
    pub const SramsizeM: Self = Self(0x000f_0000);
    #[doc = "Mask for the FLASH_SIZE field. Values: 0: 16KB 1: 32KB 2: 64KB 3: 128KB 4: 256KB 5: 512KB 6: 1MB 7: 2MB value."]
    pub const FlashsizeM: Self = Self(0x00f0_0000);
    #[doc = "Apollo part number is 0x01xxxxxx. value."]
    pub const Apollo: Self = Self(0x0100_0000);
    #[doc = "Apollo2 part number is 0x03xxxxxx. value."]
    pub const Apollo2: Self = Self(0x0300_0000);
    #[doc = "Apollo3 part number is 0x06xxxxxx. value."]
    pub const Apollo3: Self = Self(0x0600_0000);
    #[doc = "Mask for the part number field. value."]
    pub const PnM: Self = Self(0xff00_0000);
}
impl Partnum {
    pub const fn from_bits(val: u32) -> Partnum {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Partnum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("QualS"),
            0x01 => f.write_str("TempS"),
            0x03 => f.write_str("PinsS"),
            0x06 => f.write_str("PkgS"),
            0x08 => f.write_str("RevS"),
            0x10 => f.write_str("SramsizeS"),
            0x14 => f.write_str("FlashsizeS"),
            0x18 => f.write_str("PnS"),
            0x38 => f.write_str("PinsM"),
            0xc0 => f.write_str("PkgM"),
            0xff00 => f.write_str("RevM"),
            0x000f_0000 => f.write_str("SramsizeM"),
            0x00f0_0000 => f.write_str("FlashsizeM"),
            0x0100_0000 => f.write_str("Apollo"),
            0x0300_0000 => f.write_str("Apollo2"),
            0x0600_0000 => f.write_str("Apollo3"),
            0xff00_0000 => f.write_str("PnM"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Partnum {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "QualS"),
            0x01 => defmt::write!(f, "TempS"),
            0x03 => defmt::write!(f, "PinsS"),
            0x06 => defmt::write!(f, "PkgS"),
            0x08 => defmt::write!(f, "RevS"),
            0x10 => defmt::write!(f, "SramsizeS"),
            0x14 => defmt::write!(f, "FlashsizeS"),
            0x18 => defmt::write!(f, "PnS"),
            0x38 => defmt::write!(f, "PinsM"),
            0xc0 => defmt::write!(f, "PkgM"),
            0xff00 => defmt::write!(f, "RevM"),
            0x000f_0000 => defmt::write!(f, "SramsizeM"),
            0x00f0_0000 => defmt::write!(f, "FlashsizeM"),
            0x0100_0000 => defmt::write!(f, "Apollo"),
            0x0300_0000 => defmt::write!(f, "Apollo2"),
            0x0600_0000 => defmt::write!(f, "Apollo3"),
            0xff00_0000 => defmt::write!(f, "PnM"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Partnum {
    #[inline(always)]
    fn from(val: u32) -> Partnum {
        Partnum::from_bits(val)
    }
}
impl From<Partnum> for u32 {
    #[inline(always)]
    fn from(val: Partnum) -> u32 {
        Partnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdnbcmprxtal {
    #[doc = "Power down XTAL oscillator comparator. value."]
    Pwrdncomp = 0x0,
    #[doc = "Power up XTAL oscillator comparator. value."]
    Pwrupcomp = 0x01,
}
impl Pdnbcmprxtal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdnbcmprxtal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdnbcmprxtal {
    #[inline(always)]
    fn from(val: u8) -> Pdnbcmprxtal {
        Pdnbcmprxtal::from_bits(val)
    }
}
impl From<Pdnbcmprxtal> for u8 {
    #[inline(always)]
    fn from(val: Pdnbcmprxtal) -> u8 {
        Pdnbcmprxtal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdnbcorextal {
    #[doc = "Power down XTAL oscillator core. value."]
    Pwrdncore = 0x0,
    #[doc = "Power up XTAL oscillator core. value."]
    Pwrupcore = 0x01,
}
impl Pdnbcorextal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdnbcorextal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdnbcorextal {
    #[inline(always)]
    fn from(val: u8) -> Pdnbcorextal {
        Pdnbcorextal::from_bits(val)
    }
}
impl From<Pdnbcorextal> for u8 {
    #[inline(always)]
    fn from(val: Pdnbcorextal) -> u8 {
        Pdnbcorextal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwdbodxtal {
    #[doc = "Power up xtal on BOD value."]
    Pwrupbod = 0x0,
    #[doc = "Power down XTAL on BOD. value."]
    Pwrdnbod = 0x01,
}
impl Pwdbodxtal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwdbodxtal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwdbodxtal {
    #[inline(always)]
    fn from(val: u8) -> Pwdbodxtal {
        Pwdbodxtal::from_bits(val)
    }
}
impl From<Pwdbodxtal> for u8 {
    #[inline(always)]
    fn from(val: Pwdbodxtal) -> u8 {
        Pwdbodxtal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Revmaj {
    _RESERVED_0 = 0x0,
    #[doc = "Apollo3 revision A value."]
    A = 0x01,
    #[doc = "Apollo3 revision B value."]
    B = 0x02,
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
impl Revmaj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Revmaj {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Revmaj {
    #[inline(always)]
    fn from(val: u8) -> Revmaj {
        Revmaj::from_bits(val)
    }
}
impl From<Revmaj> for u8 {
    #[inline(always)]
    fn from(val: Revmaj) -> u8 {
        Revmaj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Revmin {
    _RESERVED_0 = 0x0,
    #[doc = "Apollo3 minor rev 0. Minor revision value, succeeding minor revisions will increment from this value. value."]
    Rev0 = 0x01,
    #[doc = "Apollo3 minor rev 1. value."]
    Rev1 = 0x02,
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
impl Revmin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Revmin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Revmin {
    #[inline(always)]
    fn from(val: u8) -> Revmin {
        Revmin::from_bits(val)
    }
}
impl From<Revmin> for u8 {
    #[inline(always)]
    fn from(val: Revmin) -> u8 {
        Revmin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Secbootstatus {
    #[doc = "Secure boot disabled value."]
    Disabled = 0x0,
    #[doc = "Secure boot enabled value."]
    Enabled = 0x01,
    #[doc = "Error in secure boot configuration value."]
    Error = 0x02,
    _RESERVED_3 = 0x03,
}
impl Secbootstatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Secbootstatus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Secbootstatus {
    #[inline(always)]
    fn from(val: u8) -> Secbootstatus {
        Secbootstatus::from_bits(val)
    }
}
impl From<Secbootstatus> for u8 {
    #[inline(always)]
    fn from(val: Secbootstatus) -> u8 {
        Secbootstatus::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vendorid(u32);
impl Vendorid {
    #[doc = "Ambiq Vendor ID value."]
    pub const Ambiq: Self = Self(0x414d_4251);
}
impl Vendorid {
    pub const fn from_bits(val: u32) -> Vendorid {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Vendorid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x414d_4251 => f.write_str("Ambiq"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vendorid {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x414d_4251 => defmt::write!(f, "Ambiq"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Vendorid {
    #[inline(always)]
    fn from(val: u32) -> Vendorid {
        Vendorid::from_bits(val)
    }
}
impl From<Vendorid> for u32 {
    #[inline(always)]
    fn from(val: Vendorid) -> u32 {
        Vendorid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xtalswe {
    #[doc = "XTAL Software Override Disable. value."]
    OverrideDis = 0x0,
    #[doc = "XTAL Software Override Enable. value."]
    OverrideEn = 0x01,
}
impl Xtalswe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xtalswe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xtalswe {
    #[inline(always)]
    fn from(val: u8) -> Xtalswe {
        Xtalswe::from_bits(val)
    }
}
impl From<Xtalswe> for u8 {
    #[inline(always)]
    fn from(val: Xtalswe) -> u8 {
        Xtalswe::to_bits(val)
    }
}
