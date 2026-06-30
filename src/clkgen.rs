#[doc = "Clock Generator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkgen {
    ptr: *mut u8,
}
unsafe impl Send for Clkgen {}
unsafe impl Sync for Clkgen {}
impl Clkgen {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "XT Oscillator Control."]
    #[inline(always)]
    pub const fn calxt(self) -> crate::common::Reg<regs::Calxt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RC Oscillator Control."]
    #[inline(always)]
    pub const fn calrc(self) -> crate::common::Reg<regs::Calrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Autocalibration Counter."]
    #[inline(always)]
    pub const fn acalctr(self) -> crate::common::Reg<regs::Acalctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Oscillator Control."]
    #[inline(always)]
    pub const fn octrl(self) -> crate::common::Reg<regs::Octrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CLKOUT Frequency Select."]
    #[inline(always)]
    pub const fn clkout(self) -> crate::common::Reg<regs::Clkout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Key Register for Clock Control Register."]
    #[inline(always)]
    pub const fn clkkey(self) -> crate::common::Reg<regs::Clkkey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "HFRC Clock Control."]
    #[inline(always)]
    pub const fn cctrl(self) -> crate::common::Reg<regs::Cctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Clock Generator Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "HFRC Adjustment."]
    #[inline(always)]
    pub const fn hfadj(self) -> crate::common::Reg<regs::Hfadj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Clock Enable Status."]
    #[inline(always)]
    pub const fn clockenstat(self) -> crate::common::Reg<regs::Clockenstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Clock Enable Status."]
    #[inline(always)]
    pub const fn clocken2stat(self) -> crate::common::Reg<regs::Clocken2stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Clock Enable Status."]
    #[inline(always)]
    pub const fn clocken3stat(self) -> crate::common::Reg<regs::Clocken3stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "HFRC Frequency Control register."]
    #[inline(always)]
    pub const fn freqctrl(self) -> crate::common::Reg<regs::Freqctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "BLE BUCK TON ADJUST."]
    #[inline(always)]
    pub const fn blebucktonadj(self) -> crate::common::Reg<regs::Blebucktonadj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "CLKGEN Interrupt Register: Enable."]
    #[inline(always)]
    pub const fn intrpten(self) -> crate::common::Reg<regs::Intrpten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "CLKGEN Interrupt Register: Status."]
    #[inline(always)]
    pub const fn intrptstat(self) -> crate::common::Reg<regs::Intrptstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "CLKGEN Interrupt Register: Clear."]
    #[inline(always)]
    pub const fn intrptclr(self) -> crate::common::Reg<regs::Intrptclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "CLKGEN Interrupt Register: Set."]
    #[inline(always)]
    pub const fn intrptset(self) -> crate::common::Reg<regs::Intrptset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
