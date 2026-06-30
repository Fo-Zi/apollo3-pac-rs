#[doc = "Serial UART."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart0 {
    ptr: *mut u8,
}
unsafe impl Send for Uart0 {}
unsafe impl Sync for Uart0 {}
impl Uart0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "UART Data Register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "UART Status Register."]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Flag Register."]
    #[inline(always)]
    pub const fn fr(self) -> crate::common::Reg<regs::Fr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "IrDA Counter."]
    #[inline(always)]
    pub const fn ilpr(self) -> crate::common::Reg<regs::Ilpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Integer Baud Rate Divisor."]
    #[inline(always)]
    pub const fn ibrd(self) -> crate::common::Reg<regs::Ibrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Fractional Baud Rate Divisor."]
    #[inline(always)]
    pub const fn fbrd(self) -> crate::common::Reg<regs::Fbrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Line Control High."]
    #[inline(always)]
    pub const fn lcrh(self) -> crate::common::Reg<regs::Lcrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "FIFO Interrupt Level Select."]
    #[inline(always)]
    pub const fn ifls(self) -> crate::common::Reg<regs::Ifls, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn ies(self) -> crate::common::Reg<regs::Ies, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Masked Interrupt Status."]
    #[inline(always)]
    pub const fn mis(self) -> crate::common::Reg<regs::Mis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Clear."]
    #[inline(always)]
    pub const fn iec(self) -> crate::common::Reg<regs::Iec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
}
pub mod regs;
pub mod vals;
