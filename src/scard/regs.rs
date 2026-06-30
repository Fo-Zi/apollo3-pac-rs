#[doc = "Clock Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkctrl(pub u32);
impl Clkctrl {
    #[doc = "Enable the serial source clock for SCARD."]
    #[must_use]
    #[inline(always)]
    pub const fn clken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the serial source clock for SCARD."]
    #[inline(always)]
    pub const fn set_clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the SCARD APB clock to run continuously."]
    #[must_use]
    #[inline(always)]
    pub const fn apbclken(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the SCARD APB clock to run continuously."]
    #[inline(always)]
    pub const fn set_apbclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Clkctrl {
    #[inline(always)]
    fn default() -> Clkctrl {
        Clkctrl(0)
    }
}
impl core::fmt::Debug for Clkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkctrl")
            .field("clken", &self.clken())
            .field("apbclken", &self.apbclken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkctrl {{ clken: {=bool:?}, apbclken: {=bool:?} }}",
            self.clken(),
            self.apbclken()
        )
    }
}
#[doc = "ISO7816 data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data register."]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr").field("dr", &self.dr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr {{ dr: {=u8:?} }}", self.dr())
    }
}
#[doc = "ISO7816 resent count inquiry."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Retxcntrmi(pub u32);
impl Retxcntrmi {
    #[doc = "Resent count inquiry register."]
    #[must_use]
    #[inline(always)]
    pub const fn retxcntrmi(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resent count inquiry register."]
    #[inline(always)]
    pub const fn set_retxcntrmi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Retxcntrmi {
    #[inline(always)]
    fn default() -> Retxcntrmi {
        Retxcntrmi(0)
    }
}
impl core::fmt::Debug for Retxcntrmi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Retxcntrmi")
            .field("retxcntrmi", &self.retxcntrmi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Retxcntrmi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Retxcntrmi {{ retxcntrmi: {=u8:?} }}", self.retxcntrmi())
    }
}
#[doc = "ISO7816 interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "RX FIFO not empty."]
    #[must_use]
    #[inline(always)]
    pub const fn fne(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO not empty."]
    #[inline(always)]
    pub const fn set_fne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO empty (transmit) or full (receive)."]
    #[must_use]
    #[inline(always)]
    pub const fn tberbf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO empty (transmit) or full (receive)."]
    #[inline(always)]
    pub const fn set_tberbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Framing error."]
    #[must_use]
    #[inline(always)]
    pub const fn fer(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error."]
    #[inline(always)]
    pub const fn set_fer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RX FIFO overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn ovr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO overflow."]
    #[inline(always)]
    pub const fn set_ovr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Parity Error."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TX to RX finished."]
    #[must_use]
    #[inline(always)]
    pub const fn ft2rend(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TX to RX finished."]
    #[inline(always)]
    pub const fn set_ft2rend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "FIFO Half Full."]
    #[must_use]
    #[inline(always)]
    pub const fn fhf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Half Full."]
    #[inline(always)]
    pub const fn set_fhf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("fne", &self.fne())
            .field("tberbf", &self.tberbf())
            .field("fer", &self.fer())
            .field("ovr", &self.ovr())
            .field("pe", &self.pe())
            .field("ft2rend", &self.ft2rend())
            .field("fhf", &self.fhf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sr {{ fne: {=bool:?}, tberbf: {=bool:?}, fer: {=bool:?}, ovr: {=bool:?}, pe: {=bool:?}, ft2rend: {=bool:?}, fhf: {=bool:?} }}" , self . fne () , self . tberbf () , self . fer () , self . ovr () , self . pe () , self . ft2rend () , self . fhf ())
    }
}
#[doc = "ISO7816 interrupt status 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr1(pub u32);
impl Sr1 {
    #[doc = "ETU counter overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn ecntover(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ETU counter overflow."]
    #[inline(always)]
    pub const fn set_ecntover(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Card insert/remove."]
    #[must_use]
    #[inline(always)]
    pub const fn prl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Card insert/remove."]
    #[inline(always)]
    pub const fn set_prl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write complete synchronization."]
    #[must_use]
    #[inline(always)]
    pub const fn syncend(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write complete synchronization."]
    #[inline(always)]
    pub const fn set_syncend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ISO7816 idle."]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ISO7816 idle."]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Sr1 {
    #[inline(always)]
    fn default() -> Sr1 {
        Sr1(0)
    }
}
impl core::fmt::Debug for Sr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr1")
            .field("ecntover", &self.ecntover())
            .field("prl", &self.prl())
            .field("syncend", &self.syncend())
            .field("idle", &self.idle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr1 {{ ecntover: {=bool:?}, prl: {=bool:?}, syncend: {=bool:?}, idle: {=bool:?} }}",
            self.ecntover(),
            self.prl(),
            self.syncend(),
            self.idle()
        )
    }
}
