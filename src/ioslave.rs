#[doc = "I2C/SPI Slave."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ioslave {
    ptr: *mut u8,
}
unsafe impl Send for Ioslave {}
unsafe impl Sync for Ioslave {}
impl Ioslave {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Current FIFO Pointer."]
    #[inline(always)]
    pub const fn fifoptr(self) -> crate::common::Reg<regs::Fifoptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "FIFO Configuration."]
    #[inline(always)]
    pub const fn fifocfg(self) -> crate::common::Reg<regs::Fifocfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "FIFO Threshold Configuration."]
    #[inline(always)]
    pub const fn fifothr(self) -> crate::common::Reg<regs::Fifothr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "FIFO Update Status."]
    #[inline(always)]
    pub const fn fupd(self) -> crate::common::Reg<regs::Fupd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Overall FIFO Counter."]
    #[inline(always)]
    pub const fn fifoctr(self) -> crate::common::Reg<regs::Fifoctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Overall FIFO Counter Increment."]
    #[inline(always)]
    pub const fn fifoinc(self) -> crate::common::Reg<regs::Fifoinc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "I/O Slave Configuration."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "I/O Slave Interrupt Priority Encode."]
    #[inline(always)]
    pub const fn prenc(self) -> crate::common::Reg<regs::Prenc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "I/O Interrupt Control."]
    #[inline(always)]
    pub const fn iointctl(self) -> crate::common::Reg<regs::Iointctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "General Address Data."]
    #[inline(always)]
    pub const fn genadd(self) -> crate::common::Reg<regs::Genadd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "IO Slave Interrupts: Enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "IO Slave Interrupts: Status."]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "IO Slave Interrupts: Clear."]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "IO Slave Interrupts: Set."]
    #[inline(always)]
    pub const fn intset(self) -> crate::common::Reg<regs::Intset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Register Access Interrupts: Enable."]
    #[inline(always)]
    pub const fn regaccinten(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Register Access Interrupts: Status."]
    #[inline(always)]
    pub const fn regaccintstat(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Register Access Interrupts: Clear."]
    #[inline(always)]
    pub const fn regaccintclr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Register Access Interrupts: Set."]
    #[inline(always)]
    pub const fn regaccintset(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
