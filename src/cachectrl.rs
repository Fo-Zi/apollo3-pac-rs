#[doc = "Flash Cache Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cachectrl {
    ptr: *mut u8,
}
unsafe impl Send for Cachectrl {}
unsafe impl Sync for Cachectrl {}
impl Cachectrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash Cache Control Register."]
    #[inline(always)]
    pub const fn cachecfg(self) -> crate::common::Reg<regs::Cachecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Flash Control Register."]
    #[inline(always)]
    pub const fn flashcfg(self) -> crate::common::Reg<regs::Flashcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Cache Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Flash Cache Noncachable Region 0 Start."]
    #[inline(always)]
    pub const fn ncr0start(self) -> crate::common::Reg<regs::Ncr0start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Flash Cache Noncachable Region 0 End."]
    #[inline(always)]
    pub const fn ncr0end(self) -> crate::common::Reg<regs::Ncr0end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Flash Cache Noncachable Region 1 Start."]
    #[inline(always)]
    pub const fn ncr1start(self) -> crate::common::Reg<regs::Ncr1start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Flash Cache Noncachable Region 1 End."]
    #[inline(always)]
    pub const fn ncr1end(self) -> crate::common::Reg<regs::Ncr1end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Data Cache Total Accesses."]
    #[inline(always)]
    pub const fn dmon0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Data Cache Tag Lookups."]
    #[inline(always)]
    pub const fn dmon1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Data Cache Hits."]
    #[inline(always)]
    pub const fn dmon2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Data Cache Line Hits."]
    #[inline(always)]
    pub const fn dmon3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Instruction Cache Total Accesses."]
    #[inline(always)]
    pub const fn imon0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Instruction Cache Tag Lookups."]
    #[inline(always)]
    pub const fn imon1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Instruction Cache Hits."]
    #[inline(always)]
    pub const fn imon2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Instruction Cache Line Hits."]
    #[inline(always)]
    pub const fn imon3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
