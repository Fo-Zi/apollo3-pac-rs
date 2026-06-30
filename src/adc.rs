#[doc = "Analog Digital Converter Control."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ADC Power Status."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Software trigger."]
    #[inline(always)]
    pub const fn swt(self) -> crate::common::Reg<regs::Swt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Slot 0 Configuration Register."]
    #[inline(always)]
    pub const fn sl0cfg(self) -> crate::common::Reg<regs::Sl0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Slot 1 Configuration Register."]
    #[inline(always)]
    pub const fn sl1cfg(self) -> crate::common::Reg<regs::Sl1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Slot 2 Configuration Register."]
    #[inline(always)]
    pub const fn sl2cfg(self) -> crate::common::Reg<regs::Sl2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Slot 3 Configuration Register."]
    #[inline(always)]
    pub const fn sl3cfg(self) -> crate::common::Reg<regs::Sl3cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Slot 4 Configuration Register."]
    #[inline(always)]
    pub const fn sl4cfg(self) -> crate::common::Reg<regs::Sl4cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Slot 5 Configuration Register."]
    #[inline(always)]
    pub const fn sl5cfg(self) -> crate::common::Reg<regs::Sl5cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Slot 6 Configuration Register."]
    #[inline(always)]
    pub const fn sl6cfg(self) -> crate::common::Reg<regs::Sl6cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Slot 7 Configuration Register."]
    #[inline(always)]
    pub const fn sl7cfg(self) -> crate::common::Reg<regs::Sl7cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Window Comparator Upper Limits Register."]
    #[inline(always)]
    pub const fn wulim(self) -> crate::common::Reg<regs::Wulim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Window Comparator Lower Limits Register."]
    #[inline(always)]
    pub const fn wllim(self) -> crate::common::Reg<regs::Wllim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Scale Window Comparator Limits."]
    #[inline(always)]
    pub const fn scwlim(self) -> crate::common::Reg<regs::Scwlim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "FIFO Data and Valid Count Register."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "FIFO Data and Valid Count Register."]
    #[inline(always)]
    pub const fn fifopr(self) -> crate::common::Reg<regs::Fifopr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "ADC Interrupt registers: Enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "ADC Interrupt registers: Status."]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "ADC Interrupt registers: Clear."]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "ADC Interrupt registers: Set."]
    #[inline(always)]
    pub const fn intset(self) -> crate::common::Reg<regs::Intset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "DMA Trigger Enable Register."]
    #[inline(always)]
    pub const fn dmatrigen(self) -> crate::common::Reg<regs::Dmatrigen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "DMA Trigger Status Register."]
    #[inline(always)]
    pub const fn dmatrigstat(self) -> crate::common::Reg<regs::Dmatrigstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "DMA Configuration Register."]
    #[inline(always)]
    pub const fn dmacfg(self) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "DMA Total Transfer Count."]
    #[inline(always)]
    pub const fn dmatotcount(self) -> crate::common::Reg<regs::Dmatotcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "DMA Target Address Register."]
    #[inline(always)]
    pub const fn dmatargaddr(self) -> crate::common::Reg<regs::Dmatargaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "DMA Status Register."]
    #[inline(always)]
    pub const fn dmastat(self) -> crate::common::Reg<regs::Dmastat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
}
pub mod regs;
pub mod vals;
