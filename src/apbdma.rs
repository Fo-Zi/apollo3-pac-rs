#[doc = "APB DMA Register Interfaces."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Apbdma {
    ptr: *mut u8,
}
unsafe impl Send for Apbdma {}
unsafe impl Sync for Apbdma {}
impl Apbdma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn bbvalue(self) -> crate::common::Reg<regs::Bbvalue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Set/Clear Register."]
    #[inline(always)]
    pub const fn bbsetclear(self) -> crate::common::Reg<regs::Bbsetclear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PIO Input Values."]
    #[inline(always)]
    pub const fn bbinput(self) -> crate::common::Reg<regs::Bbinput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PIO Input Values."]
    #[inline(always)]
    pub const fn debugdata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PIO Input Values."]
    #[inline(always)]
    pub const fn debug(self) -> crate::common::Reg<regs::Debug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
}
pub mod regs;
pub mod vals;
