#[doc = "RTC Alarms Lower."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Almlow(pub u32);
impl Almlow {
    #[doc = "100ths of a second Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn alm100(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "100ths of a second Alarm."]
    #[inline(always)]
    pub const fn set_alm100(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Seconds Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn almsec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Seconds Alarm."]
    #[inline(always)]
    pub const fn set_almsec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Minutes Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn almmin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Minutes Alarm."]
    #[inline(always)]
    pub const fn set_almmin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Hours Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn almhr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Hours Alarm."]
    #[inline(always)]
    pub const fn set_almhr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Almlow {
    #[inline(always)]
    fn default() -> Almlow {
        Almlow(0)
    }
}
impl core::fmt::Debug for Almlow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Almlow")
            .field("alm100", &self.alm100())
            .field("almsec", &self.almsec())
            .field("almmin", &self.almmin())
            .field("almhr", &self.almhr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Almlow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Almlow {{ alm100: {=u8:?}, almsec: {=u8:?}, almmin: {=u8:?}, almhr: {=u8:?} }}",
            self.alm100(),
            self.almsec(),
            self.almmin(),
            self.almhr()
        )
    }
}
#[doc = "RTC Alarms Upper."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Almup(pub u32);
impl Almup {
    #[doc = "Date Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn almdate(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Date Alarm."]
    #[inline(always)]
    pub const fn set_almdate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Months Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn almmo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Months Alarm."]
    #[inline(always)]
    pub const fn set_almmo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Weekdays Alarm."]
    #[must_use]
    #[inline(always)]
    pub const fn almwkdy(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Weekdays Alarm."]
    #[inline(always)]
    pub const fn set_almwkdy(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Almup {
    #[inline(always)]
    fn default() -> Almup {
        Almup(0)
    }
}
impl core::fmt::Debug for Almup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Almup")
            .field("almdate", &self.almdate())
            .field("almmo", &self.almmo())
            .field("almwkdy", &self.almwkdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Almup {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Almup {{ almdate: {=u8:?}, almmo: {=u8:?}, almwkdy: {=u8:?} }}",
            self.almdate(),
            self.almmo(),
            self.almwkdy()
        )
    }
}
#[doc = "RTC Counters Lower."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrlow(pub u32);
impl Ctrlow {
    #[doc = "100ths of a second Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctr100(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "100ths of a second Counter."]
    #[inline(always)]
    pub const fn set_ctr100(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Seconds Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrsec(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Seconds Counter."]
    #[inline(always)]
    pub const fn set_ctrsec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Minutes Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrmin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Minutes Counter."]
    #[inline(always)]
    pub const fn set_ctrmin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Hours Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrhr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Hours Counter."]
    #[inline(always)]
    pub const fn set_ctrhr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Ctrlow {
    #[inline(always)]
    fn default() -> Ctrlow {
        Ctrlow(0)
    }
}
impl core::fmt::Debug for Ctrlow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrlow")
            .field("ctr100", &self.ctr100())
            .field("ctrsec", &self.ctrsec())
            .field("ctrmin", &self.ctrmin())
            .field("ctrhr", &self.ctrhr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrlow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrlow {{ ctr100: {=u8:?}, ctrsec: {=u8:?}, ctrmin: {=u8:?}, ctrhr: {=u8:?} }}",
            self.ctr100(),
            self.ctrsec(),
            self.ctrmin(),
            self.ctrhr()
        )
    }
}
#[doc = "RTC Counters Upper."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrup(pub u32);
impl Ctrup {
    #[doc = "Date Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrdate(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Date Counter."]
    #[inline(always)]
    pub const fn set_ctrdate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Months Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrmo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Months Counter."]
    #[inline(always)]
    pub const fn set_ctrmo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Years Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctryr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Years Counter."]
    #[inline(always)]
    pub const fn set_ctryr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Weekdays Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrwkdy(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Weekdays Counter."]
    #[inline(always)]
    pub const fn set_ctrwkdy(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Century."]
    #[must_use]
    #[inline(always)]
    pub const fn cb(&self) -> super::vals::Cb {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Cb::from_bits(val as u8)
    }
    #[doc = "Century."]
    #[inline(always)]
    pub const fn set_cb(&mut self, val: super::vals::Cb) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Century enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ceb(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Century enable."]
    #[inline(always)]
    pub const fn set_ceb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
    #[must_use]
    #[inline(always)]
    pub const fn cterr(&self) -> super::vals::Cterr {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cterr::from_bits(val as u8)
    }
    #[doc = "Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
    #[inline(always)]
    pub const fn set_cterr(&mut self, val: super::vals::Cterr) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrup {
    #[inline(always)]
    fn default() -> Ctrup {
        Ctrup(0)
    }
}
impl core::fmt::Debug for Ctrup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrup")
            .field("ctrdate", &self.ctrdate())
            .field("ctrmo", &self.ctrmo())
            .field("ctryr", &self.ctryr())
            .field("ctrwkdy", &self.ctrwkdy())
            .field("cb", &self.cb())
            .field("ceb", &self.ceb())
            .field("cterr", &self.cterr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrup {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrup {{ ctrdate: {=u8:?}, ctrmo: {=u8:?}, ctryr: {=u8:?}, ctrwkdy: {=u8:?}, cb: {:?}, ceb: {=bool:?}, cterr: {:?} }}" , self . ctrdate () , self . ctrmo () , self . ctryr () , self . ctrwkdy () , self . cb () , self . ceb () , self . cterr ())
    }
}
#[doc = "RTC Interrupt Register: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "RTC Alarm interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn alm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Alarm interrupt."]
    #[inline(always)]
    pub const fn set_alm(&mut self, val: bool) {
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
        f.debug_struct("Intclr").field("alm", &self.alm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intclr {{ alm: {=bool:?} }}", self.alm())
    }
}
#[doc = "RTC Interrupt Register: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "RTC Alarm interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn alm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Alarm interrupt."]
    #[inline(always)]
    pub const fn set_alm(&mut self, val: bool) {
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
        f.debug_struct("Inten").field("alm", &self.alm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Inten {{ alm: {=bool:?} }}", self.alm())
    }
}
#[doc = "RTC Interrupt Register: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "RTC Alarm interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn alm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Alarm interrupt."]
    #[inline(always)]
    pub const fn set_alm(&mut self, val: bool) {
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
        f.debug_struct("Intset").field("alm", &self.alm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intset {{ alm: {=bool:?} }}", self.alm())
    }
}
#[doc = "RTC Interrupt Register: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "RTC Alarm interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn alm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RTC Alarm interrupt."]
    #[inline(always)]
    pub const fn set_alm(&mut self, val: bool) {
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
        f.debug_struct("Intstat").field("alm", &self.alm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intstat {{ alm: {=bool:?} }}", self.alm())
    }
}
#[doc = "RTC Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtcctl(pub u32);
impl Rtcctl {
    #[doc = "Counter write control."]
    #[must_use]
    #[inline(always)]
    pub const fn wrtc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter write control."]
    #[inline(always)]
    pub const fn set_wrtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Alarm repeat interval."]
    #[must_use]
    #[inline(always)]
    pub const fn rpt(&self) -> super::vals::Rpt {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Rpt::from_bits(val as u8)
    }
    #[doc = "Alarm repeat interval."]
    #[inline(always)]
    pub const fn set_rpt(&mut self, val: super::vals::Rpt) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "RTC input clock control."]
    #[must_use]
    #[inline(always)]
    pub const fn rstop(&self) -> super::vals::Rstop {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Rstop::from_bits(val as u8)
    }
    #[doc = "RTC input clock control."]
    #[inline(always)]
    pub const fn set_rstop(&mut self, val: super::vals::Rstop) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Hours Counter mode."]
    #[must_use]
    #[inline(always)]
    pub const fn hr1224(&self) -> super::vals::Hr1224 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hr1224::from_bits(val as u8)
    }
    #[doc = "Hours Counter mode."]
    #[inline(always)]
    pub const fn set_hr1224(&mut self, val: super::vals::Hr1224) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Rtcctl {
    #[inline(always)]
    fn default() -> Rtcctl {
        Rtcctl(0)
    }
}
impl core::fmt::Debug for Rtcctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rtcctl")
            .field("wrtc", &self.wrtc())
            .field("rpt", &self.rpt())
            .field("rstop", &self.rstop())
            .field("hr1224", &self.hr1224())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rtcctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rtcctl {{ wrtc: {=bool:?}, rpt: {:?}, rstop: {:?}, hr1224: {:?} }}",
            self.wrtc(),
            self.rpt(),
            self.rstop(),
            self.hr1224()
        )
    }
}
