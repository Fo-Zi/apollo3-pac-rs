#[doc = "Security Interfaces."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Security {
    ptr: *mut u8,
}
unsafe impl Send for Security {}
unsafe impl Sync for Security {}
impl Security {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Source Addresss."]
    #[inline(always)]
    pub const fn srcaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Length."]
    #[inline(always)]
    pub const fn len(self) -> crate::common::Reg<regs::Len, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CRC Seed/Result Register."]
    #[inline(always)]
    pub const fn result(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "LOCK Control Register."]
    #[inline(always)]
    pub const fn lockctrl(self) -> crate::common::Reg<regs::Lockctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "LOCK Status Register."]
    #[inline(always)]
    pub const fn lockstat(self) -> crate::common::Reg<regs::Lockstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Key0 Register."]
    #[inline(always)]
    pub const fn key0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Key1 Register."]
    #[inline(always)]
    pub const fn key1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Key2 Register."]
    #[inline(always)]
    pub const fn key2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Key3 Register."]
    #[inline(always)]
    pub const fn key3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
