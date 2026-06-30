#[doc = "Real Time Clock."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC Counters Lower."]
    #[inline(always)]
    pub const fn ctrlow(self) -> crate::common::Reg<regs::Ctrlow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "RTC Counters Upper."]
    #[inline(always)]
    pub const fn ctrup(self) -> crate::common::Reg<regs::Ctrup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "RTC Alarms Lower."]
    #[inline(always)]
    pub const fn almlow(self) -> crate::common::Reg<regs::Almlow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "RTC Alarms Upper."]
    #[inline(always)]
    pub const fn almup(self) -> crate::common::Reg<regs::Almup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "RTC Control Register."]
    #[inline(always)]
    pub const fn rtcctl(self) -> crate::common::Reg<regs::Rtcctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RTC Interrupt Register: Enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "RTC Interrupt Register: Status."]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "RTC Interrupt Register: Clear."]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "RTC Interrupt Register: Set."]
    #[inline(always)]
    pub const fn intset(self) -> crate::common::Reg<regs::Intset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
