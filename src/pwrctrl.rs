#[doc = "PWR Controller Register Bank."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrctrl {
    ptr: *mut u8,
}
unsafe impl Send for Pwrctrl {}
unsafe impl Sync for Pwrctrl {}
impl Pwrctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Voltage Regulator Select Register."]
    #[inline(always)]
    pub const fn supplysrc(self) -> crate::common::Reg<regs::Supplysrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Voltage Regulators status."]
    #[inline(always)]
    pub const fn supplystatus(self) -> crate::common::Reg<regs::Supplystatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Device Power Enables."]
    #[inline(always)]
    pub const fn devpwren(self) -> crate::common::Reg<regs::Devpwren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Powerdown SRAM banks in Deep Sleep mode."]
    #[inline(always)]
    pub const fn mempwdinsleep(self) -> crate::common::Reg<regs::Mempwdinsleep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Enables individual banks of the MEMORY array."]
    #[inline(always)]
    pub const fn mempwren(self) -> crate::common::Reg<regs::Mempwren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Mem Power ON Status."]
    #[inline(always)]
    pub const fn mempwrstatus(self) -> crate::common::Reg<regs::Mempwrstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Device Power ON Status."]
    #[inline(always)]
    pub const fn devpwrstatus(self) -> crate::common::Reg<regs::Devpwrstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SRAM Control register."]
    #[inline(always)]
    pub const fn sramctrl(self) -> crate::common::Reg<regs::Sramctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Power Status Register for ADC Block."]
    #[inline(always)]
    pub const fn adcstatus(self) -> crate::common::Reg<regs::Adcstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Power Optimization Control Bits."]
    #[inline(always)]
    pub const fn misc(self) -> crate::common::Reg<regs::Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
    #[inline(always)]
    pub const fn devpwreventen(self) -> crate::common::Reg<regs::Devpwreventen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
    #[inline(always)]
    pub const fn mempwreventen(self) -> crate::common::Reg<regs::Mempwreventen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
