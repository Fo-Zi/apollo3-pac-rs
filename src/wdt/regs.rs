#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "This bitfield enables the WDT."]
    #[must_use]
    #[inline(always)]
    pub const fn wdten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bitfield enables the WDT."]
    #[inline(always)]
    pub const fn set_wdten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[must_use]
    #[inline(always)]
    pub const fn inten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bitfield enables the WDT interrupt. Note : This bit must be set before the interrupt status bit will reflect a watchdog timer expiration. The IER interrupt register must also be enabled for a WDT interrupt to be sent to the NVIC."]
    #[inline(always)]
    pub const fn set_inten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[must_use]
    #[inline(always)]
    pub const fn resen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bitfield enables the WDT reset. This needs to be set together with the WDREN bit in REG_RSTGEN_CFG register (in reset gen) to trigger the reset."]
    #[inline(always)]
    pub const fn set_resen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[must_use]
    #[inline(always)]
    pub const fn resval(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This bitfield is the compare value for counter bits 7:0 to generate a watchdog reset. This will cause a software reset."]
    #[inline(always)]
    pub const fn set_resval(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn intval(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "This bitfield is the compare value for counter bits 7:0 to generate a watchdog interrupt."]
    #[inline(always)]
    pub const fn set_intval(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> super::vals::Clksel {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Clksel::from_bits(val as u8)
    }
    #[doc = "Select the frequency for the WDT. All values not enumerated below are undefined."]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: super::vals::Clksel) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
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
            .field("wdten", &self.wdten())
            .field("inten", &self.inten())
            .field("resen", &self.resen())
            .field("resval", &self.resval())
            .field("intval", &self.intval())
            .field("clksel", &self.clksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg {{ wdten: {=bool:?}, inten: {=bool:?}, resen: {=bool:?}, resval: {=u8:?}, intval: {=u8:?}, clksel: {:?} }}" , self . wdten () , self . inten () , self . resen () , self . resval () , self . intval () , self . clksel ())
    }
}
#[doc = "Current Counter Value for WDT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[doc = "Read-Only current value of the WDT counter."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read-Only current value of the WDT counter."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Count {
    #[inline(always)]
    fn default() -> Count {
        Count(0)
    }
}
impl core::fmt::Debug for Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Count")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Count {{ count: {=u8:?} }}", self.count())
    }
}
#[doc = "WDT Interrupt register: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "Watchdog Timer Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wdtint(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer Interrupt."]
    #[inline(always)]
    pub const fn set_wdtint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("wdtint", &self.wdtint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intclr {{ wdtint: {=bool:?} }}", self.wdtint())
    }
}
#[doc = "WDT Interrupt register: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Watchdog Timer Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wdtint(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer Interrupt."]
    #[inline(always)]
    pub const fn set_wdtint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("wdtint", &self.wdtint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Inten {{ wdtint: {=bool:?} }}", self.wdtint())
    }
}
#[doc = "WDT Interrupt register: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "Watchdog Timer Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wdtint(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer Interrupt."]
    #[inline(always)]
    pub const fn set_wdtint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("wdtint", &self.wdtint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intset {{ wdtint: {=bool:?} }}", self.wdtint())
    }
}
#[doc = "WDT Interrupt register: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Watchdog Timer Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wdtint(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer Interrupt."]
    #[inline(always)]
    pub const fn set_wdtint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            .field("wdtint", &self.wdtint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intstat {{ wdtint: {=bool:?} }}", self.wdtint())
    }
}
#[doc = "Locks the WDT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Writing 0x3A locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::Lock {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Lock::from_bits(val as u8)
    }
    #[doc = "Writing 0x3A locks the watchdog timer. Once locked, the WDTCFG reg cannot be written and WDTEN is set."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::Lock) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock").field("lock", &self.lock()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lock {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "Restart the watchdog timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstrt(pub u32);
impl Rstrt {
    #[doc = "Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rstrt(&self) -> super::vals::Rstrt {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Rstrt::from_bits(val as u8)
    }
    #[doc = "Writing 0xB2 to WDTRSTRT restarts the watchdog timer. This is a write only register. Reading this register will only provide all 0."]
    #[inline(always)]
    pub const fn set_rstrt(&mut self, val: super::vals::Rstrt) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Rstrt {
    #[inline(always)]
    fn default() -> Rstrt {
        Rstrt(0)
    }
}
impl core::fmt::Debug for Rstrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rstrt")
            .field("rstrt", &self.rstrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstrt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rstrt {{ rstrt: {:?} }}", self.rstrt())
    }
}
