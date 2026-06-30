#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Brown out high (2.1v) reset enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bodhren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Brown out high (2.1v) reset enable."]
    #[inline(always)]
    pub const fn set_bodhren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
    #[must_use]
    #[inline(always)]
    pub const fn wdren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timer Reset Enable. NOTE: The WDT module must also be configured for WDT reset. This includes enabling the RESEN bit in WDTCFG register in Watch dog timer block."]
    #[inline(always)]
    pub const fn set_wdren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .field("bodhren", &self.bodhren())
            .field("wdren", &self.wdren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ bodhren: {=bool:?}, wdren: {=bool:?} }}",
            self.bodhren(),
            self.wdren()
        )
    }
}
#[doc = "Reset Interrupt register: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "Enables an interrupt that triggers when VCC is below BODH level."]
    #[must_use]
    #[inline(always)]
    pub const fn bodh(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub const fn set_bodh(&mut self, val: bool) {
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
            .field("bodh", &self.bodh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intclr {{ bodh: {=bool:?} }}", self.bodh())
    }
}
#[doc = "Reset Interrupt register: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Enables an interrupt that triggers when VCC is below BODH level."]
    #[must_use]
    #[inline(always)]
    pub const fn bodh(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub const fn set_bodh(&mut self, val: bool) {
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
        f.debug_struct("Inten").field("bodh", &self.bodh()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Inten {{ bodh: {=bool:?} }}", self.bodh())
    }
}
#[doc = "Reset Interrupt register: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "Enables an interrupt that triggers when VCC is below BODH level."]
    #[must_use]
    #[inline(always)]
    pub const fn bodh(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub const fn set_bodh(&mut self, val: bool) {
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
            .field("bodh", &self.bodh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intset {{ bodh: {=bool:?} }}", self.bodh())
    }
}
#[doc = "Reset Interrupt register: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Enables an interrupt that triggers when VCC is below BODH level."]
    #[must_use]
    #[inline(always)]
    pub const fn bodh(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables an interrupt that triggers when VCC is below BODH level."]
    #[inline(always)]
    pub const fn set_bodh(&mut self, val: bool) {
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
            .field("bodh", &self.bodh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intstat {{ bodh: {=bool:?} }}", self.bodh())
    }
}
#[doc = "Status Register (SBL)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Reset was initiated by an External Reset (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn exrstat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reset was initiated by an External Reset (SBL)."]
    #[inline(always)]
    pub const fn set_exrstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset was initiated by a Power-On Reset (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn porstat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reset was initiated by a Power-On Reset (SBL)."]
    #[inline(always)]
    pub const fn set_porstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reset was initiated by a Brown-Out Reset (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn borstat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reset was initiated by a Brown-Out Reset (SBL)."]
    #[inline(always)]
    pub const fn set_borstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset was a initiated by SW POR or AIRCR Reset (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn swrstat(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reset was a initiated by SW POR or AIRCR Reset (SBL)."]
    #[inline(always)]
    pub const fn set_swrstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset was a initiated by Software POI Reset (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn poirstat(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset was a initiated by Software POI Reset (SBL)."]
    #[inline(always)]
    pub const fn set_poirstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Reset was a initiated by Debugger Reset (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgrstat(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Reset was a initiated by Debugger Reset (SBL)."]
    #[inline(always)]
    pub const fn set_dbgrstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reset was initiated by a Watchdog Timer Reset (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn wdrstat(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reset was initiated by a Watchdog Timer Reset (SBL)."]
    #[inline(always)]
    pub const fn set_wdrstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "An Unregulated Supply Brownout Event occurred (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn boustat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "An Unregulated Supply Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub const fn set_boustat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "A Core Regulator Brownout Event occurred (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn bocstat(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "A Core Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub const fn set_bocstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "A Memory Regulator Brownout Event occurred (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn bofstat(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "A Memory Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub const fn set_bofstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "A BLE/Burst Regulator Brownout Event occurred (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn bobstat(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "A BLE/Burst Regulator Brownout Event occurred (SBL)."]
    #[inline(always)]
    pub const fn set_bobstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set if current boot was initiated by soft reset and resulted in Fast Boot (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn fboot(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Set if current boot was initiated by soft reset and resulted in Fast Boot (SBL)."]
    #[inline(always)]
    pub const fn set_fboot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set when booting securely (SBL)."]
    #[must_use]
    #[inline(always)]
    pub const fn sboot(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set when booting securely (SBL)."]
    #[inline(always)]
    pub const fn set_sboot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("exrstat", &self.exrstat())
            .field("porstat", &self.porstat())
            .field("borstat", &self.borstat())
            .field("swrstat", &self.swrstat())
            .field("poirstat", &self.poirstat())
            .field("dbgrstat", &self.dbgrstat())
            .field("wdrstat", &self.wdrstat())
            .field("boustat", &self.boustat())
            .field("bocstat", &self.bocstat())
            .field("bofstat", &self.bofstat())
            .field("bobstat", &self.bobstat())
            .field("fboot", &self.fboot())
            .field("sboot", &self.sboot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stat {{ exrstat: {=bool:?}, porstat: {=bool:?}, borstat: {=bool:?}, swrstat: {=bool:?}, poirstat: {=bool:?}, dbgrstat: {=bool:?}, wdrstat: {=bool:?}, boustat: {=bool:?}, bocstat: {=bool:?}, bofstat: {=bool:?}, bobstat: {=bool:?}, fboot: {=bool:?}, sboot: {=bool:?} }}" , self . exrstat () , self . porstat () , self . borstat () , self . swrstat () , self . poirstat () , self . dbgrstat () , self . wdrstat () , self . boustat () , self . bocstat () , self . bofstat () , self . bobstat () , self . fboot () , self . sboot ())
    }
}
#[doc = "Software POI Reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swpoi(pub u32);
impl Swpoi {
    #[doc = "0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
    #[must_use]
    #[inline(always)]
    pub const fn swpoikey(&self) -> super::vals::Swpoikey {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Swpoikey::from_bits(val as u8)
    }
    #[doc = "0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
    #[inline(always)]
    pub const fn set_swpoikey(&mut self, val: super::vals::Swpoikey) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Swpoi {
    #[inline(always)]
    fn default() -> Swpoi {
        Swpoi(0)
    }
}
impl core::fmt::Debug for Swpoi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swpoi")
            .field("swpoikey", &self.swpoikey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swpoi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swpoi {{ swpoikey: {:?} }}", self.swpoikey())
    }
}
#[doc = "Software POR Reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swpor(pub u32);
impl Swpor {
    #[doc = "0xD4 generates a software POR reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swporkey(&self) -> super::vals::Swporkey {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Swporkey::from_bits(val as u8)
    }
    #[doc = "0xD4 generates a software POR reset."]
    #[inline(always)]
    pub const fn set_swporkey(&mut self, val: super::vals::Swporkey) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Swpor {
    #[inline(always)]
    fn default() -> Swpor {
        Swpor(0)
    }
}
impl core::fmt::Debug for Swpor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swpor")
            .field("swporkey", &self.swporkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swpor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swpor {{ swporkey: {:?} }}", self.swporkey())
    }
}
#[doc = "TPIU reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpiurst(pub u32);
impl Tpiurst {
    #[doc = "Static reset for the TPIU. Write to '1' to assert reset to TPIU. Write to '0' to clear the reset."]
    #[must_use]
    #[inline(always)]
    pub const fn tpiurst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Static reset for the TPIU. Write to '1' to assert reset to TPIU. Write to '0' to clear the reset."]
    #[inline(always)]
    pub const fn set_tpiurst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Tpiurst {
    #[inline(always)]
    fn default() -> Tpiurst {
        Tpiurst(0)
    }
}
impl core::fmt::Debug for Tpiurst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tpiurst")
            .field("tpiurst", &self.tpiurst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tpiurst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tpiurst {{ tpiurst: {=bool:?} }}", self.tpiurst())
    }
}
