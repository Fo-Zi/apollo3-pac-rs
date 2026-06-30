#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "This bitfield selects the positive input to the comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::Psel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Psel::from_bits(val as u8)
    }
    #[doc = "This bitfield selects the positive input to the comparator."]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: super::vals::Psel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "This bitfield selects the negative input to the comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn nsel(&self) -> super::vals::Nsel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Nsel::from_bits(val as u8)
    }
    #[doc = "This bitfield selects the negative input to the comparator."]
    #[inline(always)]
    pub const fn set_nsel(&mut self, val: super::vals::Nsel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn lvlsel(&self) -> super::vals::Lvlsel {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Lvlsel::from_bits(val as u8)
    }
    #[doc = "When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[inline(always)]
    pub const fn set_lvlsel(&mut self, val: super::vals::Lvlsel) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("psel", &self.psel())
            .field("nsel", &self.nsel())
            .field("lvlsel", &self.lvlsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ psel: {:?}, nsel: {:?}, lvlsel: {:?} }}",
            self.psel(),
            self.nsel(),
            self.lvlsel()
        )
    }
}
#[doc = "Voltage Comparator Interrupt registers: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "This bit is the vcompout low interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn outlow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the vcompout low interrupt."]
    #[inline(always)]
    pub const fn set_outlow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit is the vcompout high interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn outhi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the vcompout high interrupt."]
    #[inline(always)]
    pub const fn set_outhi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Intclr {
    #[inline(always)]
    fn default() -> Intclr {
        Intclr(0)
    }
}
impl core::fmt::Debug for Intclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intclr")
            .field("outlow", &self.outlow())
            .field("outhi", &self.outhi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intclr {{ outlow: {=bool:?}, outhi: {=bool:?} }}",
            self.outlow(),
            self.outhi()
        )
    }
}
#[doc = "Voltage Comparator Interrupt registers: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "This bit is the vcompout low interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn outlow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the vcompout low interrupt."]
    #[inline(always)]
    pub const fn set_outlow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit is the vcompout high interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn outhi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the vcompout high interrupt."]
    #[inline(always)]
    pub const fn set_outhi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("outlow", &self.outlow())
            .field("outhi", &self.outhi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ outlow: {=bool:?}, outhi: {=bool:?} }}",
            self.outlow(),
            self.outhi()
        )
    }
}
#[doc = "Voltage Comparator Interrupt registers: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "This bit is the vcompout low interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn outlow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the vcompout low interrupt."]
    #[inline(always)]
    pub const fn set_outlow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit is the vcompout high interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn outhi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the vcompout high interrupt."]
    #[inline(always)]
    pub const fn set_outhi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Intset {
    #[inline(always)]
    fn default() -> Intset {
        Intset(0)
    }
}
impl core::fmt::Debug for Intset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intset")
            .field("outlow", &self.outlow())
            .field("outhi", &self.outhi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intset {{ outlow: {=bool:?}, outhi: {=bool:?} }}",
            self.outlow(),
            self.outhi()
        )
    }
}
#[doc = "Voltage Comparator Interrupt registers: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "This bit is the vcompout low interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn outlow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the vcompout low interrupt."]
    #[inline(always)]
    pub const fn set_outlow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit is the vcompout high interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn outhi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the vcompout high interrupt."]
    #[inline(always)]
    pub const fn set_outhi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("outlow", &self.outlow())
            .field("outhi", &self.outhi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ outlow: {=bool:?}, outhi: {=bool:?} }}",
            self.outlow(),
            self.outhi()
        )
    }
}
#[doc = "Key Register for Powering Down the Voltage Comparator."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwdkey(pub u32);
impl Pwdkey {
    #[doc = "Key register value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwdkey(&self) -> super::vals::Pwdkey {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Pwdkey::from_bits(val as u32)
    }
    #[doc = "Key register value."]
    #[inline(always)]
    pub const fn set_pwdkey(&mut self, val: super::vals::Pwdkey) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pwdkey {
    #[inline(always)]
    fn default() -> Pwdkey {
        Pwdkey(0)
    }
}
impl core::fmt::Debug for Pwdkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwdkey")
            .field("pwdkey", &self.pwdkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwdkey {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pwdkey {{ pwdkey: {:?} }}", self.pwdkey())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpout(&self) -> super::vals::Cmpout {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cmpout::from_bits(val as u8)
    }
    #[doc = "This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub const fn set_cmpout(&mut self, val: super::vals::Cmpout) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit indicates the power down state of the voltage comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn pwdstat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the power down state of the voltage comparator."]
    #[inline(always)]
    pub const fn set_pwdstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("cmpout", &self.cmpout())
            .field("pwdstat", &self.pwdstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ cmpout: {:?}, pwdstat: {=bool:?} }}",
            self.cmpout(),
            self.pwdstat()
        )
    }
}
