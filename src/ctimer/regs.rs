#[doc = "Counter/Timer Auxiliary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux0(pub u32);
impl Aux0 {
    #[doc = "Counter/Timer A0 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0lmt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Counter/Timer A0 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmra0lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Counter/Timer A0 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0trig(&self) -> super::vals::Tmra0trig {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::Tmra0trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A0 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmra0trig(&mut self, val: super::vals::Tmra0trig) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmra0nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A0 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0tinv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmra0tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer A0 Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A0 Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmra0pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A0 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A0 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmra0en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B0 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0lmt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Counter/Timer B0 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmrb0lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Counter/Timer B0 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0trig(&self) -> super::vals::Tmrb0trig {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Tmrb0trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B0 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmrb0trig(&mut self, val: super::vals::Tmrb0trig) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmrb0nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B0 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0tinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmrb0tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmrb0pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer B0 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B0 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmrb0en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Aux0 {
    #[inline(always)]
    fn default() -> Aux0 {
        Aux0(0)
    }
}
impl core::fmt::Debug for Aux0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux0")
            .field("tmra0lmt", &self.tmra0lmt())
            .field("tmra0trig", &self.tmra0trig())
            .field("tmra0nosync", &self.tmra0nosync())
            .field("tmra0tinv", &self.tmra0tinv())
            .field("tmra0pol23", &self.tmra0pol23())
            .field("tmra0en23", &self.tmra0en23())
            .field("tmrb0lmt", &self.tmrb0lmt())
            .field("tmrb0trig", &self.tmrb0trig())
            .field("tmrb0nosync", &self.tmrb0nosync())
            .field("tmrb0tinv", &self.tmrb0tinv())
            .field("tmrb0pol23", &self.tmrb0pol23())
            .field("tmrb0en23", &self.tmrb0en23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aux0 {{ tmra0lmt: {=u8:?}, tmra0trig: {:?}, tmra0nosync: {:?}, tmra0tinv: {=bool:?}, tmra0pol23: {:?}, tmra0en23: {:?}, tmrb0lmt: {=u8:?}, tmrb0trig: {:?}, tmrb0nosync: {:?}, tmrb0tinv: {=bool:?}, tmrb0pol23: {:?}, tmrb0en23: {:?} }}" , self . tmra0lmt () , self . tmra0trig () , self . tmra0nosync () , self . tmra0tinv () , self . tmra0pol23 () , self . tmra0en23 () , self . tmrb0lmt () , self . tmrb0trig () , self . tmrb0nosync () , self . tmrb0tinv () , self . tmrb0pol23 () , self . tmrb0en23 ())
    }
}
#[doc = "Counter/Timer Auxiliary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux1(pub u32);
impl Aux1 {
    #[doc = "Counter/Timer A1 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1lmt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Counter/Timer A1 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmra1lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Counter/Timer A1 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1trig(&self) -> super::vals::Tmra1trig {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::Tmra1trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A1 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmra1trig(&mut self, val: super::vals::Tmra1trig) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmra1nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A1 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1tinv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmra1tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer A1 Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A1 Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmra1pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A1 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A1 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmra1en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B1 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1lmt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Counter/Timer B1 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmrb1lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Counter/Timer B1 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1trig(&self) -> super::vals::Tmrb1trig {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Tmrb1trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B1 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmrb1trig(&mut self, val: super::vals::Tmrb1trig) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmrb1nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B1 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1tinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmrb1tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmrb1pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer B1 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B1 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmrb1en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Aux1 {
    #[inline(always)]
    fn default() -> Aux1 {
        Aux1(0)
    }
}
impl core::fmt::Debug for Aux1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux1")
            .field("tmra1lmt", &self.tmra1lmt())
            .field("tmra1trig", &self.tmra1trig())
            .field("tmra1nosync", &self.tmra1nosync())
            .field("tmra1tinv", &self.tmra1tinv())
            .field("tmra1pol23", &self.tmra1pol23())
            .field("tmra1en23", &self.tmra1en23())
            .field("tmrb1lmt", &self.tmrb1lmt())
            .field("tmrb1trig", &self.tmrb1trig())
            .field("tmrb1nosync", &self.tmrb1nosync())
            .field("tmrb1tinv", &self.tmrb1tinv())
            .field("tmrb1pol23", &self.tmrb1pol23())
            .field("tmrb1en23", &self.tmrb1en23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aux1 {{ tmra1lmt: {=u8:?}, tmra1trig: {:?}, tmra1nosync: {:?}, tmra1tinv: {=bool:?}, tmra1pol23: {:?}, tmra1en23: {:?}, tmrb1lmt: {=u8:?}, tmrb1trig: {:?}, tmrb1nosync: {:?}, tmrb1tinv: {=bool:?}, tmrb1pol23: {:?}, tmrb1en23: {:?} }}" , self . tmra1lmt () , self . tmra1trig () , self . tmra1nosync () , self . tmra1tinv () , self . tmra1pol23 () , self . tmra1en23 () , self . tmrb1lmt () , self . tmrb1trig () , self . tmrb1nosync () , self . tmrb1tinv () , self . tmrb1pol23 () , self . tmrb1en23 ())
    }
}
#[doc = "Counter/Timer Auxiliary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux2(pub u32);
impl Aux2 {
    #[doc = "Counter/Timer A2 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2lmt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Counter/Timer A2 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmra2lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Counter/Timer A2 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2trig(&self) -> super::vals::Tmra2trig {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::Tmra2trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A2 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmra2trig(&mut self, val: super::vals::Tmra2trig) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmra2nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A2 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2tinv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmra2tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer A2 Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A2 Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmra2pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A2 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A2 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmra2en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B2 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2lmt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Counter/Timer B2 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmrb2lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Counter/Timer B2 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2trig(&self) -> super::vals::Tmrb2trig {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Tmrb2trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B2 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmrb2trig(&mut self, val: super::vals::Tmrb2trig) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmrb2nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B2 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2tinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmrb2tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmrb2pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer B2 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B2 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmrb2en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Aux2 {
    #[inline(always)]
    fn default() -> Aux2 {
        Aux2(0)
    }
}
impl core::fmt::Debug for Aux2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux2")
            .field("tmra2lmt", &self.tmra2lmt())
            .field("tmra2trig", &self.tmra2trig())
            .field("tmra2nosync", &self.tmra2nosync())
            .field("tmra2tinv", &self.tmra2tinv())
            .field("tmra2pol23", &self.tmra2pol23())
            .field("tmra2en23", &self.tmra2en23())
            .field("tmrb2lmt", &self.tmrb2lmt())
            .field("tmrb2trig", &self.tmrb2trig())
            .field("tmrb2nosync", &self.tmrb2nosync())
            .field("tmrb2tinv", &self.tmrb2tinv())
            .field("tmrb2pol23", &self.tmrb2pol23())
            .field("tmrb2en23", &self.tmrb2en23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aux2 {{ tmra2lmt: {=u8:?}, tmra2trig: {:?}, tmra2nosync: {:?}, tmra2tinv: {=bool:?}, tmra2pol23: {:?}, tmra2en23: {:?}, tmrb2lmt: {=u8:?}, tmrb2trig: {:?}, tmrb2nosync: {:?}, tmrb2tinv: {=bool:?}, tmrb2pol23: {:?}, tmrb2en23: {:?} }}" , self . tmra2lmt () , self . tmra2trig () , self . tmra2nosync () , self . tmra2tinv () , self . tmra2pol23 () , self . tmra2en23 () , self . tmrb2lmt () , self . tmrb2trig () , self . tmrb2nosync () , self . tmrb2tinv () , self . tmrb2pol23 () , self . tmrb2en23 ())
    }
}
#[doc = "Counter/Timer Auxiliary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux3(pub u32);
impl Aux3 {
    #[doc = "Counter/Timer A3 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3lmt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Counter/Timer A3 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmra3lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Counter/Timer A3 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3trig(&self) -> super::vals::Tmra3trig {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::Tmra3trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A3 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmra3trig(&mut self, val: super::vals::Tmra3trig) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmra3nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A3 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3tinv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmra3tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer A3 Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A3 Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmra3pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A3 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A3 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmra3en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B3 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3lmt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Counter/Timer B3 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmrb3lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Counter/Timer B3 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3trig(&self) -> super::vals::Tmrb3trig {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Tmrb3trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B3 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmrb3trig(&mut self, val: super::vals::Tmrb3trig) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmrb3nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B3 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3tinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmrb3tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmrb3pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer B3 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B3 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmrb3en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Aux3 {
    #[inline(always)]
    fn default() -> Aux3 {
        Aux3(0)
    }
}
impl core::fmt::Debug for Aux3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux3")
            .field("tmra3lmt", &self.tmra3lmt())
            .field("tmra3trig", &self.tmra3trig())
            .field("tmra3nosync", &self.tmra3nosync())
            .field("tmra3tinv", &self.tmra3tinv())
            .field("tmra3pol23", &self.tmra3pol23())
            .field("tmra3en23", &self.tmra3en23())
            .field("tmrb3lmt", &self.tmrb3lmt())
            .field("tmrb3trig", &self.tmrb3trig())
            .field("tmrb3nosync", &self.tmrb3nosync())
            .field("tmrb3tinv", &self.tmrb3tinv())
            .field("tmrb3pol23", &self.tmrb3pol23())
            .field("tmrb3en23", &self.tmrb3en23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aux3 {{ tmra3lmt: {=u8:?}, tmra3trig: {:?}, tmra3nosync: {:?}, tmra3tinv: {=bool:?}, tmra3pol23: {:?}, tmra3en23: {:?}, tmrb3lmt: {=u8:?}, tmrb3trig: {:?}, tmrb3nosync: {:?}, tmrb3tinv: {=bool:?}, tmrb3pol23: {:?}, tmrb3en23: {:?} }}" , self . tmra3lmt () , self . tmra3trig () , self . tmra3nosync () , self . tmra3tinv () , self . tmra3pol23 () , self . tmra3en23 () , self . tmrb3lmt () , self . tmrb3trig () , self . tmrb3nosync () , self . tmrb3tinv () , self . tmrb3pol23 () , self . tmrb3en23 ())
    }
}
#[doc = "Counter/Timer Auxiliary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux4(pub u32);
impl Aux4 {
    #[doc = "Counter/Timer A4 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4lmt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Counter/Timer A4 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmra4lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Counter/Timer A4 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4trig(&self) -> super::vals::Tmra4trig {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::Tmra4trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A4 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmra4trig(&mut self, val: super::vals::Tmra4trig) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmra4nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A4 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4tinv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmra4tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer A4 Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A4 Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmra4pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A4 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A4 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmra4en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B4 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4lmt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Counter/Timer B4 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmrb4lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Counter/Timer B4 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4trig(&self) -> super::vals::Tmrb4trig {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Tmrb4trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B4 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmrb4trig(&mut self, val: super::vals::Tmrb4trig) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmrb4nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B4 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4tinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmrb4tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmrb4pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer B4 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B4 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmrb4en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Aux4 {
    #[inline(always)]
    fn default() -> Aux4 {
        Aux4(0)
    }
}
impl core::fmt::Debug for Aux4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux4")
            .field("tmra4lmt", &self.tmra4lmt())
            .field("tmra4trig", &self.tmra4trig())
            .field("tmra4nosync", &self.tmra4nosync())
            .field("tmra4tinv", &self.tmra4tinv())
            .field("tmra4pol23", &self.tmra4pol23())
            .field("tmra4en23", &self.tmra4en23())
            .field("tmrb4lmt", &self.tmrb4lmt())
            .field("tmrb4trig", &self.tmrb4trig())
            .field("tmrb4nosync", &self.tmrb4nosync())
            .field("tmrb4tinv", &self.tmrb4tinv())
            .field("tmrb4pol23", &self.tmrb4pol23())
            .field("tmrb4en23", &self.tmrb4en23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aux4 {{ tmra4lmt: {=u8:?}, tmra4trig: {:?}, tmra4nosync: {:?}, tmra4tinv: {=bool:?}, tmra4pol23: {:?}, tmra4en23: {:?}, tmrb4lmt: {=u8:?}, tmrb4trig: {:?}, tmrb4nosync: {:?}, tmrb4tinv: {=bool:?}, tmrb4pol23: {:?}, tmrb4en23: {:?} }}" , self . tmra4lmt () , self . tmra4trig () , self . tmra4nosync () , self . tmra4tinv () , self . tmra4pol23 () , self . tmra4en23 () , self . tmrb4lmt () , self . tmrb4trig () , self . tmrb4nosync () , self . tmrb4tinv () , self . tmrb4pol23 () , self . tmrb4en23 ())
    }
}
#[doc = "Counter/Timer Auxiliary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux5(pub u32);
impl Aux5 {
    #[doc = "Counter/Timer A5 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5lmt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Counter/Timer A5 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmra5lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Counter/Timer A5 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5trig(&self) -> super::vals::Tmra5trig {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::Tmra5trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A5 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmra5trig(&mut self, val: super::vals::Tmra5trig) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmra5nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A5 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5tinv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmra5tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer A5 Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A5 Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmra5pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A5 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A5 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmra5en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B5 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5lmt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Counter/Timer B5 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmrb5lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Counter/Timer B5 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5trig(&self) -> super::vals::Tmrb5trig {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Tmrb5trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B5 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmrb5trig(&mut self, val: super::vals::Tmrb5trig) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmrb5nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B5 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5tinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmrb5tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmrb5pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer B5 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B5 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmrb5en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Aux5 {
    #[inline(always)]
    fn default() -> Aux5 {
        Aux5(0)
    }
}
impl core::fmt::Debug for Aux5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux5")
            .field("tmra5lmt", &self.tmra5lmt())
            .field("tmra5trig", &self.tmra5trig())
            .field("tmra5nosync", &self.tmra5nosync())
            .field("tmra5tinv", &self.tmra5tinv())
            .field("tmra5pol23", &self.tmra5pol23())
            .field("tmra5en23", &self.tmra5en23())
            .field("tmrb5lmt", &self.tmrb5lmt())
            .field("tmrb5trig", &self.tmrb5trig())
            .field("tmrb5nosync", &self.tmrb5nosync())
            .field("tmrb5tinv", &self.tmrb5tinv())
            .field("tmrb5pol23", &self.tmrb5pol23())
            .field("tmrb5en23", &self.tmrb5en23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aux5 {{ tmra5lmt: {=u8:?}, tmra5trig: {:?}, tmra5nosync: {:?}, tmra5tinv: {=bool:?}, tmra5pol23: {:?}, tmra5en23: {:?}, tmrb5lmt: {=u8:?}, tmrb5trig: {:?}, tmrb5nosync: {:?}, tmrb5tinv: {=bool:?}, tmrb5pol23: {:?}, tmrb5en23: {:?} }}" , self . tmra5lmt () , self . tmra5trig () , self . tmra5nosync () , self . tmra5tinv () , self . tmra5pol23 () , self . tmra5en23 () , self . tmrb5lmt () , self . tmrb5trig () , self . tmrb5nosync () , self . tmrb5tinv () , self . tmrb5pol23 () , self . tmrb5en23 ())
    }
}
#[doc = "Counter/Timer Auxiliary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux6(pub u32);
impl Aux6 {
    #[doc = "Counter/Timer A6 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6lmt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Counter/Timer A6 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmra6lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Counter/Timer A6 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6trig(&self) -> super::vals::Tmra6trig {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::Tmra6trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A6 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmra6trig(&mut self, val: super::vals::Tmra6trig) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmra6nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A6 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6tinv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmra6tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer A6 Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A6 Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmra6pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A6 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A6 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmra6en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B6 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6lmt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Counter/Timer B6 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmrb6lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Counter/Timer B6 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6trig(&self) -> super::vals::Tmrb6trig {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Tmrb6trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B6 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmrb6trig(&mut self, val: super::vals::Tmrb6trig) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmrb6nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B6 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6tinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmrb6tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmrb6pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer B6 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B6 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmrb6en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Aux6 {
    #[inline(always)]
    fn default() -> Aux6 {
        Aux6(0)
    }
}
impl core::fmt::Debug for Aux6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux6")
            .field("tmra6lmt", &self.tmra6lmt())
            .field("tmra6trig", &self.tmra6trig())
            .field("tmra6nosync", &self.tmra6nosync())
            .field("tmra6tinv", &self.tmra6tinv())
            .field("tmra6pol23", &self.tmra6pol23())
            .field("tmra6en23", &self.tmra6en23())
            .field("tmrb6lmt", &self.tmrb6lmt())
            .field("tmrb6trig", &self.tmrb6trig())
            .field("tmrb6nosync", &self.tmrb6nosync())
            .field("tmrb6tinv", &self.tmrb6tinv())
            .field("tmrb6pol23", &self.tmrb6pol23())
            .field("tmrb6en23", &self.tmrb6en23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux6 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aux6 {{ tmra6lmt: {=u8:?}, tmra6trig: {:?}, tmra6nosync: {:?}, tmra6tinv: {=bool:?}, tmra6pol23: {:?}, tmra6en23: {:?}, tmrb6lmt: {=u8:?}, tmrb6trig: {:?}, tmrb6nosync: {:?}, tmrb6tinv: {=bool:?}, tmrb6pol23: {:?}, tmrb6en23: {:?} }}" , self . tmra6lmt () , self . tmra6trig () , self . tmra6nosync () , self . tmra6tinv () , self . tmra6pol23 () , self . tmra6en23 () , self . tmrb6lmt () , self . tmrb6trig () , self . tmrb6nosync () , self . tmrb6tinv () , self . tmrb6pol23 () , self . tmrb6en23 ())
    }
}
#[doc = "Counter/Timer Auxiliary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux7(pub u32);
impl Aux7 {
    #[doc = "Counter/Timer A7 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7lmt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Counter/Timer A7 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmra7lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Counter/Timer A7 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7trig(&self) -> super::vals::Tmra7trig {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::Tmra7trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A7 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmra7trig(&mut self, val: super::vals::Tmra7trig) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmra7nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A7 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7tinv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmra7tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer A7 Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A7 Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmra7pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A7 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A7 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmra7en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B7 Pattern Limit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7lmt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Counter/Timer B7 Pattern Limit Count."]
    #[inline(always)]
    pub const fn set_tmrb7lmt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Counter/Timer B7 Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7trig(&self) -> super::vals::Tmrb7trig {
        let val = (self.0 >> 23usize) & 0x0f;
        super::vals::Tmrb7trig::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B7 Trigger Select."]
    #[inline(always)]
    pub const fn set_tmrb7trig(&mut self, val: super::vals::Tmrb7trig) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val.to_bits() as u32) & 0x0f) << 23usize);
    }
    #[doc = "Source clock synchronization control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7nosync(&self) -> super::vals::Tmrnosync {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Tmrnosync::from_bits(val as u8)
    }
    #[doc = "Source clock synchronization control."]
    #[inline(always)]
    pub const fn set_tmrb7nosync(&mut self, val: super::vals::Tmrnosync) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B7 Invert on trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7tinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 Invert on trigger."]
    #[inline(always)]
    pub const fn set_tmrb7tinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Upper output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7pol23(&self) -> super::vals::Tmrpol23 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Tmrpol23::from_bits(val as u8)
    }
    #[doc = "Upper output polarity."]
    #[inline(always)]
    pub const fn set_tmrb7pol23(&mut self, val: super::vals::Tmrpol23) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer B7 Upper compare enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7en23(&self) -> super::vals::Tmren23 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tmren23::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B7 Upper compare enable."]
    #[inline(always)]
    pub const fn set_tmrb7en23(&mut self, val: super::vals::Tmren23) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Aux7 {
    #[inline(always)]
    fn default() -> Aux7 {
        Aux7(0)
    }
}
impl core::fmt::Debug for Aux7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux7")
            .field("tmra7lmt", &self.tmra7lmt())
            .field("tmra7trig", &self.tmra7trig())
            .field("tmra7nosync", &self.tmra7nosync())
            .field("tmra7tinv", &self.tmra7tinv())
            .field("tmra7pol23", &self.tmra7pol23())
            .field("tmra7en23", &self.tmra7en23())
            .field("tmrb7lmt", &self.tmrb7lmt())
            .field("tmrb7trig", &self.tmrb7trig())
            .field("tmrb7nosync", &self.tmrb7nosync())
            .field("tmrb7tinv", &self.tmrb7tinv())
            .field("tmrb7pol23", &self.tmrb7pol23())
            .field("tmrb7en23", &self.tmrb7en23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux7 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Aux7 {{ tmra7lmt: {=u8:?}, tmra7trig: {:?}, tmra7nosync: {:?}, tmra7tinv: {=bool:?}, tmra7pol23: {:?}, tmra7en23: {:?}, tmrb7lmt: {=u8:?}, tmrb7trig: {:?}, tmrb7nosync: {:?}, tmrb7tinv: {=bool:?}, tmrb7pol23: {:?}, tmrb7en23: {:?} }}" , self . tmra7lmt () , self . tmra7trig () , self . tmra7nosync () , self . tmra7tinv () , self . tmra7pol23 () , self . tmra7en23 () , self . tmrb7lmt () , self . tmrb7trig () , self . tmrb7nosync () , self . tmrb7tinv () , self . tmrb7pol23 () , self . tmrb7en23 ())
    }
}
#[doc = "Capture Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capturecontrol(pub u32);
impl Capturecontrol {
    #[doc = "Selects whether capture is enabled for the specified capture register."]
    #[must_use]
    #[inline(always)]
    pub const fn capture0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub const fn set_capture0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects whether capture is enabled for the specified capture register."]
    #[must_use]
    #[inline(always)]
    pub const fn capture1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub const fn set_capture1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Selects whether capture is enabled for the specified capture register."]
    #[must_use]
    #[inline(always)]
    pub const fn capture2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub const fn set_capture2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Selects whether capture is enabled for the specified capture register."]
    #[must_use]
    #[inline(always)]
    pub const fn capture3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub const fn set_capture3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Capturecontrol {
    #[inline(always)]
    fn default() -> Capturecontrol {
        Capturecontrol(0)
    }
}
impl core::fmt::Debug for Capturecontrol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capturecontrol")
            .field("capture0", &self.capture0())
            .field("capture1", &self.capture1())
            .field("capture2", &self.capture2())
            .field("capture3", &self.capture3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capturecontrol {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Capturecontrol {{ capture0: {=bool:?}, capture1: {=bool:?}, capture2: {=bool:?}, capture3: {=bool:?} }}" , self . capture0 () , self . capture1 () , self . capture2 () , self . capture3 ())
    }
}
#[doc = "Counter/Timer A0 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpra0(pub u32);
impl Cmpra0 {
    #[doc = "Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0a0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr0a0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1a0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr1a0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmpra0 {
    #[inline(always)]
    fn default() -> Cmpra0 {
        Cmpra0(0)
    }
}
impl core::fmt::Debug for Cmpra0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpra0")
            .field("cmpr0a0", &self.cmpr0a0())
            .field("cmpr1a0", &self.cmpr1a0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpra0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpra0 {{ cmpr0a0: {=u16:?}, cmpr1a0: {=u16:?} }}",
            self.cmpr0a0(),
            self.cmpr1a0()
        )
    }
}
#[doc = "Counter/Timer A1 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpra1(pub u32);
impl Cmpra1 {
    #[doc = "Counter/Timer A1 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0a1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A1 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0a1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A1 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1a1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A1 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1a1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmpra1 {
    #[inline(always)]
    fn default() -> Cmpra1 {
        Cmpra1(0)
    }
}
impl core::fmt::Debug for Cmpra1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpra1")
            .field("cmpr0a1", &self.cmpr0a1())
            .field("cmpr1a1", &self.cmpr1a1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpra1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpra1 {{ cmpr0a1: {=u16:?}, cmpr1a1: {=u16:?} }}",
            self.cmpr0a1(),
            self.cmpr1a1()
        )
    }
}
#[doc = "Counter/Timer A2 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpra2(pub u32);
impl Cmpra2 {
    #[doc = "Counter/Timer A2 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0a2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A2 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0a2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A2 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1a2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A2 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1a2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmpra2 {
    #[inline(always)]
    fn default() -> Cmpra2 {
        Cmpra2(0)
    }
}
impl core::fmt::Debug for Cmpra2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpra2")
            .field("cmpr0a2", &self.cmpr0a2())
            .field("cmpr1a2", &self.cmpr1a2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpra2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpra2 {{ cmpr0a2: {=u16:?}, cmpr1a2: {=u16:?} }}",
            self.cmpr0a2(),
            self.cmpr1a2()
        )
    }
}
#[doc = "Counter/Timer A3 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpra3(pub u32);
impl Cmpra3 {
    #[doc = "Counter/Timer A3 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0a3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A3 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0a3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A3 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1a3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A3 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1a3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmpra3 {
    #[inline(always)]
    fn default() -> Cmpra3 {
        Cmpra3(0)
    }
}
impl core::fmt::Debug for Cmpra3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpra3")
            .field("cmpr0a3", &self.cmpr0a3())
            .field("cmpr1a3", &self.cmpr1a3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpra3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpra3 {{ cmpr0a3: {=u16:?}, cmpr1a3: {=u16:?} }}",
            self.cmpr0a3(),
            self.cmpr1a3()
        )
    }
}
#[doc = "Counter/Timer A4 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpra4(pub u32);
impl Cmpra4 {
    #[doc = "Counter/Timer A4 Compare Register 0. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0a4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A4 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr0a4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A4 Compare Register 1. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1a4(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A4 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr1a4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmpra4 {
    #[inline(always)]
    fn default() -> Cmpra4 {
        Cmpra4(0)
    }
}
impl core::fmt::Debug for Cmpra4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpra4")
            .field("cmpr0a4", &self.cmpr0a4())
            .field("cmpr1a4", &self.cmpr1a4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpra4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpra4 {{ cmpr0a4: {=u16:?}, cmpr1a4: {=u16:?} }}",
            self.cmpr0a4(),
            self.cmpr1a4()
        )
    }
}
#[doc = "Counter/Timer A5 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpra5(pub u32);
impl Cmpra5 {
    #[doc = "Counter/Timer A5 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0a5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A5 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0a5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A5 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1a5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A5 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1a5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmpra5 {
    #[inline(always)]
    fn default() -> Cmpra5 {
        Cmpra5(0)
    }
}
impl core::fmt::Debug for Cmpra5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpra5")
            .field("cmpr0a5", &self.cmpr0a5())
            .field("cmpr1a5", &self.cmpr1a5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpra5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpra5 {{ cmpr0a5: {=u16:?}, cmpr1a5: {=u16:?} }}",
            self.cmpr0a5(),
            self.cmpr1a5()
        )
    }
}
#[doc = "Counter/Timer A6 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpra6(pub u32);
impl Cmpra6 {
    #[doc = "Counter/Timer A6 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0a6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A6 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0a6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A6 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1a6(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A6 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1a6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmpra6 {
    #[inline(always)]
    fn default() -> Cmpra6 {
        Cmpra6(0)
    }
}
impl core::fmt::Debug for Cmpra6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpra6")
            .field("cmpr0a6", &self.cmpr0a6())
            .field("cmpr1a6", &self.cmpr1a6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpra6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpra6 {{ cmpr0a6: {=u16:?}, cmpr1a6: {=u16:?} }}",
            self.cmpr0a6(),
            self.cmpr1a6()
        )
    }
}
#[doc = "Counter/Timer A7 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpra7(pub u32);
impl Cmpra7 {
    #[doc = "Counter/Timer A7 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0a7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A7 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0a7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A7 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1a7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A7 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1a7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmpra7 {
    #[inline(always)]
    fn default() -> Cmpra7 {
        Cmpra7(0)
    }
}
impl core::fmt::Debug for Cmpra7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpra7")
            .field("cmpr0a7", &self.cmpr0a7())
            .field("cmpr1a7", &self.cmpr1a7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpra7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpra7 {{ cmpr0a7: {=u16:?}, cmpr1a7: {=u16:?} }}",
            self.cmpr0a7(),
            self.cmpr1a7()
        )
    }
}
#[doc = "Counter/Timer A0 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxa0(pub u32);
impl Cmprauxa0 {
    #[doc = "Counter/Timer A0 Compare Register 2. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2a0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A0 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr2a0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A0 Compare Register 3. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3a0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A0 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr3a0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxa0 {
    #[inline(always)]
    fn default() -> Cmprauxa0 {
        Cmprauxa0(0)
    }
}
impl core::fmt::Debug for Cmprauxa0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxa0")
            .field("cmpr2a0", &self.cmpr2a0())
            .field("cmpr3a0", &self.cmpr3a0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxa0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxa0 {{ cmpr2a0: {=u16:?}, cmpr3a0: {=u16:?} }}",
            self.cmpr2a0(),
            self.cmpr3a0()
        )
    }
}
#[doc = "Counter/Timer A1 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxa1(pub u32);
impl Cmprauxa1 {
    #[doc = "Counter/Timer A1 Compare Register 2. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2a1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A1 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr2a1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A1 Compare Register 3. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3a1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A1 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr3a1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxa1 {
    #[inline(always)]
    fn default() -> Cmprauxa1 {
        Cmprauxa1(0)
    }
}
impl core::fmt::Debug for Cmprauxa1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxa1")
            .field("cmpr2a1", &self.cmpr2a1())
            .field("cmpr3a1", &self.cmpr3a1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxa1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxa1 {{ cmpr2a1: {=u16:?}, cmpr3a1: {=u16:?} }}",
            self.cmpr2a1(),
            self.cmpr3a1()
        )
    }
}
#[doc = "Counter/Timer A2 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxa2(pub u32);
impl Cmprauxa2 {
    #[doc = "Counter/Timer A2 Compare Register 2. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2a2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A2 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr2a2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A2 Compare Register 3. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3a2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A2 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr3a2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxa2 {
    #[inline(always)]
    fn default() -> Cmprauxa2 {
        Cmprauxa2(0)
    }
}
impl core::fmt::Debug for Cmprauxa2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxa2")
            .field("cmpr2a2", &self.cmpr2a2())
            .field("cmpr3a2", &self.cmpr3a2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxa2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxa2 {{ cmpr2a2: {=u16:?}, cmpr3a2: {=u16:?} }}",
            self.cmpr2a2(),
            self.cmpr3a2()
        )
    }
}
#[doc = "Counter/Timer A3 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxa3(pub u32);
impl Cmprauxa3 {
    #[doc = "Counter/Timer A3 Compare Register 2. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2a3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A3 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr2a3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A3 Compare Register 3. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3a3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A3 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr3a3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxa3 {
    #[inline(always)]
    fn default() -> Cmprauxa3 {
        Cmprauxa3(0)
    }
}
impl core::fmt::Debug for Cmprauxa3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxa3")
            .field("cmpr2a3", &self.cmpr2a3())
            .field("cmpr3a3", &self.cmpr3a3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxa3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxa3 {{ cmpr2a3: {=u16:?}, cmpr3a3: {=u16:?} }}",
            self.cmpr2a3(),
            self.cmpr3a3()
        )
    }
}
#[doc = "Counter/Timer A4 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxa4(pub u32);
impl Cmprauxa4 {
    #[doc = "Counter/Timer A4 Compare Register 2. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2a4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A4 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr2a4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A4 Compare Register 3. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3a4(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A4 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr3a4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxa4 {
    #[inline(always)]
    fn default() -> Cmprauxa4 {
        Cmprauxa4(0)
    }
}
impl core::fmt::Debug for Cmprauxa4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxa4")
            .field("cmpr2a4", &self.cmpr2a4())
            .field("cmpr3a4", &self.cmpr3a4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxa4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxa4 {{ cmpr2a4: {=u16:?}, cmpr3a4: {=u16:?} }}",
            self.cmpr2a4(),
            self.cmpr3a4()
        )
    }
}
#[doc = "Counter/Timer A5 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxa5(pub u32);
impl Cmprauxa5 {
    #[doc = "Counter/Timer A5 Compare Register 2. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2a5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A5 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr2a5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A5 Compare Register 3. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3a5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A5 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr3a5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxa5 {
    #[inline(always)]
    fn default() -> Cmprauxa5 {
        Cmprauxa5(0)
    }
}
impl core::fmt::Debug for Cmprauxa5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxa5")
            .field("cmpr2a5", &self.cmpr2a5())
            .field("cmpr3a5", &self.cmpr3a5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxa5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxa5 {{ cmpr2a5: {=u16:?}, cmpr3a5: {=u16:?} }}",
            self.cmpr2a5(),
            self.cmpr3a5()
        )
    }
}
#[doc = "Counter/Timer A6 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxa6(pub u32);
impl Cmprauxa6 {
    #[doc = "Counter/Timer A6 Compare Register 2. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2a6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A6 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr2a6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A6 Compare Register 3. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3a6(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A6 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr3a6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxa6 {
    #[inline(always)]
    fn default() -> Cmprauxa6 {
        Cmprauxa6(0)
    }
}
impl core::fmt::Debug for Cmprauxa6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxa6")
            .field("cmpr2a6", &self.cmpr2a6())
            .field("cmpr3a6", &self.cmpr3a6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxa6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxa6 {{ cmpr2a6: {=u16:?}, cmpr3a6: {=u16:?} }}",
            self.cmpr2a6(),
            self.cmpr3a6()
        )
    }
}
#[doc = "Counter/Timer A7 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxa7(pub u32);
impl Cmprauxa7 {
    #[doc = "Counter/Timer A7 Compare Register 2. Holds the lower limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2a7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A7 Compare Register 2. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr2a7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer A7 Compare Register 3. Holds the upper limit for timer half A."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3a7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A7 Compare Register 3. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub const fn set_cmpr3a7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxa7 {
    #[inline(always)]
    fn default() -> Cmprauxa7 {
        Cmprauxa7(0)
    }
}
impl core::fmt::Debug for Cmprauxa7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxa7")
            .field("cmpr2a7", &self.cmpr2a7())
            .field("cmpr3a7", &self.cmpr3a7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxa7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxa7 {{ cmpr2a7: {=u16:?}, cmpr3a7: {=u16:?} }}",
            self.cmpr2a7(),
            self.cmpr3a7()
        )
    }
}
#[doc = "Counter/Timer B0 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxb0(pub u32);
impl Cmprauxb0 {
    #[doc = "Counter/Timer B0 Compare Register 2. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2b0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B0 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr2b0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B0 Compare Register 3. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3b0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B0 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr3b0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxb0 {
    #[inline(always)]
    fn default() -> Cmprauxb0 {
        Cmprauxb0(0)
    }
}
impl core::fmt::Debug for Cmprauxb0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxb0")
            .field("cmpr2b0", &self.cmpr2b0())
            .field("cmpr3b0", &self.cmpr3b0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxb0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxb0 {{ cmpr2b0: {=u16:?}, cmpr3b0: {=u16:?} }}",
            self.cmpr2b0(),
            self.cmpr3b0()
        )
    }
}
#[doc = "Counter/Timer B1 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxb1(pub u32);
impl Cmprauxb1 {
    #[doc = "Counter/Timer B1 Compare Register 2. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2b1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B1 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr2b1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B1 Compare Register 3. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3b1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B1 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr3b1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxb1 {
    #[inline(always)]
    fn default() -> Cmprauxb1 {
        Cmprauxb1(0)
    }
}
impl core::fmt::Debug for Cmprauxb1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxb1")
            .field("cmpr2b1", &self.cmpr2b1())
            .field("cmpr3b1", &self.cmpr3b1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxb1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxb1 {{ cmpr2b1: {=u16:?}, cmpr3b1: {=u16:?} }}",
            self.cmpr2b1(),
            self.cmpr3b1()
        )
    }
}
#[doc = "Counter/Timer B2 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxb2(pub u32);
impl Cmprauxb2 {
    #[doc = "Counter/Timer B2 Compare Register 2. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2b2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B2 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr2b2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B2 Compare Register 3. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3b2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B2 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr3b2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxb2 {
    #[inline(always)]
    fn default() -> Cmprauxb2 {
        Cmprauxb2(0)
    }
}
impl core::fmt::Debug for Cmprauxb2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxb2")
            .field("cmpr2b2", &self.cmpr2b2())
            .field("cmpr3b2", &self.cmpr3b2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxb2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxb2 {{ cmpr2b2: {=u16:?}, cmpr3b2: {=u16:?} }}",
            self.cmpr2b2(),
            self.cmpr3b2()
        )
    }
}
#[doc = "Counter/Timer B3 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxb3(pub u32);
impl Cmprauxb3 {
    #[doc = "Counter/Timer B3 Compare Register 2. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2b3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B3 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr2b3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B3 Compare Register 3. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3b3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B3 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr3b3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxb3 {
    #[inline(always)]
    fn default() -> Cmprauxb3 {
        Cmprauxb3(0)
    }
}
impl core::fmt::Debug for Cmprauxb3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxb3")
            .field("cmpr2b3", &self.cmpr2b3())
            .field("cmpr3b3", &self.cmpr3b3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxb3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxb3 {{ cmpr2b3: {=u16:?}, cmpr3b3: {=u16:?} }}",
            self.cmpr2b3(),
            self.cmpr3b3()
        )
    }
}
#[doc = "Counter/Timer B4 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxb4(pub u32);
impl Cmprauxb4 {
    #[doc = "Counter/Timer B4 Compare Register 2. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2b4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B4 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr2b4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B4 Compare Register 3. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3b4(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B4 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr3b4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxb4 {
    #[inline(always)]
    fn default() -> Cmprauxb4 {
        Cmprauxb4(0)
    }
}
impl core::fmt::Debug for Cmprauxb4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxb4")
            .field("cmpr2b4", &self.cmpr2b4())
            .field("cmpr3b4", &self.cmpr3b4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxb4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxb4 {{ cmpr2b4: {=u16:?}, cmpr3b4: {=u16:?} }}",
            self.cmpr2b4(),
            self.cmpr3b4()
        )
    }
}
#[doc = "Counter/Timer B5 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxb5(pub u32);
impl Cmprauxb5 {
    #[doc = "Counter/Timer B5 Compare Register 2. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2b5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B5 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr2b5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B5 Compare Register 3. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3b5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B5 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr3b5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxb5 {
    #[inline(always)]
    fn default() -> Cmprauxb5 {
        Cmprauxb5(0)
    }
}
impl core::fmt::Debug for Cmprauxb5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxb5")
            .field("cmpr2b5", &self.cmpr2b5())
            .field("cmpr3b5", &self.cmpr3b5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxb5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxb5 {{ cmpr2b5: {=u16:?}, cmpr3b5: {=u16:?} }}",
            self.cmpr2b5(),
            self.cmpr3b5()
        )
    }
}
#[doc = "Counter/Timer B6 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxb6(pub u32);
impl Cmprauxb6 {
    #[doc = "Counter/Timer B6 Compare Register 2. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2b6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B6 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr2b6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B6 Compare Register 3. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3b6(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B6 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr3b6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxb6 {
    #[inline(always)]
    fn default() -> Cmprauxb6 {
        Cmprauxb6(0)
    }
}
impl core::fmt::Debug for Cmprauxb6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxb6")
            .field("cmpr2b6", &self.cmpr2b6())
            .field("cmpr3b6", &self.cmpr3b6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxb6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxb6 {{ cmpr2b6: {=u16:?}, cmpr3b6: {=u16:?} }}",
            self.cmpr2b6(),
            self.cmpr3b6()
        )
    }
}
#[doc = "Counter/Timer B7 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprauxb7(pub u32);
impl Cmprauxb7 {
    #[doc = "Counter/Timer B7 Compare Register 2. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr2b7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B7 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr2b7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B7 Compare Register 3. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr3b7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B7 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr3b7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprauxb7 {
    #[inline(always)]
    fn default() -> Cmprauxb7 {
        Cmprauxb7(0)
    }
}
impl core::fmt::Debug for Cmprauxb7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprauxb7")
            .field("cmpr2b7", &self.cmpr2b7())
            .field("cmpr3b7", &self.cmpr3b7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprauxb7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprauxb7 {{ cmpr2b7: {=u16:?}, cmpr3b7: {=u16:?} }}",
            self.cmpr2b7(),
            self.cmpr3b7()
        )
    }
}
#[doc = "Counter/Timer B0 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprb0(pub u32);
impl Cmprb0 {
    #[doc = "Counter/Timer B0 Compare Register 0. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0b0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B0 Compare Register 0. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr0b0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B0 Compare Register 1. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1b0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B0 Compare Register 1. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr1b0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprb0 {
    #[inline(always)]
    fn default() -> Cmprb0 {
        Cmprb0(0)
    }
}
impl core::fmt::Debug for Cmprb0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprb0")
            .field("cmpr0b0", &self.cmpr0b0())
            .field("cmpr1b0", &self.cmpr1b0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprb0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprb0 {{ cmpr0b0: {=u16:?}, cmpr1b0: {=u16:?} }}",
            self.cmpr0b0(),
            self.cmpr1b0()
        )
    }
}
#[doc = "Counter/Timer B1 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprb1(pub u32);
impl Cmprb1 {
    #[doc = "Counter/Timer B1 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0b1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B1 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0b1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B1 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1b1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B1 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1b1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprb1 {
    #[inline(always)]
    fn default() -> Cmprb1 {
        Cmprb1(0)
    }
}
impl core::fmt::Debug for Cmprb1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprb1")
            .field("cmpr0b1", &self.cmpr0b1())
            .field("cmpr1b1", &self.cmpr1b1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprb1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprb1 {{ cmpr0b1: {=u16:?}, cmpr1b1: {=u16:?} }}",
            self.cmpr0b1(),
            self.cmpr1b1()
        )
    }
}
#[doc = "Counter/Timer B2 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprb2(pub u32);
impl Cmprb2 {
    #[doc = "Counter/Timer B2 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0b2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B2 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0b2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B2 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1b2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B2 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1b2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprb2 {
    #[inline(always)]
    fn default() -> Cmprb2 {
        Cmprb2(0)
    }
}
impl core::fmt::Debug for Cmprb2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprb2")
            .field("cmpr0b2", &self.cmpr0b2())
            .field("cmpr1b2", &self.cmpr1b2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprb2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprb2 {{ cmpr0b2: {=u16:?}, cmpr1b2: {=u16:?} }}",
            self.cmpr0b2(),
            self.cmpr1b2()
        )
    }
}
#[doc = "Counter/Timer B3 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprb3(pub u32);
impl Cmprb3 {
    #[doc = "Counter/Timer B3 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0b3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B3 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0b3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B3 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1b3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B3 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1b3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprb3 {
    #[inline(always)]
    fn default() -> Cmprb3 {
        Cmprb3(0)
    }
}
impl core::fmt::Debug for Cmprb3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprb3")
            .field("cmpr0b3", &self.cmpr0b3())
            .field("cmpr1b3", &self.cmpr1b3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprb3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprb3 {{ cmpr0b3: {=u16:?}, cmpr1b3: {=u16:?} }}",
            self.cmpr0b3(),
            self.cmpr1b3()
        )
    }
}
#[doc = "Counter/Timer B4 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprb4(pub u32);
impl Cmprb4 {
    #[doc = "Counter/Timer B4 Compare Register 0. Holds the lower limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0b4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B4 Compare Register 0. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr0b4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B4 Compare Register 1. Holds the upper limit for timer half B."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1b4(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B4 Compare Register 1. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub const fn set_cmpr1b4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprb4 {
    #[inline(always)]
    fn default() -> Cmprb4 {
        Cmprb4(0)
    }
}
impl core::fmt::Debug for Cmprb4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprb4")
            .field("cmpr0b4", &self.cmpr0b4())
            .field("cmpr1b4", &self.cmpr1b4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprb4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprb4 {{ cmpr0b4: {=u16:?}, cmpr1b4: {=u16:?} }}",
            self.cmpr0b4(),
            self.cmpr1b4()
        )
    }
}
#[doc = "Counter/Timer B5 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprb5(pub u32);
impl Cmprb5 {
    #[doc = "Counter/Timer B5 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0b5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B5 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0b5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B5 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1b5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B5 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1b5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprb5 {
    #[inline(always)]
    fn default() -> Cmprb5 {
        Cmprb5(0)
    }
}
impl core::fmt::Debug for Cmprb5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprb5")
            .field("cmpr0b5", &self.cmpr0b5())
            .field("cmpr1b5", &self.cmpr1b5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprb5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprb5 {{ cmpr0b5: {=u16:?}, cmpr1b5: {=u16:?} }}",
            self.cmpr0b5(),
            self.cmpr1b5()
        )
    }
}
#[doc = "Counter/Timer B6 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprb6(pub u32);
impl Cmprb6 {
    #[doc = "Counter/Timer B6 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0b6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B6 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0b6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B6 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1b6(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B6 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1b6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprb6 {
    #[inline(always)]
    fn default() -> Cmprb6 {
        Cmprb6(0)
    }
}
impl core::fmt::Debug for Cmprb6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprb6")
            .field("cmpr0b6", &self.cmpr0b6())
            .field("cmpr1b6", &self.cmpr1b6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprb6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprb6 {{ cmpr0b6: {=u16:?}, cmpr1b6: {=u16:?} }}",
            self.cmpr0b6(),
            self.cmpr1b6()
        )
    }
}
#[doc = "Counter/Timer B7 Compare Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprb7(pub u32);
impl Cmprb7 {
    #[doc = "Counter/Timer B3 Compare Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr0b7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B3 Compare Register 0."]
    #[inline(always)]
    pub const fn set_cmpr0b7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B3 Compare Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpr1b7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B3 Compare Register 1."]
    #[inline(always)]
    pub const fn set_cmpr1b7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cmprb7 {
    #[inline(always)]
    fn default() -> Cmprb7 {
        Cmprb7(0)
    }
}
impl core::fmt::Debug for Cmprb7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprb7")
            .field("cmpr0b7", &self.cmpr0b7())
            .field("cmpr1b7", &self.cmpr1b7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprb7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprb7 {{ cmpr0b7: {=u16:?}, cmpr1b7: {=u16:?} }}",
            self.cmpr0b7(),
            self.cmpr1b7()
        )
    }
}
#[doc = "Counter/Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc = "Counter/Timer A0 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 Enable bit."]
    #[inline(always)]
    pub const fn set_tmra0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer A0 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0clk(&self) -> super::vals::Tmra0clk {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tmra0clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A0 Clock Select."]
    #[inline(always)]
    pub const fn set_tmra0clk(&mut self, val: super::vals::Tmra0clk) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Counter/Timer A0 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A0 Function Select."]
    #[inline(always)]
    pub const fn set_tmra0fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0ie0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub const fn set_tmra0ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0ie1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub const fn set_tmra0ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer A0 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A0 Clear bit."]
    #[inline(always)]
    pub const fn set_tmra0clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A0 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra0pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A0 output polarity."]
    #[inline(always)]
    pub const fn set_tmra0pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B0 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 Enable bit."]
    #[inline(always)]
    pub const fn set_tmrb0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B0 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0clk(&self) -> super::vals::Tmrb0clk {
        let val = (self.0 >> 17usize) & 0x1f;
        super::vals::Tmrb0clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B0 Clock Select."]
    #[inline(always)]
    pub const fn set_tmrb0clk(&mut self, val: super::vals::Tmrb0clk) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val.to_bits() as u32) & 0x1f) << 17usize);
    }
    #[doc = "Counter/Timer B0 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B0 Function Select."]
    #[inline(always)]
    pub const fn set_tmrb0fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Counter/Timer B0 Interrupt Enable bit for COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0ie0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub const fn set_tmrb0ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer B0 Interrupt Enable bit for COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0ie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub const fn set_tmrb0ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B0 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B0 Clear bit."]
    #[inline(always)]
    pub const fn set_tmrb0clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B0 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb0pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B0 output polarity."]
    #[inline(always)]
    pub const fn set_tmrb0pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer A0/B0 Link bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ctlink0(&self) -> super::vals::Ctlink {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ctlink::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A0/B0 Link bit."]
    #[inline(always)]
    pub const fn set_ctlink0(&mut self, val: super::vals::Ctlink) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("tmra0en", &self.tmra0en())
            .field("tmra0clk", &self.tmra0clk())
            .field("tmra0fn", &self.tmra0fn())
            .field("tmra0ie0", &self.tmra0ie0())
            .field("tmra0ie1", &self.tmra0ie1())
            .field("tmra0clr", &self.tmra0clr())
            .field("tmra0pol", &self.tmra0pol())
            .field("tmrb0en", &self.tmrb0en())
            .field("tmrb0clk", &self.tmrb0clk())
            .field("tmrb0fn", &self.tmrb0fn())
            .field("tmrb0ie0", &self.tmrb0ie0())
            .field("tmrb0ie1", &self.tmrb0ie1())
            .field("tmrb0clr", &self.tmrb0clr())
            .field("tmrb0pol", &self.tmrb0pol())
            .field("ctlink0", &self.ctlink0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl0 {{ tmra0en: {=bool:?}, tmra0clk: {:?}, tmra0fn: {:?}, tmra0ie0: {=bool:?}, tmra0ie1: {=bool:?}, tmra0clr: {:?}, tmra0pol: {:?}, tmrb0en: {=bool:?}, tmrb0clk: {:?}, tmrb0fn: {:?}, tmrb0ie0: {=bool:?}, tmrb0ie1: {=bool:?}, tmrb0clr: {:?}, tmrb0pol: {:?}, ctlink0: {:?} }}" , self . tmra0en () , self . tmra0clk () , self . tmra0fn () , self . tmra0ie0 () , self . tmra0ie1 () , self . tmra0clr () , self . tmra0pol () , self . tmrb0en () , self . tmrb0clk () , self . tmrb0fn () , self . tmrb0ie0 () , self . tmrb0ie1 () , self . tmrb0clr () , self . tmrb0pol () , self . ctlink0 ())
    }
}
#[doc = "Counter/Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Counter/Timer A1 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 Enable bit."]
    #[inline(always)]
    pub const fn set_tmra1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer A1 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1clk(&self) -> super::vals::Tmra1clk {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tmra1clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A1 Clock Select."]
    #[inline(always)]
    pub const fn set_tmra1clk(&mut self, val: super::vals::Tmra1clk) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Counter/Timer A1 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A1 Function Select."]
    #[inline(always)]
    pub const fn set_tmra1fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Counter/Timer A1 Interrupt Enable bit based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1ie0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub const fn set_tmra1ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A1 Interrupt Enable bit based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1ie1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub const fn set_tmra1ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer A1 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A1 Clear bit."]
    #[inline(always)]
    pub const fn set_tmra1clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A1 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra1pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A1 output polarity."]
    #[inline(always)]
    pub const fn set_tmra1pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B1 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 Enable bit."]
    #[inline(always)]
    pub const fn set_tmrb1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B1 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1clk(&self) -> super::vals::Tmrb1clk {
        let val = (self.0 >> 17usize) & 0x1f;
        super::vals::Tmrb1clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B1 Clock Select."]
    #[inline(always)]
    pub const fn set_tmrb1clk(&mut self, val: super::vals::Tmrb1clk) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val.to_bits() as u32) & 0x1f) << 17usize);
    }
    #[doc = "Counter/Timer B1 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B1 Function Select."]
    #[inline(always)]
    pub const fn set_tmrb1fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Counter/Timer B1 Interrupt Enable bit for COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1ie0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub const fn set_tmrb1ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer B1 Interrupt Enable bit for COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1ie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub const fn set_tmrb1ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B1 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B1 Clear bit."]
    #[inline(always)]
    pub const fn set_tmrb1clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B1 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb1pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B1 output polarity."]
    #[inline(always)]
    pub const fn set_tmrb1pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer A1/B1 Link bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ctlink1(&self) -> super::vals::Ctlink {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ctlink::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A1/B1 Link bit."]
    #[inline(always)]
    pub const fn set_ctlink1(&mut self, val: super::vals::Ctlink) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("tmra1en", &self.tmra1en())
            .field("tmra1clk", &self.tmra1clk())
            .field("tmra1fn", &self.tmra1fn())
            .field("tmra1ie0", &self.tmra1ie0())
            .field("tmra1ie1", &self.tmra1ie1())
            .field("tmra1clr", &self.tmra1clr())
            .field("tmra1pol", &self.tmra1pol())
            .field("tmrb1en", &self.tmrb1en())
            .field("tmrb1clk", &self.tmrb1clk())
            .field("tmrb1fn", &self.tmrb1fn())
            .field("tmrb1ie0", &self.tmrb1ie0())
            .field("tmrb1ie1", &self.tmrb1ie1())
            .field("tmrb1clr", &self.tmrb1clr())
            .field("tmrb1pol", &self.tmrb1pol())
            .field("ctlink1", &self.ctlink1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl1 {{ tmra1en: {=bool:?}, tmra1clk: {:?}, tmra1fn: {:?}, tmra1ie0: {=bool:?}, tmra1ie1: {=bool:?}, tmra1clr: {:?}, tmra1pol: {:?}, tmrb1en: {=bool:?}, tmrb1clk: {:?}, tmrb1fn: {:?}, tmrb1ie0: {=bool:?}, tmrb1ie1: {=bool:?}, tmrb1clr: {:?}, tmrb1pol: {:?}, ctlink1: {:?} }}" , self . tmra1en () , self . tmra1clk () , self . tmra1fn () , self . tmra1ie0 () , self . tmra1ie1 () , self . tmra1clr () , self . tmra1pol () , self . tmrb1en () , self . tmrb1clk () , self . tmrb1fn () , self . tmrb1ie0 () , self . tmrb1ie1 () , self . tmrb1clr () , self . tmrb1pol () , self . ctlink1 ())
    }
}
#[doc = "Counter/Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "Counter/Timer A2 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 Enable bit."]
    #[inline(always)]
    pub const fn set_tmra2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer A2 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2clk(&self) -> super::vals::Tmra2clk {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tmra2clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A2 Clock Select."]
    #[inline(always)]
    pub const fn set_tmra2clk(&mut self, val: super::vals::Tmra2clk) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Counter/Timer A2 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A2 Function Select."]
    #[inline(always)]
    pub const fn set_tmra2fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2ie0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub const fn set_tmra2ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2ie1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub const fn set_tmra2ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer A2 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A2 Clear bit."]
    #[inline(always)]
    pub const fn set_tmra2clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A2 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra2pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A2 output polarity."]
    #[inline(always)]
    pub const fn set_tmra2pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B2 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 Enable bit."]
    #[inline(always)]
    pub const fn set_tmrb2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B2 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2clk(&self) -> super::vals::Tmrb2clk {
        let val = (self.0 >> 17usize) & 0x1f;
        super::vals::Tmrb2clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B2 Clock Select."]
    #[inline(always)]
    pub const fn set_tmrb2clk(&mut self, val: super::vals::Tmrb2clk) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val.to_bits() as u32) & 0x1f) << 17usize);
    }
    #[doc = "Counter/Timer B2 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B2 Function Select."]
    #[inline(always)]
    pub const fn set_tmrb2fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Counter/Timer B2 Interrupt Enable bit for COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2ie0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub const fn set_tmrb2ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer B2 Interrupt Enable bit for COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2ie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub const fn set_tmrb2ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B2 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B2 Clear bit."]
    #[inline(always)]
    pub const fn set_tmrb2clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B2 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb2pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B2 output polarity."]
    #[inline(always)]
    pub const fn set_tmrb2pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer A2/B2 Link bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ctlink2(&self) -> super::vals::Ctlink {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ctlink::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A2/B2 Link bit."]
    #[inline(always)]
    pub const fn set_ctlink2(&mut self, val: super::vals::Ctlink) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("tmra2en", &self.tmra2en())
            .field("tmra2clk", &self.tmra2clk())
            .field("tmra2fn", &self.tmra2fn())
            .field("tmra2ie0", &self.tmra2ie0())
            .field("tmra2ie1", &self.tmra2ie1())
            .field("tmra2clr", &self.tmra2clr())
            .field("tmra2pol", &self.tmra2pol())
            .field("tmrb2en", &self.tmrb2en())
            .field("tmrb2clk", &self.tmrb2clk())
            .field("tmrb2fn", &self.tmrb2fn())
            .field("tmrb2ie0", &self.tmrb2ie0())
            .field("tmrb2ie1", &self.tmrb2ie1())
            .field("tmrb2clr", &self.tmrb2clr())
            .field("tmrb2pol", &self.tmrb2pol())
            .field("ctlink2", &self.ctlink2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl2 {{ tmra2en: {=bool:?}, tmra2clk: {:?}, tmra2fn: {:?}, tmra2ie0: {=bool:?}, tmra2ie1: {=bool:?}, tmra2clr: {:?}, tmra2pol: {:?}, tmrb2en: {=bool:?}, tmrb2clk: {:?}, tmrb2fn: {:?}, tmrb2ie0: {=bool:?}, tmrb2ie1: {=bool:?}, tmrb2clr: {:?}, tmrb2pol: {:?}, ctlink2: {:?} }}" , self . tmra2en () , self . tmra2clk () , self . tmra2fn () , self . tmra2ie0 () , self . tmra2ie1 () , self . tmra2clr () , self . tmra2pol () , self . tmrb2en () , self . tmrb2clk () , self . tmrb2fn () , self . tmrb2ie0 () , self . tmrb2ie1 () , self . tmrb2clr () , self . tmrb2pol () , self . ctlink2 ())
    }
}
#[doc = "Counter/Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3(pub u32);
impl Ctrl3 {
    #[doc = "Counter/Timer A3 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 Enable bit."]
    #[inline(always)]
    pub const fn set_tmra3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer A3 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3clk(&self) -> super::vals::Tmra3clk {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tmra3clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A3 Clock Select."]
    #[inline(always)]
    pub const fn set_tmra3clk(&mut self, val: super::vals::Tmra3clk) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Counter/Timer A3 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A3 Function Select."]
    #[inline(always)]
    pub const fn set_tmra3fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3ie0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub const fn set_tmra3ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3ie1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub const fn set_tmra3ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer A3 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A3 Clear bit."]
    #[inline(always)]
    pub const fn set_tmra3clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A3 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra3pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A3 output polarity."]
    #[inline(always)]
    pub const fn set_tmra3pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Special Timer A3 enable for ADC function."]
    #[must_use]
    #[inline(always)]
    pub const fn adcen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Special Timer A3 enable for ADC function."]
    #[inline(always)]
    pub const fn set_adcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Counter/Timer B3 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 Enable bit."]
    #[inline(always)]
    pub const fn set_tmrb3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B3 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3clk(&self) -> super::vals::Tmrb3clk {
        let val = (self.0 >> 17usize) & 0x1f;
        super::vals::Tmrb3clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B3 Clock Select."]
    #[inline(always)]
    pub const fn set_tmrb3clk(&mut self, val: super::vals::Tmrb3clk) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val.to_bits() as u32) & 0x1f) << 17usize);
    }
    #[doc = "Counter/Timer B3 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B3 Function Select."]
    #[inline(always)]
    pub const fn set_tmrb3fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Counter/Timer B3 Interrupt Enable bit for COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3ie0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub const fn set_tmrb3ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer B3 Interrupt Enable bit for COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3ie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub const fn set_tmrb3ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B3 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B3 Clear bit."]
    #[inline(always)]
    pub const fn set_tmrb3clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B3 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb3pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B3 output polarity."]
    #[inline(always)]
    pub const fn set_tmrb3pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer A3/B3 Link bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ctlink3(&self) -> super::vals::Ctlink {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ctlink::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A3/B3 Link bit."]
    #[inline(always)]
    pub const fn set_ctlink3(&mut self, val: super::vals::Ctlink) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl3 {
    #[inline(always)]
    fn default() -> Ctrl3 {
        Ctrl3(0)
    }
}
impl core::fmt::Debug for Ctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl3")
            .field("tmra3en", &self.tmra3en())
            .field("tmra3clk", &self.tmra3clk())
            .field("tmra3fn", &self.tmra3fn())
            .field("tmra3ie0", &self.tmra3ie0())
            .field("tmra3ie1", &self.tmra3ie1())
            .field("tmra3clr", &self.tmra3clr())
            .field("tmra3pol", &self.tmra3pol())
            .field("adcen", &self.adcen())
            .field("tmrb3en", &self.tmrb3en())
            .field("tmrb3clk", &self.tmrb3clk())
            .field("tmrb3fn", &self.tmrb3fn())
            .field("tmrb3ie0", &self.tmrb3ie0())
            .field("tmrb3ie1", &self.tmrb3ie1())
            .field("tmrb3clr", &self.tmrb3clr())
            .field("tmrb3pol", &self.tmrb3pol())
            .field("ctlink3", &self.ctlink3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl3 {{ tmra3en: {=bool:?}, tmra3clk: {:?}, tmra3fn: {:?}, tmra3ie0: {=bool:?}, tmra3ie1: {=bool:?}, tmra3clr: {:?}, tmra3pol: {:?}, adcen: {=bool:?}, tmrb3en: {=bool:?}, tmrb3clk: {:?}, tmrb3fn: {:?}, tmrb3ie0: {=bool:?}, tmrb3ie1: {=bool:?}, tmrb3clr: {:?}, tmrb3pol: {:?}, ctlink3: {:?} }}" , self . tmra3en () , self . tmra3clk () , self . tmra3fn () , self . tmra3ie0 () , self . tmra3ie1 () , self . tmra3clr () , self . tmra3pol () , self . adcen () , self . tmrb3en () , self . tmrb3clk () , self . tmrb3fn () , self . tmrb3ie0 () , self . tmrb3ie1 () , self . tmrb3clr () , self . tmrb3pol () , self . ctlink3 ())
    }
}
#[doc = "Counter/Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl4(pub u32);
impl Ctrl4 {
    #[doc = "Counter/Timer A4 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 Enable bit."]
    #[inline(always)]
    pub const fn set_tmra4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer A4 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4clk(&self) -> super::vals::Tmra4clk {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tmra4clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A4 Clock Select."]
    #[inline(always)]
    pub const fn set_tmra4clk(&mut self, val: super::vals::Tmra4clk) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Counter/Timer A4 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A4 Function Select."]
    #[inline(always)]
    pub const fn set_tmra4fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Counter/Timer A4 Interrupt Enable bit based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4ie0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub const fn set_tmra4ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A4 Interrupt Enable bit based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4ie1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub const fn set_tmra4ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer A4 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A4 Clear bit."]
    #[inline(always)]
    pub const fn set_tmra4clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A4 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra4pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A4 output polarity."]
    #[inline(always)]
    pub const fn set_tmra4pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B4 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 Enable bit."]
    #[inline(always)]
    pub const fn set_tmrb4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B4 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4clk(&self) -> super::vals::Tmrb4clk {
        let val = (self.0 >> 17usize) & 0x1f;
        super::vals::Tmrb4clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B4 Clock Select."]
    #[inline(always)]
    pub const fn set_tmrb4clk(&mut self, val: super::vals::Tmrb4clk) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val.to_bits() as u32) & 0x1f) << 17usize);
    }
    #[doc = "Counter/Timer B4 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B4 Function Select."]
    #[inline(always)]
    pub const fn set_tmrb4fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Counter/Timer B4 Interrupt Enable bit for COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4ie0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub const fn set_tmrb4ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer B4 Interrupt Enable bit for COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4ie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub const fn set_tmrb4ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B4 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B4 Clear bit."]
    #[inline(always)]
    pub const fn set_tmrb4clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B4 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb4pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B4 output polarity."]
    #[inline(always)]
    pub const fn set_tmrb4pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer A4/B4 Link bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ctlink4(&self) -> super::vals::Ctlink {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ctlink::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A4/B4 Link bit."]
    #[inline(always)]
    pub const fn set_ctlink4(&mut self, val: super::vals::Ctlink) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl4 {
    #[inline(always)]
    fn default() -> Ctrl4 {
        Ctrl4(0)
    }
}
impl core::fmt::Debug for Ctrl4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl4")
            .field("tmra4en", &self.tmra4en())
            .field("tmra4clk", &self.tmra4clk())
            .field("tmra4fn", &self.tmra4fn())
            .field("tmra4ie0", &self.tmra4ie0())
            .field("tmra4ie1", &self.tmra4ie1())
            .field("tmra4clr", &self.tmra4clr())
            .field("tmra4pol", &self.tmra4pol())
            .field("tmrb4en", &self.tmrb4en())
            .field("tmrb4clk", &self.tmrb4clk())
            .field("tmrb4fn", &self.tmrb4fn())
            .field("tmrb4ie0", &self.tmrb4ie0())
            .field("tmrb4ie1", &self.tmrb4ie1())
            .field("tmrb4clr", &self.tmrb4clr())
            .field("tmrb4pol", &self.tmrb4pol())
            .field("ctlink4", &self.ctlink4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl4 {{ tmra4en: {=bool:?}, tmra4clk: {:?}, tmra4fn: {:?}, tmra4ie0: {=bool:?}, tmra4ie1: {=bool:?}, tmra4clr: {:?}, tmra4pol: {:?}, tmrb4en: {=bool:?}, tmrb4clk: {:?}, tmrb4fn: {:?}, tmrb4ie0: {=bool:?}, tmrb4ie1: {=bool:?}, tmrb4clr: {:?}, tmrb4pol: {:?}, ctlink4: {:?} }}" , self . tmra4en () , self . tmra4clk () , self . tmra4fn () , self . tmra4ie0 () , self . tmra4ie1 () , self . tmra4clr () , self . tmra4pol () , self . tmrb4en () , self . tmrb4clk () , self . tmrb4fn () , self . tmrb4ie0 () , self . tmrb4ie1 () , self . tmrb4clr () , self . tmrb4pol () , self . ctlink4 ())
    }
}
#[doc = "Counter/Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl5(pub u32);
impl Ctrl5 {
    #[doc = "Counter/Timer A5 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 Enable bit."]
    #[inline(always)]
    pub const fn set_tmra5en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer A5 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5clk(&self) -> super::vals::Tmra5clk {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tmra5clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A5 Clock Select."]
    #[inline(always)]
    pub const fn set_tmra5clk(&mut self, val: super::vals::Tmra5clk) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Counter/Timer A5 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A5 Function Select."]
    #[inline(always)]
    pub const fn set_tmra5fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Counter/Timer A5 Interrupt Enable bit based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5ie0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub const fn set_tmra5ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A5 Interrupt Enable bit based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5ie1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub const fn set_tmra5ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer A5 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A5 Clear bit."]
    #[inline(always)]
    pub const fn set_tmra5clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A5 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra5pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A5 output polarity."]
    #[inline(always)]
    pub const fn set_tmra5pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B5 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 Enable bit."]
    #[inline(always)]
    pub const fn set_tmrb5en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B5 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5clk(&self) -> super::vals::Tmrb5clk {
        let val = (self.0 >> 17usize) & 0x1f;
        super::vals::Tmrb5clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B5 Clock Select."]
    #[inline(always)]
    pub const fn set_tmrb5clk(&mut self, val: super::vals::Tmrb5clk) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val.to_bits() as u32) & 0x1f) << 17usize);
    }
    #[doc = "Counter/Timer B5 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B5 Function Select."]
    #[inline(always)]
    pub const fn set_tmrb5fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Counter/Timer B5 Interrupt Enable bit for COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5ie0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub const fn set_tmrb5ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer B5 Interrupt Enable bit for COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5ie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub const fn set_tmrb5ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B5 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B5 Clear bit."]
    #[inline(always)]
    pub const fn set_tmrb5clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B5 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb5pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B5 output polarity."]
    #[inline(always)]
    pub const fn set_tmrb5pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer A5/B5 Link bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ctlink5(&self) -> super::vals::Ctlink {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ctlink::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A5/B5 Link bit."]
    #[inline(always)]
    pub const fn set_ctlink5(&mut self, val: super::vals::Ctlink) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl5 {
    #[inline(always)]
    fn default() -> Ctrl5 {
        Ctrl5(0)
    }
}
impl core::fmt::Debug for Ctrl5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl5")
            .field("tmra5en", &self.tmra5en())
            .field("tmra5clk", &self.tmra5clk())
            .field("tmra5fn", &self.tmra5fn())
            .field("tmra5ie0", &self.tmra5ie0())
            .field("tmra5ie1", &self.tmra5ie1())
            .field("tmra5clr", &self.tmra5clr())
            .field("tmra5pol", &self.tmra5pol())
            .field("tmrb5en", &self.tmrb5en())
            .field("tmrb5clk", &self.tmrb5clk())
            .field("tmrb5fn", &self.tmrb5fn())
            .field("tmrb5ie0", &self.tmrb5ie0())
            .field("tmrb5ie1", &self.tmrb5ie1())
            .field("tmrb5clr", &self.tmrb5clr())
            .field("tmrb5pol", &self.tmrb5pol())
            .field("ctlink5", &self.ctlink5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl5 {{ tmra5en: {=bool:?}, tmra5clk: {:?}, tmra5fn: {:?}, tmra5ie0: {=bool:?}, tmra5ie1: {=bool:?}, tmra5clr: {:?}, tmra5pol: {:?}, tmrb5en: {=bool:?}, tmrb5clk: {:?}, tmrb5fn: {:?}, tmrb5ie0: {=bool:?}, tmrb5ie1: {=bool:?}, tmrb5clr: {:?}, tmrb5pol: {:?}, ctlink5: {:?} }}" , self . tmra5en () , self . tmra5clk () , self . tmra5fn () , self . tmra5ie0 () , self . tmra5ie1 () , self . tmra5clr () , self . tmra5pol () , self . tmrb5en () , self . tmrb5clk () , self . tmrb5fn () , self . tmrb5ie0 () , self . tmrb5ie1 () , self . tmrb5clr () , self . tmrb5pol () , self . ctlink5 ())
    }
}
#[doc = "Counter/Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl6(pub u32);
impl Ctrl6 {
    #[doc = "Counter/Timer A6 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 Enable bit."]
    #[inline(always)]
    pub const fn set_tmra6en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer A6 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6clk(&self) -> super::vals::Tmra6clk {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tmra6clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A6 Clock Select."]
    #[inline(always)]
    pub const fn set_tmra6clk(&mut self, val: super::vals::Tmra6clk) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Counter/Timer A6 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A6 Function Select."]
    #[inline(always)]
    pub const fn set_tmra6fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Counter/Timer A6 Interrupt Enable bit based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6ie0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub const fn set_tmra6ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A6 Interrupt Enable bit based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6ie1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub const fn set_tmra6ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer A6 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A6 Clear bit."]
    #[inline(always)]
    pub const fn set_tmra6clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A6 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra6pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A6 output polarity."]
    #[inline(always)]
    pub const fn set_tmra6pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B6 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 Enable bit."]
    #[inline(always)]
    pub const fn set_tmrb6en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B6 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6clk(&self) -> super::vals::Tmrb6clk {
        let val = (self.0 >> 17usize) & 0x1f;
        super::vals::Tmrb6clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B6 Clock Select."]
    #[inline(always)]
    pub const fn set_tmrb6clk(&mut self, val: super::vals::Tmrb6clk) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val.to_bits() as u32) & 0x1f) << 17usize);
    }
    #[doc = "Counter/Timer B6 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B6 Function Select."]
    #[inline(always)]
    pub const fn set_tmrb6fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Counter/Timer B6 Interrupt Enable bit for COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6ie0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub const fn set_tmrb6ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer B6 Interrupt Enable bit for COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6ie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub const fn set_tmrb6ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B6 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B6 Clear bit."]
    #[inline(always)]
    pub const fn set_tmrb6clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B6 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb6pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B6 output polarity."]
    #[inline(always)]
    pub const fn set_tmrb6pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer A6/B6 Link bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ctlink6(&self) -> super::vals::Ctlink {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ctlink::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A6/B6 Link bit."]
    #[inline(always)]
    pub const fn set_ctlink6(&mut self, val: super::vals::Ctlink) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl6 {
    #[inline(always)]
    fn default() -> Ctrl6 {
        Ctrl6(0)
    }
}
impl core::fmt::Debug for Ctrl6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl6")
            .field("tmra6en", &self.tmra6en())
            .field("tmra6clk", &self.tmra6clk())
            .field("tmra6fn", &self.tmra6fn())
            .field("tmra6ie0", &self.tmra6ie0())
            .field("tmra6ie1", &self.tmra6ie1())
            .field("tmra6clr", &self.tmra6clr())
            .field("tmra6pol", &self.tmra6pol())
            .field("tmrb6en", &self.tmrb6en())
            .field("tmrb6clk", &self.tmrb6clk())
            .field("tmrb6fn", &self.tmrb6fn())
            .field("tmrb6ie0", &self.tmrb6ie0())
            .field("tmrb6ie1", &self.tmrb6ie1())
            .field("tmrb6clr", &self.tmrb6clr())
            .field("tmrb6pol", &self.tmrb6pol())
            .field("ctlink6", &self.ctlink6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl6 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl6 {{ tmra6en: {=bool:?}, tmra6clk: {:?}, tmra6fn: {:?}, tmra6ie0: {=bool:?}, tmra6ie1: {=bool:?}, tmra6clr: {:?}, tmra6pol: {:?}, tmrb6en: {=bool:?}, tmrb6clk: {:?}, tmrb6fn: {:?}, tmrb6ie0: {=bool:?}, tmrb6ie1: {=bool:?}, tmrb6clr: {:?}, tmrb6pol: {:?}, ctlink6: {:?} }}" , self . tmra6en () , self . tmra6clk () , self . tmra6fn () , self . tmra6ie0 () , self . tmra6ie1 () , self . tmra6clr () , self . tmra6pol () , self . tmrb6en () , self . tmrb6clk () , self . tmrb6fn () , self . tmrb6ie0 () , self . tmrb6ie1 () , self . tmrb6clr () , self . tmrb6pol () , self . ctlink6 ())
    }
}
#[doc = "Counter/Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl7(pub u32);
impl Ctrl7 {
    #[doc = "Counter/Timer A7 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 Enable bit."]
    #[inline(always)]
    pub const fn set_tmra7en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer A7 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7clk(&self) -> super::vals::Tmra7clk {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Tmra7clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A7 Clock Select."]
    #[inline(always)]
    pub const fn set_tmra7clk(&mut self, val: super::vals::Tmra7clk) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Counter/Timer A7 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A7 Function Select."]
    #[inline(always)]
    pub const fn set_tmra7fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Counter/Timer A7 Interrupt Enable bit based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7ie0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub const fn set_tmra7ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A7 Interrupt Enable bit based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7ie1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub const fn set_tmra7ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer A7 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A7 Clear bit."]
    #[inline(always)]
    pub const fn set_tmra7clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A7 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmra7pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A7 output polarity."]
    #[inline(always)]
    pub const fn set_tmra7pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B7 Enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 Enable bit."]
    #[inline(always)]
    pub const fn set_tmrb7en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B7 Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7clk(&self) -> super::vals::Tmrb7clk {
        let val = (self.0 >> 17usize) & 0x1f;
        super::vals::Tmrb7clk::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B7 Clock Select."]
    #[inline(always)]
    pub const fn set_tmrb7clk(&mut self, val: super::vals::Tmrb7clk) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val.to_bits() as u32) & 0x1f) << 17usize);
    }
    #[doc = "Counter/Timer B7 Function Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7fn(&self) -> super::vals::Tmrfn {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Tmrfn::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B7 Function Select."]
    #[inline(always)]
    pub const fn set_tmrb7fn(&mut self, val: super::vals::Tmrfn) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Counter/Timer B7 Interrupt Enable bit for COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7ie0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub const fn set_tmrb7ie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer B7 Interrupt Enable bit for COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7ie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub const fn set_tmrb7ie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B7 Clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7clr(&self) -> super::vals::Clear {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B7 Clear bit."]
    #[inline(always)]
    pub const fn set_tmrb7clr(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer B7 output polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tmrb7pol(&self) -> super::vals::Tmrpol {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Tmrpol::from_bits(val as u8)
    }
    #[doc = "Counter/Timer B7 output polarity."]
    #[inline(always)]
    pub const fn set_tmrb7pol(&mut self, val: super::vals::Tmrpol) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer A7/B7 Link bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ctlink7(&self) -> super::vals::Ctlink {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ctlink::from_bits(val as u8)
    }
    #[doc = "Counter/Timer A7/B7 Link bit."]
    #[inline(always)]
    pub const fn set_ctlink7(&mut self, val: super::vals::Ctlink) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl7 {
    #[inline(always)]
    fn default() -> Ctrl7 {
        Ctrl7(0)
    }
}
impl core::fmt::Debug for Ctrl7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl7")
            .field("tmra7en", &self.tmra7en())
            .field("tmra7clk", &self.tmra7clk())
            .field("tmra7fn", &self.tmra7fn())
            .field("tmra7ie0", &self.tmra7ie0())
            .field("tmra7ie1", &self.tmra7ie1())
            .field("tmra7clr", &self.tmra7clr())
            .field("tmra7pol", &self.tmra7pol())
            .field("tmrb7en", &self.tmrb7en())
            .field("tmrb7clk", &self.tmrb7clk())
            .field("tmrb7fn", &self.tmrb7fn())
            .field("tmrb7ie0", &self.tmrb7ie0())
            .field("tmrb7ie1", &self.tmrb7ie1())
            .field("tmrb7clr", &self.tmrb7clr())
            .field("tmrb7pol", &self.tmrb7pol())
            .field("ctlink7", &self.ctlink7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl7 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl7 {{ tmra7en: {=bool:?}, tmra7clk: {:?}, tmra7fn: {:?}, tmra7ie0: {=bool:?}, tmra7ie1: {=bool:?}, tmra7clr: {:?}, tmra7pol: {:?}, tmrb7en: {=bool:?}, tmrb7clk: {:?}, tmrb7fn: {:?}, tmrb7ie0: {=bool:?}, tmrb7ie1: {=bool:?}, tmrb7clr: {:?}, tmrb7pol: {:?}, ctlink7: {:?} }}" , self . tmra7en () , self . tmra7clk () , self . tmra7fn () , self . tmra7ie0 () , self . tmra7ie1 () , self . tmra7clr () , self . tmra7pol () , self . tmrb7en () , self . tmrb7clk () , self . tmrb7fn () , self . tmrb7ie0 () , self . tmrb7ie1 () , self . tmrb7clr () , self . tmrb7pol () , self . ctlink7 ())
    }
}
#[doc = "Counter/Timer Global Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globen(pub u32);
impl Globen {
    #[doc = "Alternate enable for A0."]
    #[must_use]
    #[inline(always)]
    pub const fn ena0(&self) -> super::vals::En {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for A0."]
    #[inline(always)]
    pub const fn set_ena0(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Alternate enable for B0."]
    #[must_use]
    #[inline(always)]
    pub const fn enb0(&self) -> super::vals::En {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for B0."]
    #[inline(always)]
    pub const fn set_enb0(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Alternate enable for A1."]
    #[must_use]
    #[inline(always)]
    pub const fn ena1(&self) -> super::vals::En {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for A1."]
    #[inline(always)]
    pub const fn set_ena1(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Alternate enable for B1."]
    #[must_use]
    #[inline(always)]
    pub const fn enb1(&self) -> super::vals::En {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for B1."]
    #[inline(always)]
    pub const fn set_enb1(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Alternate enable for A2."]
    #[must_use]
    #[inline(always)]
    pub const fn ena2(&self) -> super::vals::En {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for A2."]
    #[inline(always)]
    pub const fn set_ena2(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Alternate enable for B2."]
    #[must_use]
    #[inline(always)]
    pub const fn enb2(&self) -> super::vals::En {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for B2."]
    #[inline(always)]
    pub const fn set_enb2(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Alternate enable for A3."]
    #[must_use]
    #[inline(always)]
    pub const fn ena3(&self) -> super::vals::En {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for A3."]
    #[inline(always)]
    pub const fn set_ena3(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Alternate enable for B3."]
    #[must_use]
    #[inline(always)]
    pub const fn enb3(&self) -> super::vals::En {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for B3."]
    #[inline(always)]
    pub const fn set_enb3(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate enable for A4."]
    #[must_use]
    #[inline(always)]
    pub const fn ena4(&self) -> super::vals::En {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for A4."]
    #[inline(always)]
    pub const fn set_ena4(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Alternate enable for B4."]
    #[must_use]
    #[inline(always)]
    pub const fn enb4(&self) -> super::vals::En {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for B4."]
    #[inline(always)]
    pub const fn set_enb4(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Alternate enable for A5."]
    #[must_use]
    #[inline(always)]
    pub const fn ena5(&self) -> super::vals::En {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for A5."]
    #[inline(always)]
    pub const fn set_ena5(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Alternate enable for B5."]
    #[must_use]
    #[inline(always)]
    pub const fn enb5(&self) -> super::vals::En {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for B5."]
    #[inline(always)]
    pub const fn set_enb5(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Alternate enable for A6."]
    #[must_use]
    #[inline(always)]
    pub const fn ena6(&self) -> super::vals::En {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for A6."]
    #[inline(always)]
    pub const fn set_ena6(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Alternate enable for B6."]
    #[must_use]
    #[inline(always)]
    pub const fn enb6(&self) -> super::vals::En {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for B6."]
    #[inline(always)]
    pub const fn set_enb6(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Alternate enable for A7."]
    #[must_use]
    #[inline(always)]
    pub const fn ena7(&self) -> super::vals::En {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for A7."]
    #[inline(always)]
    pub const fn set_ena7(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Alternate enable for B7."]
    #[must_use]
    #[inline(always)]
    pub const fn enb7(&self) -> super::vals::En {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "Alternate enable for B7."]
    #[inline(always)]
    pub const fn set_enb7(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Globen {
    #[inline(always)]
    fn default() -> Globen {
        Globen(0)
    }
}
impl core::fmt::Debug for Globen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Globen")
            .field("ena0", &self.ena0())
            .field("enb0", &self.enb0())
            .field("ena1", &self.ena1())
            .field("enb1", &self.enb1())
            .field("ena2", &self.ena2())
            .field("enb2", &self.enb2())
            .field("ena3", &self.ena3())
            .field("enb3", &self.enb3())
            .field("ena4", &self.ena4())
            .field("enb4", &self.enb4())
            .field("ena5", &self.ena5())
            .field("enb5", &self.enb5())
            .field("ena6", &self.ena6())
            .field("enb6", &self.enb6())
            .field("ena7", &self.ena7())
            .field("enb7", &self.enb7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Globen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Globen {{ ena0: {:?}, enb0: {:?}, ena1: {:?}, enb1: {:?}, ena2: {:?}, enb2: {:?}, ena3: {:?}, enb3: {:?}, ena4: {:?}, enb4: {:?}, ena5: {:?}, enb5: {:?}, ena6: {:?}, enb6: {:?}, ena7: {:?}, enb7: {:?} }}" , self . ena0 () , self . enb0 () , self . ena1 () , self . enb1 () , self . ena2 () , self . enb2 () , self . ena3 () , self . enb3 () , self . ena4 () , self . enb4 () , self . ena5 () , self . enb5 () , self . ena6 () , self . enb6 () , self . ena7 () , self . enb7 ())
    }
}
#[doc = "Counter/Timer Input Config."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Incfg(pub u32);
impl Incfg {
    #[doc = "CTIMER A0 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfga0(&self) -> super::vals::Cfga0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cfga0::from_bits(val as u8)
    }
    #[doc = "CTIMER A0 input configuration."]
    #[inline(always)]
    pub const fn set_cfga0(&mut self, val: super::vals::Cfga0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CTIMER B0 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgb0(&self) -> super::vals::Cfgb0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cfgb0::from_bits(val as u8)
    }
    #[doc = "CTIMER B0 input configuration."]
    #[inline(always)]
    pub const fn set_cfgb0(&mut self, val: super::vals::Cfgb0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CTIMER A1 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfga1(&self) -> super::vals::Cfga1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cfga1::from_bits(val as u8)
    }
    #[doc = "CTIMER A1 input configuration."]
    #[inline(always)]
    pub const fn set_cfga1(&mut self, val: super::vals::Cfga1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CTIMER B1 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgb1(&self) -> super::vals::Cfgb1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cfgb1::from_bits(val as u8)
    }
    #[doc = "CTIMER B1 input configuration."]
    #[inline(always)]
    pub const fn set_cfgb1(&mut self, val: super::vals::Cfgb1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "CTIMER A2 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfga2(&self) -> super::vals::Cfga2 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Cfga2::from_bits(val as u8)
    }
    #[doc = "CTIMER A2 input configuration."]
    #[inline(always)]
    pub const fn set_cfga2(&mut self, val: super::vals::Cfga2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "CTIMER B2 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgb2(&self) -> super::vals::Cfgb2 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cfgb2::from_bits(val as u8)
    }
    #[doc = "CTIMER B2 input configuration."]
    #[inline(always)]
    pub const fn set_cfgb2(&mut self, val: super::vals::Cfgb2) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "CTIMER A3 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfga3(&self) -> super::vals::Cfga3 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cfga3::from_bits(val as u8)
    }
    #[doc = "CTIMER A3 input configuration."]
    #[inline(always)]
    pub const fn set_cfga3(&mut self, val: super::vals::Cfga3) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CTIMER B3 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgb3(&self) -> super::vals::Cfgb3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cfgb3::from_bits(val as u8)
    }
    #[doc = "CTIMER B3 input configuration."]
    #[inline(always)]
    pub const fn set_cfgb3(&mut self, val: super::vals::Cfgb3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CTIMER A4 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfga4(&self) -> super::vals::Cfga4 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cfga4::from_bits(val as u8)
    }
    #[doc = "CTIMER A4 input configuration."]
    #[inline(always)]
    pub const fn set_cfga4(&mut self, val: super::vals::Cfga4) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CTIMER B4 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgb4(&self) -> super::vals::Cfgb4 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cfgb4::from_bits(val as u8)
    }
    #[doc = "CTIMER B4 input configuration."]
    #[inline(always)]
    pub const fn set_cfgb4(&mut self, val: super::vals::Cfgb4) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "CTIMER A5 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfga5(&self) -> super::vals::Cfga5 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Cfga5::from_bits(val as u8)
    }
    #[doc = "CTIMER A5 input configuration."]
    #[inline(always)]
    pub const fn set_cfga5(&mut self, val: super::vals::Cfga5) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "CTIMER B5 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgb5(&self) -> super::vals::Cfgb5 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cfgb5::from_bits(val as u8)
    }
    #[doc = "CTIMER B5 input configuration."]
    #[inline(always)]
    pub const fn set_cfgb5(&mut self, val: super::vals::Cfgb5) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "CTIMER A6 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfga6(&self) -> super::vals::Cfga6 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Cfga6::from_bits(val as u8)
    }
    #[doc = "CTIMER A6 input configuration."]
    #[inline(always)]
    pub const fn set_cfga6(&mut self, val: super::vals::Cfga6) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "CTIMER B6 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgb6(&self) -> super::vals::Cfgb6 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cfgb6::from_bits(val as u8)
    }
    #[doc = "CTIMER B6 input configuration."]
    #[inline(always)]
    pub const fn set_cfgb6(&mut self, val: super::vals::Cfgb6) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "CTIMER A7 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfga7(&self) -> super::vals::Cfga7 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Cfga7::from_bits(val as u8)
    }
    #[doc = "CTIMER A7 input configuration."]
    #[inline(always)]
    pub const fn set_cfga7(&mut self, val: super::vals::Cfga7) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "CTIMER B7 input configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfgb7(&self) -> super::vals::Cfgb7 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Cfgb7::from_bits(val as u8)
    }
    #[doc = "CTIMER B7 input configuration."]
    #[inline(always)]
    pub const fn set_cfgb7(&mut self, val: super::vals::Cfgb7) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Incfg {
    #[inline(always)]
    fn default() -> Incfg {
        Incfg(0)
    }
}
impl core::fmt::Debug for Incfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Incfg")
            .field("cfga0", &self.cfga0())
            .field("cfgb0", &self.cfgb0())
            .field("cfga1", &self.cfga1())
            .field("cfgb1", &self.cfgb1())
            .field("cfga2", &self.cfga2())
            .field("cfgb2", &self.cfgb2())
            .field("cfga3", &self.cfga3())
            .field("cfgb3", &self.cfgb3())
            .field("cfga4", &self.cfga4())
            .field("cfgb4", &self.cfgb4())
            .field("cfga5", &self.cfga5())
            .field("cfgb5", &self.cfgb5())
            .field("cfga6", &self.cfga6())
            .field("cfgb6", &self.cfgb6())
            .field("cfga7", &self.cfga7())
            .field("cfgb7", &self.cfgb7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Incfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Incfg {{ cfga0: {:?}, cfgb0: {:?}, cfga1: {:?}, cfgb1: {:?}, cfga2: {:?}, cfgb2: {:?}, cfga3: {:?}, cfgb3: {:?}, cfga4: {:?}, cfgb4: {:?}, cfga5: {:?}, cfgb5: {:?}, cfga6: {:?}, cfgb6: {:?}, cfga7: {:?}, cfgb7: {:?} }}" , self . cfga0 () , self . cfgb0 () , self . cfga1 () , self . cfgb1 () , self . cfga2 () , self . cfgb2 () , self . cfga3 () , self . cfgb3 () , self . cfga4 () , self . cfgb4 () , self . cfga5 () , self . cfgb5 () , self . cfga6 () , self . cfgb6 () , self . cfga7 () , self . cfgb7 ())
    }
}
#[doc = "Counter/Timer Interrupts: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "Counter/Timer A0 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra0c0int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra0c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb0c0int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb0c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra1c0int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra1c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb1c0int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb1c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra2c0int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra2c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb2c0int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb2c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra3c0int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra3c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb3c0int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb3c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra4c0int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra4c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb4c0int(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb4c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra5c0int(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra5c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb5c0int(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb5c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra6c0int(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra6c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb6c0int(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb6c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra7c0int(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra7c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb7c0int(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb7c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra0c1int(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra0c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb0c1int(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb0c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra1c1int(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra1c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb1c1int(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb1c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra2c1int(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra2c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb2c1int(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb2c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra3c1int(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra3c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb3c1int(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb3c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra4c1int(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra4c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb4c1int(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb4c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra5c1int(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra5c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb5c1int(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb5c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra6c1int(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra6c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb6c1int(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb6c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra7c1int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra7c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb7c1int(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb7c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("ctmra0c0int", &self.ctmra0c0int())
            .field("ctmrb0c0int", &self.ctmrb0c0int())
            .field("ctmra1c0int", &self.ctmra1c0int())
            .field("ctmrb1c0int", &self.ctmrb1c0int())
            .field("ctmra2c0int", &self.ctmra2c0int())
            .field("ctmrb2c0int", &self.ctmrb2c0int())
            .field("ctmra3c0int", &self.ctmra3c0int())
            .field("ctmrb3c0int", &self.ctmrb3c0int())
            .field("ctmra4c0int", &self.ctmra4c0int())
            .field("ctmrb4c0int", &self.ctmrb4c0int())
            .field("ctmra5c0int", &self.ctmra5c0int())
            .field("ctmrb5c0int", &self.ctmrb5c0int())
            .field("ctmra6c0int", &self.ctmra6c0int())
            .field("ctmrb6c0int", &self.ctmrb6c0int())
            .field("ctmra7c0int", &self.ctmra7c0int())
            .field("ctmrb7c0int", &self.ctmrb7c0int())
            .field("ctmra0c1int", &self.ctmra0c1int())
            .field("ctmrb0c1int", &self.ctmrb0c1int())
            .field("ctmra1c1int", &self.ctmra1c1int())
            .field("ctmrb1c1int", &self.ctmrb1c1int())
            .field("ctmra2c1int", &self.ctmra2c1int())
            .field("ctmrb2c1int", &self.ctmrb2c1int())
            .field("ctmra3c1int", &self.ctmra3c1int())
            .field("ctmrb3c1int", &self.ctmrb3c1int())
            .field("ctmra4c1int", &self.ctmra4c1int())
            .field("ctmrb4c1int", &self.ctmrb4c1int())
            .field("ctmra5c1int", &self.ctmra5c1int())
            .field("ctmrb5c1int", &self.ctmrb5c1int())
            .field("ctmra6c1int", &self.ctmra6c1int())
            .field("ctmrb6c1int", &self.ctmrb6c1int())
            .field("ctmra7c1int", &self.ctmra7c1int())
            .field("ctmrb7c1int", &self.ctmrb7c1int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intclr {{ ctmra0c0int: {=bool:?}, ctmrb0c0int: {=bool:?}, ctmra1c0int: {=bool:?}, ctmrb1c0int: {=bool:?}, ctmra2c0int: {=bool:?}, ctmrb2c0int: {=bool:?}, ctmra3c0int: {=bool:?}, ctmrb3c0int: {=bool:?}, ctmra4c0int: {=bool:?}, ctmrb4c0int: {=bool:?}, ctmra5c0int: {=bool:?}, ctmrb5c0int: {=bool:?}, ctmra6c0int: {=bool:?}, ctmrb6c0int: {=bool:?}, ctmra7c0int: {=bool:?}, ctmrb7c0int: {=bool:?}, ctmra0c1int: {=bool:?}, ctmrb0c1int: {=bool:?}, ctmra1c1int: {=bool:?}, ctmrb1c1int: {=bool:?}, ctmra2c1int: {=bool:?}, ctmrb2c1int: {=bool:?}, ctmra3c1int: {=bool:?}, ctmrb3c1int: {=bool:?}, ctmra4c1int: {=bool:?}, ctmrb4c1int: {=bool:?}, ctmra5c1int: {=bool:?}, ctmrb5c1int: {=bool:?}, ctmra6c1int: {=bool:?}, ctmrb6c1int: {=bool:?}, ctmra7c1int: {=bool:?}, ctmrb7c1int: {=bool:?} }}" , self . ctmra0c0int () , self . ctmrb0c0int () , self . ctmra1c0int () , self . ctmrb1c0int () , self . ctmra2c0int () , self . ctmrb2c0int () , self . ctmra3c0int () , self . ctmrb3c0int () , self . ctmra4c0int () , self . ctmrb4c0int () , self . ctmra5c0int () , self . ctmrb5c0int () , self . ctmra6c0int () , self . ctmrb6c0int () , self . ctmra7c0int () , self . ctmrb7c0int () , self . ctmra0c1int () , self . ctmrb0c1int () , self . ctmra1c1int () , self . ctmrb1c1int () , self . ctmra2c1int () , self . ctmrb2c1int () , self . ctmra3c1int () , self . ctmrb3c1int () , self . ctmra4c1int () , self . ctmrb4c1int () , self . ctmra5c1int () , self . ctmrb5c1int () , self . ctmra6c1int () , self . ctmrb6c1int () , self . ctmra7c1int () , self . ctmrb7c1int ())
    }
}
#[doc = "Counter/Timer Interrupts: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Counter/Timer A0 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra0c0int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra0c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb0c0int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb0c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra1c0int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra1c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb1c0int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb1c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra2c0int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra2c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb2c0int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb2c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra3c0int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra3c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb3c0int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb3c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra4c0int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra4c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb4c0int(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb4c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra5c0int(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra5c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb5c0int(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb5c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra6c0int(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra6c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb6c0int(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb6c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra7c0int(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra7c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb7c0int(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb7c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra0c1int(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra0c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb0c1int(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb0c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra1c1int(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra1c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb1c1int(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb1c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra2c1int(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra2c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb2c1int(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb2c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra3c1int(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra3c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb3c1int(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb3c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra4c1int(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra4c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb4c1int(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb4c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra5c1int(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra5c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb5c1int(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb5c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra6c1int(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra6c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb6c1int(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb6c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra7c1int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra7c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb7c1int(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb7c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("ctmra0c0int", &self.ctmra0c0int())
            .field("ctmrb0c0int", &self.ctmrb0c0int())
            .field("ctmra1c0int", &self.ctmra1c0int())
            .field("ctmrb1c0int", &self.ctmrb1c0int())
            .field("ctmra2c0int", &self.ctmra2c0int())
            .field("ctmrb2c0int", &self.ctmrb2c0int())
            .field("ctmra3c0int", &self.ctmra3c0int())
            .field("ctmrb3c0int", &self.ctmrb3c0int())
            .field("ctmra4c0int", &self.ctmra4c0int())
            .field("ctmrb4c0int", &self.ctmrb4c0int())
            .field("ctmra5c0int", &self.ctmra5c0int())
            .field("ctmrb5c0int", &self.ctmrb5c0int())
            .field("ctmra6c0int", &self.ctmra6c0int())
            .field("ctmrb6c0int", &self.ctmrb6c0int())
            .field("ctmra7c0int", &self.ctmra7c0int())
            .field("ctmrb7c0int", &self.ctmrb7c0int())
            .field("ctmra0c1int", &self.ctmra0c1int())
            .field("ctmrb0c1int", &self.ctmrb0c1int())
            .field("ctmra1c1int", &self.ctmra1c1int())
            .field("ctmrb1c1int", &self.ctmrb1c1int())
            .field("ctmra2c1int", &self.ctmra2c1int())
            .field("ctmrb2c1int", &self.ctmrb2c1int())
            .field("ctmra3c1int", &self.ctmra3c1int())
            .field("ctmrb3c1int", &self.ctmrb3c1int())
            .field("ctmra4c1int", &self.ctmra4c1int())
            .field("ctmrb4c1int", &self.ctmrb4c1int())
            .field("ctmra5c1int", &self.ctmra5c1int())
            .field("ctmrb5c1int", &self.ctmrb5c1int())
            .field("ctmra6c1int", &self.ctmra6c1int())
            .field("ctmrb6c1int", &self.ctmrb6c1int())
            .field("ctmra7c1int", &self.ctmra7c1int())
            .field("ctmrb7c1int", &self.ctmrb7c1int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inten {{ ctmra0c0int: {=bool:?}, ctmrb0c0int: {=bool:?}, ctmra1c0int: {=bool:?}, ctmrb1c0int: {=bool:?}, ctmra2c0int: {=bool:?}, ctmrb2c0int: {=bool:?}, ctmra3c0int: {=bool:?}, ctmrb3c0int: {=bool:?}, ctmra4c0int: {=bool:?}, ctmrb4c0int: {=bool:?}, ctmra5c0int: {=bool:?}, ctmrb5c0int: {=bool:?}, ctmra6c0int: {=bool:?}, ctmrb6c0int: {=bool:?}, ctmra7c0int: {=bool:?}, ctmrb7c0int: {=bool:?}, ctmra0c1int: {=bool:?}, ctmrb0c1int: {=bool:?}, ctmra1c1int: {=bool:?}, ctmrb1c1int: {=bool:?}, ctmra2c1int: {=bool:?}, ctmrb2c1int: {=bool:?}, ctmra3c1int: {=bool:?}, ctmrb3c1int: {=bool:?}, ctmra4c1int: {=bool:?}, ctmrb4c1int: {=bool:?}, ctmra5c1int: {=bool:?}, ctmrb5c1int: {=bool:?}, ctmra6c1int: {=bool:?}, ctmrb6c1int: {=bool:?}, ctmra7c1int: {=bool:?}, ctmrb7c1int: {=bool:?} }}" , self . ctmra0c0int () , self . ctmrb0c0int () , self . ctmra1c0int () , self . ctmrb1c0int () , self . ctmra2c0int () , self . ctmrb2c0int () , self . ctmra3c0int () , self . ctmrb3c0int () , self . ctmra4c0int () , self . ctmrb4c0int () , self . ctmra5c0int () , self . ctmrb5c0int () , self . ctmra6c0int () , self . ctmrb6c0int () , self . ctmra7c0int () , self . ctmrb7c0int () , self . ctmra0c1int () , self . ctmrb0c1int () , self . ctmra1c1int () , self . ctmrb1c1int () , self . ctmra2c1int () , self . ctmrb2c1int () , self . ctmra3c1int () , self . ctmrb3c1int () , self . ctmra4c1int () , self . ctmrb4c1int () , self . ctmra5c1int () , self . ctmrb5c1int () , self . ctmra6c1int () , self . ctmrb6c1int () , self . ctmra7c1int () , self . ctmrb7c1int ())
    }
}
#[doc = "Counter/Timer Interrupts: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "Counter/Timer A0 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra0c0int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra0c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb0c0int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb0c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra1c0int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra1c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb1c0int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb1c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra2c0int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra2c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb2c0int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb2c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra3c0int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra3c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb3c0int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb3c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra4c0int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra4c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb4c0int(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb4c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra5c0int(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra5c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb5c0int(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb5c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra6c0int(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra6c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb6c0int(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb6c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra7c0int(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra7c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb7c0int(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb7c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra0c1int(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra0c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb0c1int(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb0c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra1c1int(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra1c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb1c1int(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb1c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra2c1int(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra2c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb2c1int(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb2c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra3c1int(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra3c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb3c1int(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb3c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra4c1int(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra4c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb4c1int(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb4c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra5c1int(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra5c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb5c1int(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb5c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra6c1int(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra6c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb6c1int(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb6c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra7c1int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra7c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb7c1int(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb7c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("ctmra0c0int", &self.ctmra0c0int())
            .field("ctmrb0c0int", &self.ctmrb0c0int())
            .field("ctmra1c0int", &self.ctmra1c0int())
            .field("ctmrb1c0int", &self.ctmrb1c0int())
            .field("ctmra2c0int", &self.ctmra2c0int())
            .field("ctmrb2c0int", &self.ctmrb2c0int())
            .field("ctmra3c0int", &self.ctmra3c0int())
            .field("ctmrb3c0int", &self.ctmrb3c0int())
            .field("ctmra4c0int", &self.ctmra4c0int())
            .field("ctmrb4c0int", &self.ctmrb4c0int())
            .field("ctmra5c0int", &self.ctmra5c0int())
            .field("ctmrb5c0int", &self.ctmrb5c0int())
            .field("ctmra6c0int", &self.ctmra6c0int())
            .field("ctmrb6c0int", &self.ctmrb6c0int())
            .field("ctmra7c0int", &self.ctmra7c0int())
            .field("ctmrb7c0int", &self.ctmrb7c0int())
            .field("ctmra0c1int", &self.ctmra0c1int())
            .field("ctmrb0c1int", &self.ctmrb0c1int())
            .field("ctmra1c1int", &self.ctmra1c1int())
            .field("ctmrb1c1int", &self.ctmrb1c1int())
            .field("ctmra2c1int", &self.ctmra2c1int())
            .field("ctmrb2c1int", &self.ctmrb2c1int())
            .field("ctmra3c1int", &self.ctmra3c1int())
            .field("ctmrb3c1int", &self.ctmrb3c1int())
            .field("ctmra4c1int", &self.ctmra4c1int())
            .field("ctmrb4c1int", &self.ctmrb4c1int())
            .field("ctmra5c1int", &self.ctmra5c1int())
            .field("ctmrb5c1int", &self.ctmrb5c1int())
            .field("ctmra6c1int", &self.ctmra6c1int())
            .field("ctmrb6c1int", &self.ctmrb6c1int())
            .field("ctmra7c1int", &self.ctmra7c1int())
            .field("ctmrb7c1int", &self.ctmrb7c1int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intset {{ ctmra0c0int: {=bool:?}, ctmrb0c0int: {=bool:?}, ctmra1c0int: {=bool:?}, ctmrb1c0int: {=bool:?}, ctmra2c0int: {=bool:?}, ctmrb2c0int: {=bool:?}, ctmra3c0int: {=bool:?}, ctmrb3c0int: {=bool:?}, ctmra4c0int: {=bool:?}, ctmrb4c0int: {=bool:?}, ctmra5c0int: {=bool:?}, ctmrb5c0int: {=bool:?}, ctmra6c0int: {=bool:?}, ctmrb6c0int: {=bool:?}, ctmra7c0int: {=bool:?}, ctmrb7c0int: {=bool:?}, ctmra0c1int: {=bool:?}, ctmrb0c1int: {=bool:?}, ctmra1c1int: {=bool:?}, ctmrb1c1int: {=bool:?}, ctmra2c1int: {=bool:?}, ctmrb2c1int: {=bool:?}, ctmra3c1int: {=bool:?}, ctmrb3c1int: {=bool:?}, ctmra4c1int: {=bool:?}, ctmrb4c1int: {=bool:?}, ctmra5c1int: {=bool:?}, ctmrb5c1int: {=bool:?}, ctmra6c1int: {=bool:?}, ctmrb6c1int: {=bool:?}, ctmra7c1int: {=bool:?}, ctmrb7c1int: {=bool:?} }}" , self . ctmra0c0int () , self . ctmrb0c0int () , self . ctmra1c0int () , self . ctmrb1c0int () , self . ctmra2c0int () , self . ctmrb2c0int () , self . ctmra3c0int () , self . ctmrb3c0int () , self . ctmra4c0int () , self . ctmrb4c0int () , self . ctmra5c0int () , self . ctmrb5c0int () , self . ctmra6c0int () , self . ctmrb6c0int () , self . ctmra7c0int () , self . ctmrb7c0int () , self . ctmra0c1int () , self . ctmrb0c1int () , self . ctmra1c1int () , self . ctmrb1c1int () , self . ctmra2c1int () , self . ctmrb2c1int () , self . ctmra3c1int () , self . ctmrb3c1int () , self . ctmra4c1int () , self . ctmrb4c1int () , self . ctmra5c1int () , self . ctmrb5c1int () , self . ctmra6c1int () , self . ctmrb6c1int () , self . ctmra7c1int () , self . ctmrb7c1int ())
    }
}
#[doc = "Counter/Timer Interrupts: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Counter/Timer A0 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra0c0int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra0c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb0c0int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb0c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra1c0int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra1c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb1c0int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb1c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra2c0int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra2c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb2c0int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb2c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra3c0int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra3c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb3c0int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb3c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra4c0int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra4c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb4c0int(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb4c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra5c0int(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra5c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb5c0int(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb5c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra6c0int(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra6c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb6c0int(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb6c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra7c0int(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmra7c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb7c0int(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR0."]
    #[inline(always)]
    pub const fn set_ctmrb7c0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra0c1int(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A0 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra0c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb0c1int(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B0 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb0c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra1c1int(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A1 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra1c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb1c1int(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B1 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb1c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra2c1int(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A2 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra2c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb2c1int(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B2 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb2c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra3c1int(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A3 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra3c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb3c1int(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B3 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb3c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra4c1int(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A4 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra4c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb4c1int(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B4 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb4c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra5c1int(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A5 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra5c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb5c1int(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B5 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb5c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra6c1int(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A6 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra6c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb6c1int(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B6 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb6c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmra7c1int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer A7 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmra7c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmrb7c1int(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Counter/Timer B7 interrupt based on COMPR1."]
    #[inline(always)]
    pub const fn set_ctmrb7c1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("ctmra0c0int", &self.ctmra0c0int())
            .field("ctmrb0c0int", &self.ctmrb0c0int())
            .field("ctmra1c0int", &self.ctmra1c0int())
            .field("ctmrb1c0int", &self.ctmrb1c0int())
            .field("ctmra2c0int", &self.ctmra2c0int())
            .field("ctmrb2c0int", &self.ctmrb2c0int())
            .field("ctmra3c0int", &self.ctmra3c0int())
            .field("ctmrb3c0int", &self.ctmrb3c0int())
            .field("ctmra4c0int", &self.ctmra4c0int())
            .field("ctmrb4c0int", &self.ctmrb4c0int())
            .field("ctmra5c0int", &self.ctmra5c0int())
            .field("ctmrb5c0int", &self.ctmrb5c0int())
            .field("ctmra6c0int", &self.ctmra6c0int())
            .field("ctmrb6c0int", &self.ctmrb6c0int())
            .field("ctmra7c0int", &self.ctmra7c0int())
            .field("ctmrb7c0int", &self.ctmrb7c0int())
            .field("ctmra0c1int", &self.ctmra0c1int())
            .field("ctmrb0c1int", &self.ctmrb0c1int())
            .field("ctmra1c1int", &self.ctmra1c1int())
            .field("ctmrb1c1int", &self.ctmrb1c1int())
            .field("ctmra2c1int", &self.ctmra2c1int())
            .field("ctmrb2c1int", &self.ctmrb2c1int())
            .field("ctmra3c1int", &self.ctmra3c1int())
            .field("ctmrb3c1int", &self.ctmrb3c1int())
            .field("ctmra4c1int", &self.ctmra4c1int())
            .field("ctmrb4c1int", &self.ctmrb4c1int())
            .field("ctmra5c1int", &self.ctmra5c1int())
            .field("ctmrb5c1int", &self.ctmrb5c1int())
            .field("ctmra6c1int", &self.ctmra6c1int())
            .field("ctmrb6c1int", &self.ctmrb6c1int())
            .field("ctmra7c1int", &self.ctmra7c1int())
            .field("ctmrb7c1int", &self.ctmrb7c1int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intstat {{ ctmra0c0int: {=bool:?}, ctmrb0c0int: {=bool:?}, ctmra1c0int: {=bool:?}, ctmrb1c0int: {=bool:?}, ctmra2c0int: {=bool:?}, ctmrb2c0int: {=bool:?}, ctmra3c0int: {=bool:?}, ctmrb3c0int: {=bool:?}, ctmra4c0int: {=bool:?}, ctmrb4c0int: {=bool:?}, ctmra5c0int: {=bool:?}, ctmrb5c0int: {=bool:?}, ctmra6c0int: {=bool:?}, ctmrb6c0int: {=bool:?}, ctmra7c0int: {=bool:?}, ctmrb7c0int: {=bool:?}, ctmra0c1int: {=bool:?}, ctmrb0c1int: {=bool:?}, ctmra1c1int: {=bool:?}, ctmrb1c1int: {=bool:?}, ctmra2c1int: {=bool:?}, ctmrb2c1int: {=bool:?}, ctmra3c1int: {=bool:?}, ctmrb3c1int: {=bool:?}, ctmra4c1int: {=bool:?}, ctmrb4c1int: {=bool:?}, ctmra5c1int: {=bool:?}, ctmrb5c1int: {=bool:?}, ctmra6c1int: {=bool:?}, ctmrb6c1int: {=bool:?}, ctmra7c1int: {=bool:?}, ctmrb7c1int: {=bool:?} }}" , self . ctmra0c0int () , self . ctmrb0c0int () , self . ctmra1c0int () , self . ctmrb1c0int () , self . ctmra2c0int () , self . ctmrb2c0int () , self . ctmra3c0int () , self . ctmrb3c0int () , self . ctmra4c0int () , self . ctmrb4c0int () , self . ctmra5c0int () , self . ctmrb5c0int () , self . ctmra6c0int () , self . ctmrb6c0int () , self . ctmra7c0int () , self . ctmrb7c0int () , self . ctmra0c1int () , self . ctmrb0c1int () , self . ctmra1c1int () , self . ctmrb1c1int () , self . ctmra2c1int () , self . ctmrb2c1int () , self . ctmra3c1int () , self . ctmrb3c1int () , self . ctmra4c1int () , self . ctmrb4c1int () , self . ctmra5c1int () , self . ctmrb5c1int () , self . ctmra6c1int () , self . ctmrb6c1int () , self . ctmra7c1int () , self . ctmrb7c1int ())
    }
}
#[doc = "Counter/Timer Output Config 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outcfg0(pub u32);
impl Outcfg0 {
    #[doc = "Pad output 0 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg0(&self) -> super::vals::Cfg0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cfg0::from_bits(val as u8)
    }
    #[doc = "Pad output 0 configuration."]
    #[inline(always)]
    pub const fn set_cfg0(&mut self, val: super::vals::Cfg0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Pad output 1 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg1(&self) -> super::vals::Cfg1 {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Cfg1::from_bits(val as u8)
    }
    #[doc = "Pad output 1 configuration."]
    #[inline(always)]
    pub const fn set_cfg1(&mut self, val: super::vals::Cfg1) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad output 2 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg2(&self) -> super::vals::Cfg2 {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Cfg2::from_bits(val as u8)
    }
    #[doc = "Pad output 2 configuration."]
    #[inline(always)]
    pub const fn set_cfg2(&mut self, val: super::vals::Cfg2) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Pad output 3 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg3(&self) -> super::vals::Cfg3 {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::Cfg3::from_bits(val as u8)
    }
    #[doc = "Pad output 3 configuration."]
    #[inline(always)]
    pub const fn set_cfg3(&mut self, val: super::vals::Cfg3) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Pad output 4 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg4(&self) -> super::vals::Cfg4 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cfg4::from_bits(val as u8)
    }
    #[doc = "Pad output 4 configuration."]
    #[inline(always)]
    pub const fn set_cfg4(&mut self, val: super::vals::Cfg4) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Pad output 5 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg5(&self) -> super::vals::Cfg5 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Cfg5::from_bits(val as u8)
    }
    #[doc = "Pad output 5 configuration."]
    #[inline(always)]
    pub const fn set_cfg5(&mut self, val: super::vals::Cfg5) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Pad output 6 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg6(&self) -> super::vals::Cfg6 {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Cfg6::from_bits(val as u8)
    }
    #[doc = "Pad output 6 configuration."]
    #[inline(always)]
    pub const fn set_cfg6(&mut self, val: super::vals::Cfg6) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad output 7 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg7(&self) -> super::vals::Cfg7 {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Cfg7::from_bits(val as u8)
    }
    #[doc = "Pad output 7 configuration."]
    #[inline(always)]
    pub const fn set_cfg7(&mut self, val: super::vals::Cfg7) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Pad output 8 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg8(&self) -> super::vals::Cfg8 {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::Cfg8::from_bits(val as u8)
    }
    #[doc = "Pad output 8 configuration."]
    #[inline(always)]
    pub const fn set_cfg8(&mut self, val: super::vals::Cfg8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
    #[doc = "Pad output 9 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg9(&self) -> super::vals::Cfg9 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Cfg9::from_bits(val as u8)
    }
    #[doc = "Pad output 9 configuration."]
    #[inline(always)]
    pub const fn set_cfg9(&mut self, val: super::vals::Cfg9) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Outcfg0 {
    #[inline(always)]
    fn default() -> Outcfg0 {
        Outcfg0(0)
    }
}
impl core::fmt::Debug for Outcfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outcfg0")
            .field("cfg0", &self.cfg0())
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .field("cfg3", &self.cfg3())
            .field("cfg4", &self.cfg4())
            .field("cfg5", &self.cfg5())
            .field("cfg6", &self.cfg6())
            .field("cfg7", &self.cfg7())
            .field("cfg8", &self.cfg8())
            .field("cfg9", &self.cfg9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outcfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Outcfg0 {{ cfg0: {:?}, cfg1: {:?}, cfg2: {:?}, cfg3: {:?}, cfg4: {:?}, cfg5: {:?}, cfg6: {:?}, cfg7: {:?}, cfg8: {:?}, cfg9: {:?} }}" , self . cfg0 () , self . cfg1 () , self . cfg2 () , self . cfg3 () , self . cfg4 () , self . cfg5 () , self . cfg6 () , self . cfg7 () , self . cfg8 () , self . cfg9 ())
    }
}
#[doc = "Counter/Timer Output Config 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outcfg1(pub u32);
impl Outcfg1 {
    #[doc = "Pad output 10 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg10(&self) -> super::vals::Cfg10 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cfg10::from_bits(val as u8)
    }
    #[doc = "Pad output 10 configuration."]
    #[inline(always)]
    pub const fn set_cfg10(&mut self, val: super::vals::Cfg10) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Pad output 11 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg11(&self) -> super::vals::Cfg11 {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Cfg11::from_bits(val as u8)
    }
    #[doc = "Pad output 11 configuration."]
    #[inline(always)]
    pub const fn set_cfg11(&mut self, val: super::vals::Cfg11) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad output 12 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg12(&self) -> super::vals::Cfg12 {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Cfg12::from_bits(val as u8)
    }
    #[doc = "Pad output 12 configuration."]
    #[inline(always)]
    pub const fn set_cfg12(&mut self, val: super::vals::Cfg12) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Pad output 13 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg13(&self) -> super::vals::Cfg13 {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::Cfg13::from_bits(val as u8)
    }
    #[doc = "Pad output 13 configuration."]
    #[inline(always)]
    pub const fn set_cfg13(&mut self, val: super::vals::Cfg13) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Pad output 14 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg14(&self) -> super::vals::Cfg14 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cfg14::from_bits(val as u8)
    }
    #[doc = "Pad output 14 configuration."]
    #[inline(always)]
    pub const fn set_cfg14(&mut self, val: super::vals::Cfg14) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Pad output 15 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg15(&self) -> super::vals::Cfg15 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Cfg15::from_bits(val as u8)
    }
    #[doc = "Pad output 15 configuration."]
    #[inline(always)]
    pub const fn set_cfg15(&mut self, val: super::vals::Cfg15) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Pad output 16 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg16(&self) -> super::vals::Cfg16 {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Cfg16::from_bits(val as u8)
    }
    #[doc = "Pad output 16 configuration."]
    #[inline(always)]
    pub const fn set_cfg16(&mut self, val: super::vals::Cfg16) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad output 17 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg17(&self) -> super::vals::Cfg17 {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Cfg17::from_bits(val as u8)
    }
    #[doc = "Pad output 17 configuration."]
    #[inline(always)]
    pub const fn set_cfg17(&mut self, val: super::vals::Cfg17) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Pad output 18 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg18(&self) -> super::vals::Cfg18 {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::Cfg18::from_bits(val as u8)
    }
    #[doc = "Pad output 18 configuration."]
    #[inline(always)]
    pub const fn set_cfg18(&mut self, val: super::vals::Cfg18) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
    #[doc = "Pad output 19 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg19(&self) -> super::vals::Cfg19 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Cfg19::from_bits(val as u8)
    }
    #[doc = "Pad output 19 configuration."]
    #[inline(always)]
    pub const fn set_cfg19(&mut self, val: super::vals::Cfg19) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Outcfg1 {
    #[inline(always)]
    fn default() -> Outcfg1 {
        Outcfg1(0)
    }
}
impl core::fmt::Debug for Outcfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outcfg1")
            .field("cfg10", &self.cfg10())
            .field("cfg11", &self.cfg11())
            .field("cfg12", &self.cfg12())
            .field("cfg13", &self.cfg13())
            .field("cfg14", &self.cfg14())
            .field("cfg15", &self.cfg15())
            .field("cfg16", &self.cfg16())
            .field("cfg17", &self.cfg17())
            .field("cfg18", &self.cfg18())
            .field("cfg19", &self.cfg19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outcfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Outcfg1 {{ cfg10: {:?}, cfg11: {:?}, cfg12: {:?}, cfg13: {:?}, cfg14: {:?}, cfg15: {:?}, cfg16: {:?}, cfg17: {:?}, cfg18: {:?}, cfg19: {:?} }}" , self . cfg10 () , self . cfg11 () , self . cfg12 () , self . cfg13 () , self . cfg14 () , self . cfg15 () , self . cfg16 () , self . cfg17 () , self . cfg18 () , self . cfg19 ())
    }
}
#[doc = "Counter/Timer Output Config 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outcfg2(pub u32);
impl Outcfg2 {
    #[doc = "Pad output 20 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg20(&self) -> super::vals::Cfg20 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cfg20::from_bits(val as u8)
    }
    #[doc = "Pad output 20 configuration."]
    #[inline(always)]
    pub const fn set_cfg20(&mut self, val: super::vals::Cfg20) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Pad output 21 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg21(&self) -> super::vals::Cfg21 {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Cfg21::from_bits(val as u8)
    }
    #[doc = "Pad output 21 configuration."]
    #[inline(always)]
    pub const fn set_cfg21(&mut self, val: super::vals::Cfg21) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Pad output 22 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg22(&self) -> super::vals::Cfg22 {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Cfg22::from_bits(val as u8)
    }
    #[doc = "Pad output 22 configuration."]
    #[inline(always)]
    pub const fn set_cfg22(&mut self, val: super::vals::Cfg22) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Pad output 23 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg23(&self) -> super::vals::Cfg23 {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::Cfg23::from_bits(val as u8)
    }
    #[doc = "Pad output 23 configuration."]
    #[inline(always)]
    pub const fn set_cfg23(&mut self, val: super::vals::Cfg23) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Pad output 24 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg24(&self) -> super::vals::Cfg24 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cfg24::from_bits(val as u8)
    }
    #[doc = "Pad output 24 configuration."]
    #[inline(always)]
    pub const fn set_cfg24(&mut self, val: super::vals::Cfg24) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Pad output 25 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg25(&self) -> super::vals::Cfg25 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Cfg25::from_bits(val as u8)
    }
    #[doc = "Pad output 25 configuration."]
    #[inline(always)]
    pub const fn set_cfg25(&mut self, val: super::vals::Cfg25) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Pad output 26 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg26(&self) -> super::vals::Cfg26 {
        let val = (self.0 >> 19usize) & 0x07;
        super::vals::Cfg26::from_bits(val as u8)
    }
    #[doc = "Pad output 26 configuration."]
    #[inline(always)]
    pub const fn set_cfg26(&mut self, val: super::vals::Cfg26) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val.to_bits() as u32) & 0x07) << 19usize);
    }
    #[doc = "Pad output 27 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg27(&self) -> super::vals::Cfg27 {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::Cfg27::from_bits(val as u8)
    }
    #[doc = "Pad output 27 configuration."]
    #[inline(always)]
    pub const fn set_cfg27(&mut self, val: super::vals::Cfg27) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "Pad output 28 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg28(&self) -> super::vals::Cfg28 {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::Cfg28::from_bits(val as u8)
    }
    #[doc = "Pad output 28 configuration."]
    #[inline(always)]
    pub const fn set_cfg28(&mut self, val: super::vals::Cfg28) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
    #[doc = "Pad output 29 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg29(&self) -> super::vals::Cfg29 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Cfg29::from_bits(val as u8)
    }
    #[doc = "Pad output 29 configuration."]
    #[inline(always)]
    pub const fn set_cfg29(&mut self, val: super::vals::Cfg29) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for Outcfg2 {
    #[inline(always)]
    fn default() -> Outcfg2 {
        Outcfg2(0)
    }
}
impl core::fmt::Debug for Outcfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outcfg2")
            .field("cfg20", &self.cfg20())
            .field("cfg21", &self.cfg21())
            .field("cfg22", &self.cfg22())
            .field("cfg23", &self.cfg23())
            .field("cfg24", &self.cfg24())
            .field("cfg25", &self.cfg25())
            .field("cfg26", &self.cfg26())
            .field("cfg27", &self.cfg27())
            .field("cfg28", &self.cfg28())
            .field("cfg29", &self.cfg29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outcfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Outcfg2 {{ cfg20: {:?}, cfg21: {:?}, cfg22: {:?}, cfg23: {:?}, cfg24: {:?}, cfg25: {:?}, cfg26: {:?}, cfg27: {:?}, cfg28: {:?}, cfg29: {:?} }}" , self . cfg20 () , self . cfg21 () , self . cfg22 () , self . cfg23 () , self . cfg24 () , self . cfg25 () , self . cfg26 () , self . cfg27 () , self . cfg28 () , self . cfg29 ())
    }
}
#[doc = "Counter/Timer Output Config 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outcfg3(pub u32);
impl Outcfg3 {
    #[doc = "Pad output 30 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg30(&self) -> super::vals::Cfg30 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cfg30::from_bits(val as u8)
    }
    #[doc = "Pad output 30 configuration."]
    #[inline(always)]
    pub const fn set_cfg30(&mut self, val: super::vals::Cfg30) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Pad output 31 configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg31(&self) -> super::vals::Cfg31 {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Cfg31::from_bits(val as u8)
    }
    #[doc = "Pad output 31 configuration."]
    #[inline(always)]
    pub const fn set_cfg31(&mut self, val: super::vals::Cfg31) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
}
impl Default for Outcfg3 {
    #[inline(always)]
    fn default() -> Outcfg3 {
        Outcfg3(0)
    }
}
impl core::fmt::Debug for Outcfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outcfg3")
            .field("cfg30", &self.cfg30())
            .field("cfg31", &self.cfg31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outcfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outcfg3 {{ cfg30: {:?}, cfg31: {:?} }}",
            self.cfg30(),
            self.cfg31()
        )
    }
}
#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcfg(pub u32);
impl Stcfg {
    #[doc = "Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> super::vals::Clksel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Clksel::from_bits(val as u8)
    }
    #[doc = "Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: super::vals::Clksel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[must_use]
    #[inline(always)]
    pub const fn compare_a_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub const fn set_compare_a_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[must_use]
    #[inline(always)]
    pub const fn compare_b_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub const fn set_compare_b_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[must_use]
    #[inline(always)]
    pub const fn compare_c_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub const fn set_compare_c_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[must_use]
    #[inline(always)]
    pub const fn compare_d_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub const fn set_compare_d_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[must_use]
    #[inline(always)]
    pub const fn compare_e_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub const fn set_compare_e_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[must_use]
    #[inline(always)]
    pub const fn compare_f_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub const fn set_compare_f_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[must_use]
    #[inline(always)]
    pub const fn compare_g_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub const fn set_compare_g_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[must_use]
    #[inline(always)]
    pub const fn compare_h_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub const fn set_compare_h_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[must_use]
    #[inline(always)]
    pub const fn clear(&self) -> super::vals::Clear {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Clear::from_bits(val as u8)
    }
    #[doc = "Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline(always)]
    pub const fn set_clear(&mut self, val: super::vals::Clear) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[must_use]
    #[inline(always)]
    pub const fn freeze(&self) -> super::vals::Freeze {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Freeze::from_bits(val as u8)
    }
    #[doc = "Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline(always)]
    pub const fn set_freeze(&mut self, val: super::vals::Freeze) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Stcfg {
    #[inline(always)]
    fn default() -> Stcfg {
        Stcfg(0)
    }
}
impl core::fmt::Debug for Stcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stcfg")
            .field("clksel", &self.clksel())
            .field("compare_a_en", &self.compare_a_en())
            .field("compare_b_en", &self.compare_b_en())
            .field("compare_c_en", &self.compare_c_en())
            .field("compare_d_en", &self.compare_d_en())
            .field("compare_e_en", &self.compare_e_en())
            .field("compare_f_en", &self.compare_f_en())
            .field("compare_g_en", &self.compare_g_en())
            .field("compare_h_en", &self.compare_h_en())
            .field("clear", &self.clear())
            .field("freeze", &self.freeze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stcfg {{ clksel: {:?}, compare_a_en: {=bool:?}, compare_b_en: {=bool:?}, compare_c_en: {=bool:?}, compare_d_en: {=bool:?}, compare_e_en: {=bool:?}, compare_f_en: {=bool:?}, compare_g_en: {=bool:?}, compare_h_en: {=bool:?}, clear: {:?}, freeze: {:?} }}" , self . clksel () , self . compare_a_en () , self . compare_b_en () , self . compare_c_en () , self . compare_d_en () , self . compare_e_en () , self . compare_f_en () , self . compare_g_en () , self . compare_h_en () , self . clear () , self . freeze ())
    }
}
#[doc = "STIMER Interrupt registers: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmintclr(pub u32);
impl Stmintclr {
    #[doc = "COUNTER is greater than or equal to COMPARE register A."]
    #[must_use]
    #[inline(always)]
    pub const fn comparea(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub const fn set_comparea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register B."]
    #[must_use]
    #[inline(always)]
    pub const fn compareb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub const fn set_compareb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register C."]
    #[must_use]
    #[inline(always)]
    pub const fn comparec(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub const fn set_comparec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register D."]
    #[must_use]
    #[inline(always)]
    pub const fn compared(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub const fn set_compared(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register E."]
    #[must_use]
    #[inline(always)]
    pub const fn comparee(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub const fn set_comparee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register F."]
    #[must_use]
    #[inline(always)]
    pub const fn comparef(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub const fn set_comparef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register G."]
    #[must_use]
    #[inline(always)]
    pub const fn compareg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub const fn set_compareg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register H."]
    #[must_use]
    #[inline(always)]
    pub const fn compareh(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub const fn set_compareh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub const fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPTURE register A has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn capturea(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register A has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_capturea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CAPTURE register B has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn captureb(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register B has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_captureb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CAPTURE register C has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn capturec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register C has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_capturec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CAPTURE register D has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn captured(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register D has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_captured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Stmintclr {
    #[inline(always)]
    fn default() -> Stmintclr {
        Stmintclr(0)
    }
}
impl core::fmt::Debug for Stmintclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stmintclr")
            .field("comparea", &self.comparea())
            .field("compareb", &self.compareb())
            .field("comparec", &self.comparec())
            .field("compared", &self.compared())
            .field("comparee", &self.comparee())
            .field("comparef", &self.comparef())
            .field("compareg", &self.compareg())
            .field("compareh", &self.compareh())
            .field("overflow", &self.overflow())
            .field("capturea", &self.capturea())
            .field("captureb", &self.captureb())
            .field("capturec", &self.capturec())
            .field("captured", &self.captured())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stmintclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stmintclr {{ comparea: {=bool:?}, compareb: {=bool:?}, comparec: {=bool:?}, compared: {=bool:?}, comparee: {=bool:?}, comparef: {=bool:?}, compareg: {=bool:?}, compareh: {=bool:?}, overflow: {=bool:?}, capturea: {=bool:?}, captureb: {=bool:?}, capturec: {=bool:?}, captured: {=bool:?} }}" , self . comparea () , self . compareb () , self . comparec () , self . compared () , self . comparee () , self . comparef () , self . compareg () , self . compareh () , self . overflow () , self . capturea () , self . captureb () , self . capturec () , self . captured ())
    }
}
#[doc = "STIMER Interrupt registers: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stminten(pub u32);
impl Stminten {
    #[doc = "COUNTER is greater than or equal to COMPARE register A."]
    #[must_use]
    #[inline(always)]
    pub const fn comparea(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub const fn set_comparea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register B."]
    #[must_use]
    #[inline(always)]
    pub const fn compareb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub const fn set_compareb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register C."]
    #[must_use]
    #[inline(always)]
    pub const fn comparec(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub const fn set_comparec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register D."]
    #[must_use]
    #[inline(always)]
    pub const fn compared(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub const fn set_compared(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register E."]
    #[must_use]
    #[inline(always)]
    pub const fn comparee(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub const fn set_comparee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register F."]
    #[must_use]
    #[inline(always)]
    pub const fn comparef(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub const fn set_comparef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register G."]
    #[must_use]
    #[inline(always)]
    pub const fn compareg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub const fn set_compareg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register H."]
    #[must_use]
    #[inline(always)]
    pub const fn compareh(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub const fn set_compareh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub const fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPTURE register A has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn capturea(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register A has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_capturea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CAPTURE register B has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn captureb(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register B has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_captureb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CAPTURE register C has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn capturec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register C has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_capturec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CAPTURE register D has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn captured(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register D has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_captured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Stminten {
    #[inline(always)]
    fn default() -> Stminten {
        Stminten(0)
    }
}
impl core::fmt::Debug for Stminten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stminten")
            .field("comparea", &self.comparea())
            .field("compareb", &self.compareb())
            .field("comparec", &self.comparec())
            .field("compared", &self.compared())
            .field("comparee", &self.comparee())
            .field("comparef", &self.comparef())
            .field("compareg", &self.compareg())
            .field("compareh", &self.compareh())
            .field("overflow", &self.overflow())
            .field("capturea", &self.capturea())
            .field("captureb", &self.captureb())
            .field("capturec", &self.capturec())
            .field("captured", &self.captured())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stminten {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stminten {{ comparea: {=bool:?}, compareb: {=bool:?}, comparec: {=bool:?}, compared: {=bool:?}, comparee: {=bool:?}, comparef: {=bool:?}, compareg: {=bool:?}, compareh: {=bool:?}, overflow: {=bool:?}, capturea: {=bool:?}, captureb: {=bool:?}, capturec: {=bool:?}, captured: {=bool:?} }}" , self . comparea () , self . compareb () , self . comparec () , self . compared () , self . comparee () , self . comparef () , self . compareg () , self . compareh () , self . overflow () , self . capturea () , self . captureb () , self . capturec () , self . captured ())
    }
}
#[doc = "STIMER Interrupt registers: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmintset(pub u32);
impl Stmintset {
    #[doc = "COUNTER is greater than or equal to COMPARE register A."]
    #[must_use]
    #[inline(always)]
    pub const fn comparea(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub const fn set_comparea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register B."]
    #[must_use]
    #[inline(always)]
    pub const fn compareb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub const fn set_compareb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register C."]
    #[must_use]
    #[inline(always)]
    pub const fn comparec(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub const fn set_comparec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register D."]
    #[must_use]
    #[inline(always)]
    pub const fn compared(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub const fn set_compared(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register E."]
    #[must_use]
    #[inline(always)]
    pub const fn comparee(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub const fn set_comparee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register F."]
    #[must_use]
    #[inline(always)]
    pub const fn comparef(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub const fn set_comparef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register G."]
    #[must_use]
    #[inline(always)]
    pub const fn compareg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub const fn set_compareg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register H."]
    #[must_use]
    #[inline(always)]
    pub const fn compareh(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub const fn set_compareh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub const fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPTURE register A has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn capturea(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register A has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_capturea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CAPTURE register B has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn captureb(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register B has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_captureb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CAPTURE register C has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn capturec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register C has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_capturec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CAPTURE register D has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn captured(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register D has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_captured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Stmintset {
    #[inline(always)]
    fn default() -> Stmintset {
        Stmintset(0)
    }
}
impl core::fmt::Debug for Stmintset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stmintset")
            .field("comparea", &self.comparea())
            .field("compareb", &self.compareb())
            .field("comparec", &self.comparec())
            .field("compared", &self.compared())
            .field("comparee", &self.comparee())
            .field("comparef", &self.comparef())
            .field("compareg", &self.compareg())
            .field("compareh", &self.compareh())
            .field("overflow", &self.overflow())
            .field("capturea", &self.capturea())
            .field("captureb", &self.captureb())
            .field("capturec", &self.capturec())
            .field("captured", &self.captured())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stmintset {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stmintset {{ comparea: {=bool:?}, compareb: {=bool:?}, comparec: {=bool:?}, compared: {=bool:?}, comparee: {=bool:?}, comparef: {=bool:?}, compareg: {=bool:?}, compareh: {=bool:?}, overflow: {=bool:?}, capturea: {=bool:?}, captureb: {=bool:?}, capturec: {=bool:?}, captured: {=bool:?} }}" , self . comparea () , self . compareb () , self . comparec () , self . compared () , self . comparee () , self . comparef () , self . compareg () , self . compareh () , self . overflow () , self . capturea () , self . captureb () , self . capturec () , self . captured ())
    }
}
#[doc = "STIMER Interrupt registers: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmintstat(pub u32);
impl Stmintstat {
    #[doc = "COUNTER is greater than or equal to COMPARE register A."]
    #[must_use]
    #[inline(always)]
    pub const fn comparea(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub const fn set_comparea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register B."]
    #[must_use]
    #[inline(always)]
    pub const fn compareb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub const fn set_compareb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register C."]
    #[must_use]
    #[inline(always)]
    pub const fn comparec(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub const fn set_comparec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register D."]
    #[must_use]
    #[inline(always)]
    pub const fn compared(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub const fn set_compared(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register E."]
    #[must_use]
    #[inline(always)]
    pub const fn comparee(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub const fn set_comparee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register F."]
    #[must_use]
    #[inline(always)]
    pub const fn comparef(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub const fn set_comparef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register G."]
    #[must_use]
    #[inline(always)]
    pub const fn compareg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub const fn set_compareg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register H."]
    #[must_use]
    #[inline(always)]
    pub const fn compareh(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub const fn set_compareh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub const fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPTURE register A has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn capturea(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register A has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_capturea(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CAPTURE register B has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn captureb(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register B has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_captureb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CAPTURE register C has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn capturec(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register C has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_capturec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CAPTURE register D has grabbed the value in the counter."]
    #[must_use]
    #[inline(always)]
    pub const fn captured(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CAPTURE register D has grabbed the value in the counter."]
    #[inline(always)]
    pub const fn set_captured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Stmintstat {
    #[inline(always)]
    fn default() -> Stmintstat {
        Stmintstat(0)
    }
}
impl core::fmt::Debug for Stmintstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stmintstat")
            .field("comparea", &self.comparea())
            .field("compareb", &self.compareb())
            .field("comparec", &self.comparec())
            .field("compared", &self.compared())
            .field("comparee", &self.comparee())
            .field("comparef", &self.comparef())
            .field("compareg", &self.compareg())
            .field("compareh", &self.compareh())
            .field("overflow", &self.overflow())
            .field("capturea", &self.capturea())
            .field("captureb", &self.captureb())
            .field("capturec", &self.capturec())
            .field("captured", &self.captured())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stmintstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stmintstat {{ comparea: {=bool:?}, compareb: {=bool:?}, comparec: {=bool:?}, compared: {=bool:?}, comparee: {=bool:?}, comparef: {=bool:?}, compareg: {=bool:?}, compareh: {=bool:?}, overflow: {=bool:?}, capturea: {=bool:?}, captureb: {=bool:?}, capturec: {=bool:?}, captured: {=bool:?} }}" , self . comparea () , self . compareb () , self . comparec () , self . compared () , self . comparee () , self . comparef () , self . compareg () , self . compareh () , self . overflow () , self . capturea () , self . captureb () , self . capturec () , self . captured ())
    }
}
#[doc = "Counter/Timer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr0(pub u32);
impl Tmr0 {
    #[doc = "Counter/Timer A0."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmra0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A0."]
    #[inline(always)]
    pub const fn set_cttmra0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B0."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmrb0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B0."]
    #[inline(always)]
    pub const fn set_cttmrb0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tmr0 {
    #[inline(always)]
    fn default() -> Tmr0 {
        Tmr0(0)
    }
}
impl core::fmt::Debug for Tmr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr0")
            .field("cttmra0", &self.cttmra0())
            .field("cttmrb0", &self.cttmrb0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmr0 {{ cttmra0: {=u16:?}, cttmrb0: {=u16:?} }}",
            self.cttmra0(),
            self.cttmrb0()
        )
    }
}
#[doc = "Counter/Timer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr1(pub u32);
impl Tmr1 {
    #[doc = "Counter/Timer A1."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmra1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A1."]
    #[inline(always)]
    pub const fn set_cttmra1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B1."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmrb1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B1."]
    #[inline(always)]
    pub const fn set_cttmrb1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tmr1 {
    #[inline(always)]
    fn default() -> Tmr1 {
        Tmr1(0)
    }
}
impl core::fmt::Debug for Tmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr1")
            .field("cttmra1", &self.cttmra1())
            .field("cttmrb1", &self.cttmrb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmr1 {{ cttmra1: {=u16:?}, cttmrb1: {=u16:?} }}",
            self.cttmra1(),
            self.cttmrb1()
        )
    }
}
#[doc = "Counter/Timer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr2(pub u32);
impl Tmr2 {
    #[doc = "Counter/Timer A2."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmra2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A2."]
    #[inline(always)]
    pub const fn set_cttmra2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B2."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmrb2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B2."]
    #[inline(always)]
    pub const fn set_cttmrb2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tmr2 {
    #[inline(always)]
    fn default() -> Tmr2 {
        Tmr2(0)
    }
}
impl core::fmt::Debug for Tmr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr2")
            .field("cttmra2", &self.cttmra2())
            .field("cttmrb2", &self.cttmrb2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmr2 {{ cttmra2: {=u16:?}, cttmrb2: {=u16:?} }}",
            self.cttmra2(),
            self.cttmrb2()
        )
    }
}
#[doc = "Counter/Timer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr3(pub u32);
impl Tmr3 {
    #[doc = "Counter/Timer A3."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmra3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A3."]
    #[inline(always)]
    pub const fn set_cttmra3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B3."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmrb3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B3."]
    #[inline(always)]
    pub const fn set_cttmrb3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tmr3 {
    #[inline(always)]
    fn default() -> Tmr3 {
        Tmr3(0)
    }
}
impl core::fmt::Debug for Tmr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr3")
            .field("cttmra3", &self.cttmra3())
            .field("cttmrb3", &self.cttmrb3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmr3 {{ cttmra3: {=u16:?}, cttmrb3: {=u16:?} }}",
            self.cttmra3(),
            self.cttmrb3()
        )
    }
}
#[doc = "Counter/Timer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr4(pub u32);
impl Tmr4 {
    #[doc = "Counter/Timer A4."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmra4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A4."]
    #[inline(always)]
    pub const fn set_cttmra4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B4."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmrb4(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B4."]
    #[inline(always)]
    pub const fn set_cttmrb4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tmr4 {
    #[inline(always)]
    fn default() -> Tmr4 {
        Tmr4(0)
    }
}
impl core::fmt::Debug for Tmr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr4")
            .field("cttmra4", &self.cttmra4())
            .field("cttmrb4", &self.cttmrb4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmr4 {{ cttmra4: {=u16:?}, cttmrb4: {=u16:?} }}",
            self.cttmra4(),
            self.cttmrb4()
        )
    }
}
#[doc = "Counter/Timer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr5(pub u32);
impl Tmr5 {
    #[doc = "Counter/Timer A5."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmra5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A5."]
    #[inline(always)]
    pub const fn set_cttmra5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B5."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmrb5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B5."]
    #[inline(always)]
    pub const fn set_cttmrb5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tmr5 {
    #[inline(always)]
    fn default() -> Tmr5 {
        Tmr5(0)
    }
}
impl core::fmt::Debug for Tmr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr5")
            .field("cttmra5", &self.cttmra5())
            .field("cttmrb5", &self.cttmrb5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmr5 {{ cttmra5: {=u16:?}, cttmrb5: {=u16:?} }}",
            self.cttmra5(),
            self.cttmrb5()
        )
    }
}
#[doc = "Counter/Timer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr6(pub u32);
impl Tmr6 {
    #[doc = "Counter/Timer A6."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmra6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A6."]
    #[inline(always)]
    pub const fn set_cttmra6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B6."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmrb6(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B6."]
    #[inline(always)]
    pub const fn set_cttmrb6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tmr6 {
    #[inline(always)]
    fn default() -> Tmr6 {
        Tmr6(0)
    }
}
impl core::fmt::Debug for Tmr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr6")
            .field("cttmra6", &self.cttmra6())
            .field("cttmrb6", &self.cttmrb6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmr6 {{ cttmra6: {=u16:?}, cttmrb6: {=u16:?} }}",
            self.cttmra6(),
            self.cttmrb6()
        )
    }
}
#[doc = "Counter/Timer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr7(pub u32);
impl Tmr7 {
    #[doc = "Counter/Timer A7."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmra7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer A7."]
    #[inline(always)]
    pub const fn set_cttmra7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter/Timer B7."]
    #[must_use]
    #[inline(always)]
    pub const fn cttmrb7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter/Timer B7."]
    #[inline(always)]
    pub const fn set_cttmrb7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tmr7 {
    #[inline(always)]
    fn default() -> Tmr7 {
        Tmr7(0)
    }
}
impl core::fmt::Debug for Tmr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr7")
            .field("cttmra7", &self.cttmra7())
            .field("cttmrb7", &self.cttmrb7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmr7 {{ cttmra7: {=u16:?}, cttmrb7: {=u16:?} }}",
            self.cttmra7(),
            self.cttmrb7()
        )
    }
}
